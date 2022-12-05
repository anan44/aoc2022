use crate::utils;
use std::str;


struct Stacks {
    towers: Vec<Vec<String>>
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

fn parse_top(raw: Vec<String>) {
    let mut top_rows: Vec<String> = raw.into_iter().take_while(|x| x.len() > 1)
        .collect();

    top_rows.reverse();

    let number_of_towers = count_towers(&top_rows);

    println!("{}", number_of_towers);
    let towers: Vec<Vec<String>> = (1..number_of_towers + 1).into_iter()
        .map(|x| parse_tower_by_number(&top_rows, x))
        .collect();


    for x in towers {
        println!("{:?}", x)
    }
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day5.txt");


    parse_top(raw)

}