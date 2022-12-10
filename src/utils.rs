use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

pub fn byte_to_letter(c: u8) -> String {
    let letter = std::str::from_utf8(&vec![c]).unwrap().to_string();
    return letter;
}