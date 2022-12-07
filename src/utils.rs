use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("no such file");
    let buff = BufReader::new(file);
    buff.lines()
        .map(|x| x.expect("unable to parse line"))
        .collect()
}

pub fn read_to_string(path: &str) -> String {
    fs::read_to_string(path).unwrap().parse().unwrap()
}