use std::collections::HashSet;
use crate::utils;

#[derive(Debug)]
struct Compartments {
    first: Vec<u8>,
    second: Vec<u8>,
}

impl Compartments {
    fn find_mistake(&self) -> Vec<i32>{
        let first_set: HashSet<&u8> = HashSet::from_iter(self.first.iter());
        let second_set: HashSet<&u8> = HashSet::from_iter(self.second.iter());
        let in_both: HashSet<&u8> = first_set.intersection(&second_set)
            .copied()
            .collect();

        return in_both.into_iter()
            .map(|x| to_priories(x)).collect();
    }
}

fn to_priories(c: &u8) -> i32 {
    return match c {
        97..=122 => (c - 96) as i32,
        65..=90 => (c - 38) as i32,
        _ => {
            println!("unexpected value {}", c);
            0
        }
    }
}

fn split_to_compartments(s: &str) -> Compartments {
    let chars = s.as_bytes().to_vec();
    let mid = chars.len() / 2;
    let first_half = chars[0..mid].to_vec();
    let second_half = chars[mid..s.len()].to_vec();
    return Compartments {
        first: first_half,
        second: second_half,
    }
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/trial.txt");
    let comps: Vec<Compartments> = raw.iter()
        .map(|x| split_to_compartments(x))
        .collect();

    for x in comps {
        println!("{:?}", x)
    }


}
