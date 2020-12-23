#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;

mod identifier_issuer;
mod nquads;
mod permuter;
mod urdna2015;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Filename {}", filename);

    let dataset = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let rdf_dataset = nquads::parse_nquads(&dataset);

    urdna2015::URDNA2015::new().main(&rdf_dataset);
}
