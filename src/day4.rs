use std::collections::HashSet;
use crate::utils;


#[derive(Debug)]
struct Pair<> {
    first: Vec<i32>,
    second: Vec<i32>,
}

impl Pair {
    fn included(&self) -> bool {
        let s1: HashSet<&i32> = HashSet::from_iter(self.first.iter());
        let s2: HashSet<&i32> = HashSet::from_iter(self.second.iter());

        return s1.is_subset(&s2) || s2.is_subset(&s1);
    }

    fn overlap(&self) -> bool {
        let s1: HashSet<&i32> = HashSet::from_iter(self.first.iter());
        let s2: HashSet<&i32> = HashSet::from_iter(self.second.iter());

        return !s1.is_disjoint(&s2);
    }
}

fn new_pair(s: String) -> Pair {
    let raw_pair: Vec<&str> = s.split(",").collect();

    let r1 = get_range(raw_pair.get(0)
        .unwrap()
        .clone());
    let r2 = get_range(raw_pair.get(1)
        .unwrap()
        .clone());

    return Pair {
        first: r1.clone(),
        second: r2.clone(),
    };
}

fn get_range(s: &str) -> Vec<i32> {
    let ends: Vec<&str> = s.split("-").collect();
    let start_str = ends.get(0).unwrap().clone();
    let start: i32 = start_str.parse().unwrap();

    let end_str = ends.get(1).unwrap().clone();
    let end: i32 = end_str.parse().unwrap();

    return (start..end + 1).collect();
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day4.txt");

    let total = raw.into_iter()
        .map(|x| new_pair(x))
        .filter(|x| x.included())
        .count();

    println!("{}", total)
}

pub fn part2() {
    let raw = utils::read_lines("./inputs/day4.txt");

    let total = raw.into_iter()
        .map(|x| new_pair(x))
        .filter(|x| x.overlap())
        .count();

    println!("{}", total)
}
