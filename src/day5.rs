use std::ops::Add;
use crate::utils;
use std::str;

#[derive(Debug)]
struct Stacks {
    towers: Vec<Vec<String>>
}

fn new_stacks(towers: Vec<Vec<String>>) -> Stacks {
    return Stacks { towers }
}

fn byte_to_letter(c: u8) -> String {
    let letter = str::from_utf8(&vec![c]).unwrap().to_string();
    return letter
}

fn parse_tower_by_number(top_raw: &Vec<String>, number: i32) -> Vec<String> {
    let first_tower_index = 1;
    let increment = 4;
    let tower_index= (increment * (number - 1) + first_tower_index) as usize;

    let without_first = &top_raw[1..];

    let stack: Vec<String> = without_first.into_iter()
        .map(|x| right_pad(x.clone(), (tower_index +1) as i32))
        .map(|x| x.as_bytes()[tower_index])
        .map(|x| byte_to_letter(x))
        .take_while(|x| !x.trim().is_empty())
        .collect();

    return stack;
}

fn count_towers(top_rows: &Vec<String>) -> i32 {
    let tower_numbers = top_rows.first()
        .unwrap()
        .trim();
    let last_byte = tower_numbers.as_bytes()
        .get(tower_numbers.len() - 1)
        .unwrap();
    let last_tower_number :i32 = byte_to_letter(last_byte.clone())
        .parse()
        .unwrap();

    return last_tower_number;
}

fn right_pad(s: String, required_len: i32) -> String {
    let current_len = s.len();
    let required_spaces = required_len  - current_len as i32;
    if required_spaces < 0 {
        return s
    }
    let spaces = " ".repeat(required_spaces as usize);
    return s.add(spaces.as_str());
}

fn parse_top(raw: Vec<String>) -> Stacks {
    let mut top_rows: Vec<String> = raw.into_iter().take_while(|x| x.len() > 1)
        .collect();
    top_rows.reverse();

    let number_of_towers = count_towers(&top_rows);
    let towers: Vec<Vec<String>> = (1..number_of_towers + 1).into_iter()
        .map(|x| parse_tower_by_number(&top_rows, x))
        .collect();

    return new_stacks(towers)
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day5.txt");

    let stacks = parse_top(raw);

    println!("{:?}", stacks)

}