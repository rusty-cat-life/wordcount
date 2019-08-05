mod lib;

use std::env;
use std::fs::File;
use std::io::BufReader;

use lib::count;

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("First Argument FILENAME required.");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
