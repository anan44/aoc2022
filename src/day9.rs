use std::collections::HashSet;
use crate::utils;

#[derive(Clone)]
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Skip,
}

fn new_direction(x: &str) -> Direction {
    match x {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unknown direction")
    }
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    dist: i32,
}

impl Move {
    fn to_steps(&self) -> Vec<Direction> {
        let mut steps: Vec<Direction> = Vec::new();
        for _ in 0..self.dist {
            steps.push(self.dir.clone());
        }
        return steps;
    }
}

fn moves_to_steps(moves: Vec<Move>) -> Vec<Direction> {
    return moves.iter()
        .flat_map(|x| x.to_steps())
        .collect();
}

fn new_move(row: String) -> Move {
    let split: Vec<&str> = row.split(&" ").collect();

    let dist: i32 = split.get(1).unwrap().parse().unwrap();
    return Move {
        dir: new_direction(split.get(0).unwrap()),
        dist,
    };
}

fn parse_moves(raw: Vec<String>) -> Vec<Move> {
    let moves: Vec<Move> = raw.into_iter()
        .map(|row| new_move(row))
        .collect();

    return moves;
}

#[derive(Debug)]
struct State {
    head: (i32, i32),
    tail: (i32, i32),
    tail_positions: HashSet<(i32, i32)>,
}

impl State {
    fn make_move(&mut self, m: Direction) -> Direction {
        return match m {
            Direction::Up => {
                let new_head = (self.head.0, self.head.1 + 1);
                return self.move_helper(new_head);
            }
            Direction::Down => {
                let new_head = (self.head.0, self.head.1 - 1);
                return self.move_helper(new_head);
            }
            Direction::Right => {
                let new_head = (self.head.0 + 1, self.head.1);
                return self.move_helper(new_head);
            }
            Direction::Left => {
                let new_head = (self.head.0 - 1, self.head.1);
                return self.move_helper(new_head);
            }

            Direction::UpRight => {
                let new_head = (self.head.0 + 1, self.head.1 + 1);
                return self.move_helper(new_head);
            }
            Direction::UpLeft => {
                let new_head = (self.head.0 - 1, self.head.1 + 1);
                return self.move_helper(new_head);
            }
            Direction::DownRight => {
                let new_head = (self.head.0 + 1, self.head.1 - 1);
                return self.move_helper(new_head);
            }
            Direction::DownLeft => {
                let new_head = (self.head.0 - 1, self.head.1 - 1);
                return self.move_helper(new_head);
            }
            Direction::Skip => {
                return Direction::Skip
            }
        }
    }

    fn move_helper(&mut self, new_head: (i32, i32)) -> Direction {
        if tail_needs_to_move(new_head, self.tail) {
            let mut helper = (0, 0);

            // X move
            if new_head.0 > self.tail.0 {
                helper.0 += 1;
            }
            if new_head.0 < self.tail.0 {
                helper.0 -= 1;
            }

            // Y move
            if new_head.1 > self.tail.1 {
                helper.1 += 1;
            }
            if new_head.1 < self.tail.1 {
                helper.1 -= 1;
            }

            let new_tail = (self.tail.0 + helper.0, self.tail.1 + helper.1);
            self.tail_positions.insert(new_tail);
            self.tail = new_tail;
            self.head = new_head;
            return cord_to_dir(helper);
        }

        self.head = new_head;
        return Direction::Skip;
    }
}

fn cord_to_dir(cord: (i32, i32)) -> Direction {
    match cord {
        (1, 0) => Direction::Right,
        (-1, 0) => Direction::Left,
        (0, 1) => Direction::Up,
        (0, -1) => Direction::Down,
        (1, 1) => Direction::UpRight,
        (-1, 1) => Direction::UpLeft,
        (1, -1) => Direction::DownRight,
        (-1, -1) => Direction::DownLeft,
        _ => Direction::Skip,
    }
}

fn tail_needs_to_move(head: (i32, i32), tail: (i32, i32)) -> bool {
    let x_distance = i32::abs(tail.0 - head.0);
    let y_distance = i32::abs(tail.1 - head.1);
    if x_distance < 2 && y_distance < 2 {
        return false;
    }
    return true;
}


pub fn part1() {
    let raw = utils::read_lines("./inputs/day9.txt");
    let moves = parse_moves(raw);
    let steps = moves_to_steps(moves);

    let mut state = State {
        head: (0, 0),
        tail: (0, 0),
        tail_positions: HashSet::new(),
    };


    state.tail_positions.insert((0, 0));
    for m in steps.iter() {
        state.make_move(m.clone());
    }
    println!("{:?}", state.tail_positions.len())
}

fn move_part(steps: Vec<Direction>) -> (Vec<Direction>, usize) {
    let mut state = State {
        head: (0, 0),
        tail: (0, 0),
        tail_positions: HashSet::new(),
    };
    state.tail_positions.insert((0, 0));
    let mut next_round: Vec<Direction> = Vec::new();
    for m in steps.iter() {
        next_round.push(state.make_move(m.clone()));
    }
    return (next_round, state.tail_positions.len())
}

pub fn part2() {
    let raw = utils::read_lines("./inputs/day9.txt");
    let moves = parse_moves(raw);
    let mut steps = moves_to_steps(moves);
    let mut result = 0;

    for _ in 0..9 {
        let (new_steps, new_result) = move_part(steps.clone());
        steps = new_steps;
        result = new_result;
    }

    println!("{:?}", result)
}