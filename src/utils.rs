use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("no such file");
    let buff = BufReader::new(file);
    buff.lines()
        .map(|x| x.expect("unable to parse line"))
        .collect()
}