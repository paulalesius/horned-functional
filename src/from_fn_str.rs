use curie::PrefixMapping;
use horned_owl::model::Build;

use crate::error::Error;
use crate::error::Result;
use crate::from_pair::FromPair;
use crate::parser::OwlFunctionalParser;

/// A trait for OWL elements that can be deserialized from OWL strings.
///
/// The deserialization will fail if the entirety of the input string cannot
/// be deserialized into the declared type.
pub trait FromFunctional: Sized + FromPair {
    /// Deserialize a string containing an OWL element in functional syntax.
    fn from_ofn(s: &str, build: &Build, prefixes: &PrefixMapping) -> Result<Self>;

    #[inline]
    fn from_ofn_with_build(s: &str, build: &Build) -> Result<Self> {
        Self::from_ofn(s, build, &PrefixMapping::default())
    }

    #[inline]
    fn from_ofn_with_prefixes(s: &str, prefixes: &PrefixMapping) -> Result<Self> {
        Self::from_ofn(s, &Build::default(), prefixes)
    }

    #[inline]
    fn from_ofn_str(s: &str) -> Result<Self> {
        Self::from_ofn(s, &Build::default(), &PrefixMapping::default())
    }
}


impl<T: Sized + FromPair> FromFunctional for T {
    fn from_ofn(s: &str, build: &Build, prefixes: &PrefixMapping) -> Result<Self> {
        for rule in T::RULES {
            if let Ok(mut pairs) = OwlFunctionalParser::parse(*rule, s) {
                if pairs.as_str().len() == s.len() {
                    return T::from_pair(pairs.next().unwrap(), build, prefixes);
                } else {
                    return Err(
                        Error::from(
                            pest::error::Error::new_from_span(
                                pest::error::ErrorVariant::CustomError {
                                    message: "remaining input".to_string(),
                                },
                                pest::Span::new(s, pairs.as_str().len(), s.len()).unwrap()
                            )
                        )
                    )
                }
            }
        }

        return Err(
            Error::from(
                pest::error::Error::new_from_span(
                    pest::error::ErrorVariant::ParsingError {
                        positives: Vec::new(),
                        negatives: Self::RULES.iter().cloned().collect(),
                    },
                    pest::Span::new(s, 0, s.len()).unwrap()
                )
            )
        );
    }
}


#[cfg(test)]
mod tests {

    use horned_owl::model::DeclareClass;
    use super::*;

    #[test]
    fn test_remaining_input() {
        match DeclareClass::from_ofn_str("Class(<http://example.com/a>) Class(<http://example.com/b>)") {
            Ok(ok) => panic!("unexpected success: {:?}", ok),
            Err(Error::PestError(e)) => {
                assert_eq!(e.variant, pest::error::ErrorVariant::CustomError {
                    message: "remaining input".to_string(),
                })
            }
            Err(other) => panic!("unexpected error: {:?}", other)
        }
    }
}
