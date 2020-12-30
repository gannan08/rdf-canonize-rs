extern crate rdf_canonize;

use std::env;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // println!("Filename {}", filename);

    let mut line_counter = 0;
    if let Ok(lines) = read_lines(filename) {
        let mut t = String::with_capacity(2050);
        for line in lines {
            if let Ok(l) = line {
                line_counter += 1;
                t.push_str(&l);
                t.push('\n');
                if line_counter % 20 == 0 {
                    let q2 = rdf_canonize::nquads::parse_nquads(&t);
                    let serialized_nquads = rdf_canonize::canonize(&q2, "URDNA2015").unwrap();
                    print!("{}", serialized_nquads);
                    t = String::with_capacity(2050);
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
