extern crate rdf_canonize;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Filename {}", filename);

    let dataset_str = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("INPUT:");
    println!("{}", dataset_str);
    let rdf_dataset = rdf_canonize::nquads::parse_nquads(&dataset_str);

    println!();
    let serialized_nquads = rdf_canonize::canonize(&rdf_dataset, "URDNA2015").unwrap();
    println!("OUTPUT:");
    println!("{}", serialized_nquads);
}
