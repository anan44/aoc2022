use crate::utils;

#[derive(Clone)]
#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn to_int(&self) -> i32 {
        match self {
            Command::Noop => 0,
            Command::Addx(v) => v.clone(),
        }
    }
    fn to_cycles(&self) -> Vec<Command> {
        match self {
            Command::Noop => vec![Command::Noop],
            Command::Addx(v) => vec![self.clone(), Command::Noop],
        }
    }
}

fn new_command(row: String) -> Command {
    let split: Vec<&str> = row.split(" ").collect();
    let cmd = split.get(0).unwrap().clone();
    return match cmd {
        "noop" => Command::Noop,
        "addx" => {
            let val: i32 = split.get(1).unwrap().parse().unwrap();
            return Command::Addx(val);
        }
        _ => panic!("unknown cmd")
    };
}

fn calculate_signal_for_cycle(cmds: Vec<Command>, cycle: i32) -> i32 {
    let raw: i32 = cmds[0..(cycle - 2) as usize].iter()
        .map(|x| x.to_int())
        .sum();
    return (raw + 1) * cycle
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day10.txt");
    let commands: Vec<Command> = raw.iter()
        .map(|row| new_command(row.clone()))
        .collect();
    let cycles: Vec<Command> = commands.iter()
        .flat_map(|x| x.to_cycles())
        .collect();
    let total: i32 = vec![20, 60, 100, 140, 180, 220].iter()
        .map(|x| calculate_signal_for_cycle(cycles.clone(), x.clone()))
        .sum();
    println!("{}", total)
}