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
    return (raw + 1) * cycle;
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

fn is_on_sprite(index: i32, sprite: i32) -> bool {
    return index >= sprite && index <= sprite + 2;
}

fn calc_all_sprite_positions(cmds: Vec<Command>) -> Vec<i32> {
    let mut positions = Vec::new();
    let mut wait1 = 0;
    let mut wait2 = 0;
    for c in cmds {
        positions.push(wait2.clone());
        wait2 = wait1;
        wait1 = c.to_int() + wait2;
    }
    return positions;
}

fn draw_crt(positions: Vec<i32>) {
    let indexed: Vec<(i32, i32)> = positions.iter()
        .zip(0..positions.len())
        .map(|(x, i)| (x.clone(), i as i32))
        .collect();

    let pixels: Vec<String> = indexed.iter()
        .map(|(pos, index)| draw_cell(pos.clone(), index.clone()))
        .collect();

    for row in pixels.chunks(40) {
        println!("{:?}", row.join(""))
    }
}

fn line_correct_cell(sprite_pos: i32) -> i32 {
    return sprite_pos % 40;
}

fn draw_cell(sprite_pos: i32, cell_index: i32) -> String {
    let cell = line_correct_cell(cell_index);
    if cell >= sprite_pos && cell <= sprite_pos + 2 {
        return "#".to_string();
    }
    return ".".to_string();
}


pub fn part2() {
    let raw = utils::read_lines("./inputs/day10.txt");
    let commands: Vec<Command> = raw.iter()
        .map(|row| new_command(row.clone()))
        .collect();
    let cycles: Vec<Command> = commands.iter()
        .flat_map(|x| x.to_cycles())
        .collect();
    let positions = calc_all_sprite_positions(cycles);
    draw_crt(positions);
}