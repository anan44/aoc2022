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
    fn make_move(&mut self, m: Direction) {
        match m {
            Direction::Up => {
                let new_head = (self.head.0, self.head.1 + 1);
                self.move_helper(new_head);
            }
            Direction::Down => {
                let new_head = (self.head.0, self.head.1 - 1);
                self.move_helper(new_head);
            }
            Direction::Right => {
                let new_head = (self.head.0 + 1, self.head.1);
                self.move_helper(new_head);
            }
            Direction::Left => {
                let new_head = (self.head.0 - 1, self.head.1);
                self.move_helper(new_head);
            }

            Direction::UpRight => {
                let new_head = (self.head.0 + 1, self.head.1 + 1);
                self.move_helper(new_head);
            }
            Direction::UpLeft => {
                let new_head = (self.head.0 - 1, self.head.1 + 1);
                self.move_helper(new_head);
            }
            Direction::DownRight => {
                let new_head = (self.head.0 + 1, self.head.1 - 1);
                self.move_helper(new_head);
            }
            Direction::DownLeft => {
                let new_head = (self.head.0 - 1, self.head.1 - 1);
                self.move_helper(new_head);
            }
        }
    }

    fn move_helper(&mut self, new_head: (i32, i32)) {
        if tail_needs_to_move(new_head, self.tail) {
            let mut new_tail_x = self.tail.0;
            let mut new_tail_y = self.tail.1;

            // X move
            if new_head.0 > self.tail.0 {
                new_tail_x += 1;
            }
            if new_head.0 < self.tail.0 {
                new_tail_x -= 1;
            }

            // Y move
            if new_head.1 > self.tail.1 {
                new_tail_y += 1;
            }
            if new_head.1 < self.tail.1 {
                new_tail_y -= 1;
            }

            let new_tail = (new_tail_x, new_tail_y);
            self.tail_positions.insert(new_tail);
            if self.tail == new_tail {
                println!("{:?} {:?}", new_head, self.tail)
            }
            self.tail = new_tail;
        }

        self.head = new_head;
    }
}

fn tail_needs_to_move(head: (i32, i32), tail: (i32, i32)) -> bool{
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


pub fn part2() {
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