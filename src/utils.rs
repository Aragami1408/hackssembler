use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);

    let vec: Vec<String> = f.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    Ok(vec)
}
