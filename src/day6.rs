use std::collections::{HashSet, VecDeque};
use crate::utils;


fn find_first_unique_set(raw: String, size: i32) -> i32 {
    let mut last_four: VecDeque<char> = VecDeque::new();
    let mut index = 0;
    for x in raw.chars() {
        index += 1;
        if last_four.len() >= size as usize {
            last_four.pop_front();
        }
        last_four.push_back(x);

        let set: HashSet<&char> = HashSet::from_iter(last_four.iter());

        if set.len() == size as usize {
            return index;
        }
    }
    return -1
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day6.txt")
        .first()
        .unwrap()
        .clone();

    let marker = find_first_unique_set(raw, 4);
    println!("{}", marker)
}

pub fn part2() {
    let raw = utils::read_lines("./inputs/day6.txt")
        .first()
        .unwrap()
        .clone();

    let message = find_first_unique_set(raw, 14);
    println!("{}", message)
}