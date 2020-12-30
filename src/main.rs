extern crate rdf_canonize;

use std::env;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rdf_canonize::nquads::{Dataset};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // println!("Filename {}", filename);

    let mut line_counter = 0;
    let mut rdf_dataset = Dataset::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                line_counter += 1;
                let q2 = rdf_canonize::nquads::parse_nquad(&l);
                rdf_dataset.add(q2);
                if line_counter % 20 == 0 {
                    let serialized_nquads = rdf_canonize::canonize(&rdf_dataset, "URDNA2015").unwrap();
                    print!("{}", serialized_nquads);
                    rdf_dataset = Dataset::new();
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
