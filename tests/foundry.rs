extern crate horned_functional;

use std::io::BufRead;
use std::io::BufReader;

// use fastobo_graphs::GraphDocument;
//
// lazy_static::lazy_static! {
//     /// The latest OBO Foundry listing.
//     static ref FOUNDRY: obofoundry::Foundry = {
//         let response = ureq::get("http://www.obofoundry.org/registry/ontologies.yml")
//             .call();
//         serde_yaml::from_reader(response.into_reader())
//             .expect("could not read the OBO Foundry listing")
//     };
// }

macro_rules! foundrytest {
    ( $(#[$attr:meta])* $name:ident) => (
        #[test]
        $(#[$attr])*
        fn $name() {
            let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path = std::path::PathBuf::from(dir)
                .join("data")
                .join(stringify!($name))
                .with_extension("obo.ofn");
            let txt = std::fs::read_to_string(&path).unwrap();

            if let Err(e) = horned_functional::parse(&txt) {
                panic!("could not parse {}: {}", stringify!($name), e);
            }
        }
    );
}

foundrytest!(aero);
foundrytest!(apo);
foundrytest!(cio);
foundrytest!(doid);
foundrytest!(hao);
foundrytest!(hp);
foundrytest!(mp);
foundrytest!(ms);
foundrytest!(peco);
foundrytest!(plana);
foundrytest!(symp);
foundrytest!(to);

// Failing because of invalid IRIs created by `owltools` conversion.
foundrytest!(#[ignore] ecocore);
foundrytest!(#[ignore] cl);
foundrytest!(#[ignore] ro);

// Too large to load in memory
//
// foundrytest!(oba);
// foundrytest!(tto);
// foundrytest!(uberon);
// foundrytest!(vto);
// foundrytest!(mondo);
// foundrytest!(gaz);
// foundrytest!(go);
// foundrytest!(chebi);