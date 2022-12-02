use crate::utils;

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn new_hand(input: &str) -> Result<Hand, &str> {
    return match input {
        "A" | "X" => Ok(Hand::Rock),
        "B" | "Y" => Ok(Hand::Paper),
        "C" | "Z" => Ok(Hand::Scissors),
        _ => Err("unknown")
    };
}

#[derive(Debug)]
enum Goal {
    Win,
    Draw,
    Lose,
}

fn new_goal(input: &str) -> Result<Goal, &str> {
    return match input {
        "X" => Ok(Goal::Lose),
        "Y" => Ok(Goal::Draw),
        "Z" => Ok(Goal::Win),
        _ => Err("unknown")
    }
}

#[derive(Debug)]
struct Round {
    opponent: Hand,
    me: Hand,
}

impl Round {
    fn count_points(&self) -> i32 {
        match self {
            // Winn
            Round { me: Hand::Scissors, opponent: Hand::Paper } => 9, // 6 + 3
            Round { me: Hand::Paper, opponent: Hand::Rock } => 8, // 6 + 2
            Round { me: Hand::Rock, opponent: Hand::Scissors } => 7, // 6 + 1

            // Tie
            Round { me: Hand::Scissors, opponent: Hand::Scissors } => 6, // 3 + 3
            Round { me: Hand::Paper, opponent: Hand::Paper } => 5, // 3 + 2
            Round { me: Hand::Rock, opponent: Hand::Rock } => 4, // 3 + 1

            // Loss
            Round { me: Hand::Scissors, opponent: Hand::Rock } => 3, // 0 + 3
            Round { me: Hand::Paper, opponent: Hand::Scissors } => 2, // 0 + 2
            Round { me: Hand::Rock, opponent: Hand::Paper } => 1, // 0 + 1
        }
    }
}

fn new_round(input: &str) -> Round {
    let symbols: Vec<&str> = input.split(" ").collect();
    return Round {
        opponent: new_hand(symbols[0]).unwrap(),
        me: new_hand(symbols[1]).unwrap(),
    };
}

#[derive(Debug)]
struct RoundPlan {
    opponent: Hand,
    goal: Goal
}

fn new_round_plan(input: &str) -> RoundPlan {
    let symbols: Vec<&str> = input.split(" ").collect();
    return RoundPlan {
        opponent: new_hand(symbols[0]).unwrap(),
        goal: new_goal(symbols[1]).unwrap(),
    };
}

impl RoundPlan {
    fn count_points(&self) -> i32 {
        return match self {
            // Win
            RoundPlan { goal: Goal::Win, opponent: Hand::Paper } => 9,
            RoundPlan { goal: Goal::Win, opponent: Hand::Rock } => 8,
            RoundPlan { goal: Goal::Win, opponent: Hand::Scissors } => 7,

            // Draw
            RoundPlan { goal: Goal::Draw, opponent: Hand::Scissors } => 6,
            RoundPlan { goal: Goal::Draw, opponent: Hand::Paper } => 5,
            RoundPlan { goal: Goal::Draw, opponent: Hand::Rock } => 4,

            // Lose
            RoundPlan { goal: Goal::Lose, opponent: Hand::Rock } => 3,
            RoundPlan { goal: Goal::Lose, opponent: Hand::Scissors } => 2,
            RoundPlan { goal: Goal::Lose, opponent: Hand::Paper } => 1,
        }
    }
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day2.txt");
    let rounds: i32 = raw.iter()
        .map(|s| new_round(s))
        .map(|r| r.count_points())
        .sum();

    println!("{}", rounds)
}

pub fn part2() {
    let raw = utils::read_lines("./inputs/day2.txt");
    let rounds: i32 = raw.iter()
        .map(|s| new_round_plan(s))
        .map(|r| r.count_points())
        .sum();

    println!("{}", rounds)

}