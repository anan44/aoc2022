use crate::utils;

struct Elf {
    foods: Vec<i32>
}

impl Elf {
    fn total_calories(&self) -> i32 {
        self.foods.iter().sum()
    }
}

fn to_elves(sv: Vec<String>) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();
    let mut foods: Vec<i32> = Vec::new();
    for x in sv {
        if x == "" {
            let elf = Elf{ foods };
            elves.push(elf);
            foods = Vec::new();
            continue
        }
        let calories = x.parse::<i32>().unwrap();
        foods.push(calories);
    }
    return elves
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day1.txt");
    let elves: Vec<Elf> = to_elves(raw);
    let fat_elf = elves
        .iter()
        .map(|x| x.total_calories())
        .max()
        .unwrap();
    println!("{}", fat_elf)
}

pub fn part2() {
    let raw = utils::read_lines("./inputs/day1.txt");
    let elves: Vec<Elf> = to_elves(raw);
    let mut foods: Vec<i32> = elves
        .iter()
        .map(|x| x.total_calories())
        .collect();
    foods.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top3_total: i32 = foods[0..3]
        .to_vec()
        .iter()
        .sum();
    println!("{}", top3_total);
}
