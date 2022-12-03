use std::collections::HashSet;
use crate::utils;

#[derive(Debug)]
struct Compartments {
    first: Vec<u8>,
    second: Vec<u8>,
}

impl Compartments {
    fn find_mistake(&self) -> i32 {
        let first_set: HashSet<&u8> = HashSet::from_iter(self.first.iter());
        let second_set: HashSet<&u8> = HashSet::from_iter(self.second.iter());
        let in_both: HashSet<&u8> = first_set.intersection(&second_set)
            .copied()
            .collect();

        let vec: Vec<i32> = in_both.into_iter()
            .map(|x| to_priorities(x))
            .collect();

        return vec.get(0)
            .unwrap()
            .clone();
    }
}

fn to_priorities(c: &u8) -> i32 {
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
    let raw = utils::read_lines("./inputs/day3.txt");
    let comps: Vec<Compartments> = raw.iter()
        .map(|x| split_to_compartments(x))
        .collect();

    let mistakes: Vec<i32> = comps.iter()
        .map(|x| x.find_mistake())
        .collect();

    let sum: i32 = mistakes.iter().sum();

    println!("{}", sum)


}
