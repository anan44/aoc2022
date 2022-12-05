use std::ops::Add;
use crate::utils;
use std::str;

#[derive(Debug)]
struct Stacks {
    towers: Vec<Vec<String>>,
}

fn new_stacks(towers: Vec<Vec<String>>) -> Stacks {
    return Stacks { towers };
}

impl Stacks {
    fn make_move(self, m: &Move) -> Stacks {
        let from_index = (m.from - 1) as usize;
        let to_index = (m.to - 1) as usize;
        let mut from_tower = self.towers.get(from_index)
            .unwrap()
            .clone();
        let mut to_tower = self.towers.get(to_index)
            .unwrap()
            .clone();

        for _ in 0..m.count {
            to_tower.push(from_tower.pop().unwrap());
        }

        let mut new_towers = self.towers.clone();
        new_towers[from_index] = from_tower;
        new_towers[to_index] = to_tower;

        return Stacks {
            towers: new_towers
        };
    }

    fn make_multi_move(self, m: &Move) -> Stacks {
        let from_index = (m.from - 1) as usize;
        let to_index = (m.to - 1) as usize;
        let mut from_tower = self.towers.get(from_index)
            .unwrap()
            .clone();
        let mut to_tower = self.towers.get(to_index)
            .unwrap()
            .clone();


        let mut mover: Vec<String> = Vec::new();

        for _ in 0..m.count {
            mover.push(from_tower.pop().unwrap());
        }

        for _ in 0..m.count {
            to_tower.push(mover.pop().unwrap());
        }

        let mut new_towers = self.towers.clone();
        new_towers[from_index] = from_tower;
        new_towers[to_index] = to_tower;

        return Stacks {
            towers: new_towers
        };
    }

    fn get_top_crates(&self) -> String {
        let top_crates: Vec<String> = self.towers.iter()
            .map(|x| x.last().unwrap().clone())
            .collect();
        return top_crates.join("");
    }
}

#[derive(Debug)]
struct Move {
    count: i32,
    from: i32,
    to: i32,
}

fn new_move(cmd: String) -> Move {
    let args: Vec<&str> = cmd.split(" ").collect();
    let count: i32 = args[1].parse().unwrap();
    let from: i32 = args[3].parse().unwrap();
    let to: i32 = args[5].parse().unwrap();
    return Move { count, from, to };
}


fn byte_to_letter(c: u8) -> String {
    let letter = str::from_utf8(&vec![c]).unwrap().to_string();
    return letter;
}

fn parse_tower_by_number(top_raw: &Vec<String>, number: i32) -> Vec<String> {
    let first_tower_index = 1;
    let increment = 4;
    let tower_index = (increment * (number - 1) + first_tower_index) as usize;

    let without_first = &top_raw[1..];

    let stack: Vec<String> = without_first.into_iter()
        .map(|x| right_pad(x.clone(), (tower_index + 1) as i32))
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
    let last_tower_number: i32 = byte_to_letter(last_byte.clone())
        .parse()
        .unwrap();

    return last_tower_number;
}

fn right_pad(s: String, required_len: i32) -> String {
    let current_len = s.len();
    let required_spaces = required_len - current_len as i32;
    if required_spaces < 0 {
        return s;
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

    return new_stacks(towers);
}

fn parse_bottom(raw: Vec<String>) -> Vec<Move> {
    return raw.into_iter()
        .filter(|x| x.starts_with("move"))
        .map(|x| new_move(x))
        .collect();
}

fn make_all_moves(s: Stacks, moves: Vec<Move>) -> Stacks {
    let mut stacks = s;
    for m in moves {
        stacks = stacks.make_move(&m);
    }

    return stacks;
}

fn make_all_multi_moves(s: Stacks, moves: Vec<Move>) -> Stacks {
    let mut stacks = s;
    for m in moves {
        stacks = stacks.make_multi_move(&m);
    }

    return stacks;
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day5.txt");

    let stacks = parse_top(raw.clone());
    let moves = parse_bottom(raw.clone());

    let complete = make_all_moves(stacks, moves);

    println!("{:?}", complete.get_top_crates())
}

pub fn part2() {
    let raw = utils::read_lines("./inputs/day5.txt");

    let stacks = parse_top(raw.clone());
    let moves = parse_bottom(raw.clone());
    let complete = make_all_multi_moves(stacks, moves);

    println!("{:?}", complete.get_top_crates())
}
