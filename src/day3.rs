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
    };
}

fn split_to_compartments(s: &str) -> Compartments {
    let chars = s.as_bytes().to_vec();
    let mid = chars.len() / 2;
    let first_half = chars[0..mid].to_vec();
    let second_half = chars[mid..s.len()].to_vec();
    return Compartments {
        first: first_half,
        second: second_half,
    };
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

struct Group {
    first: String,
    second: String,
    third: String,
}

impl Group {
    fn find_badge(&self) -> i32 {
        let p1 = bag_to_priorities(self.first.clone());
        let s1: HashSet<&i32> = HashSet::from_iter(p1.iter());

        let p2 = bag_to_priorities(self.second.clone());
        let s2: HashSet<&i32> = HashSet::from_iter(p2.iter());

        let p3 = bag_to_priorities(self.third.clone());
        let s3: HashSet<&i32> = HashSet::from_iter(p3.iter());

        let first_inter: HashSet<_>= s1.intersection(&s2)
            .copied()
            .collect();

        let second_inter = s3.intersection(&first_inter);

        let badge_vec: Vec<i32> = second_inter.copied().copied().collect();
        let badge =  badge_vec.first().unwrap().clone();
        return badge
    }
}

fn new_group(s: &[String]) -> Group {
    return Group {
        first: s.get(0).unwrap().clone(),
        second: s.get(1).unwrap().clone(),
        third: s.get(2).unwrap().clone(),
    };
}

fn bag_to_priorities(s: String) -> Vec<i32> {
    return s.bytes()
        .map(|x| to_priorities(&x))
        .collect();
}


pub fn part2() {
    let raw = utils::read_lines("./inputs/day3.txt");
    let groups: Vec<Group> = raw.chunks(3)
        .map(|x| new_group(x))
        .collect();
    let sum: i32 = groups.iter()
        .map(|x| x.find_badge())
        .sum();

    println!("{}", sum)
}
