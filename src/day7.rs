use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::os::macos::raw::stat;
use crate::utils;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
enum LsResult {
    Dir(String),
    File(i32, String),
}

fn line_ls_result(line: String) -> LsResult {
    let split: Vec<&str> = line.split(" ").collect();
    let first = split.get(0).unwrap().clone();
    let name = split.get(1).unwrap().to_string();
    return match first {
        "dir" => LsResult::Dir(name),
        _ => {
            let size: i32 = first.parse().unwrap();
            LsResult::File(size, name)
        }
    };
}

fn new_ls_results(rest: String) -> Vec<LsResult> {
    return rest.split(&"\n")
        .filter(|x| !x.trim().is_empty())
        .map(|line| line_ls_result(line.to_string()))
        .collect();
}

#[derive(Debug)]
enum CommandType {
    Cd { path: String },
    Ls { results: Vec<LsResult> },
}

#[derive(Debug)]
struct Command {
    kind: CommandType,
}

#[derive(Debug)]
struct State {
    pwd: String,
    files: HashMap<String, i32>,
}


fn new_command(kind: &str, rest: &str) -> Command {
    return match kind {
        "cd" => Command { kind: CommandType::Cd { path: rest.trim().to_string() } },
        "ls" => {
            let results = new_ls_results(rest.to_string());
            Command { kind: CommandType::Ls { results } }
        }
        _ => panic!("Fug")
    };
}

fn parse_commands(raw: String) -> Vec<Command> {
    let re = Regex::new(r"\$\s(\w+)\s([/\w\s\\.]+)").unwrap();
    let commands: Vec<Command> = re.captures_iter(raw.as_str())
        .map(|x| new_command(&x[1], &x[2]))
        .collect();
    return commands;
}

fn cd_new_pwd(pwd: String, path: &str) -> String {
    lazy_static! {
        static ref re: Regex = Regex::new(r"(.*/)\w+/$").unwrap();
    }
    return match path {
        "/" => "/".to_string(),
        ".." => {
            let cap = re.captures(pwd.as_str()).unwrap();
            cap[1].to_string()
        }
        _ => format!("{}{}/", pwd, path)
    };
}

fn new_state(commands: Vec<Command>) -> State {
    let mut state = State {
        pwd: "/".to_string(),
        files: HashMap::new(),
    };
    for cmd in commands {
        match cmd.kind {
            CommandType::Cd { path } => {
                state.pwd = cd_new_pwd(state.pwd.clone(), path.as_str());
            }
            CommandType::Ls { results } => {
                for result in results {
                    match result {
                        LsResult::Dir(_) => {} // TODO
                        LsResult::File(size, path) => {
                            let full_name = format!("{}{}", state.pwd, path);
                            state.files.insert(full_name, size);
                        }
                    }
                }
            }
        }
    }
    return state;
}

fn file_to_folders(file_path: String) -> Vec<String> {
    let folders: Vec<String> = file_path.split("/")
        .map(|x| format!("/{}", x.to_string()))
        .collect();
    let mut folder_full: Vec<String> = Vec::new();
    let mut path_so_far = "".to_string();
    for x in folders {
        if path_so_far == "/" {
            path_so_far = x.clone();
            folder_full.push(path_so_far.to_string());
            continue;
        }
        path_so_far = format!("{}{}", path_so_far, x);
        folder_full.push(path_so_far.to_string());
    }
    folder_full.pop(); // Super ugly

    return folder_full;
}

fn find_folders(files: Vec<String>) -> Vec<String> {
    files.iter()
        .flat_map(|x| file_to_folders(x.to_string()))
        .collect()
}

fn get_folder_size(folder: String, files: HashMap<String, i32>) -> i32 {
    return files.into_iter()
        .filter(|(path, _)| path.starts_with(&folder))
        .map(|(_, size)| size)
        .sum();
}

fn calculate_folder_sizes(folders: HashSet<&String>, files: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut folder_sizes: HashMap<String, i32> = HashMap::new();
    for x in folders {
        folder_sizes.insert(x.clone(), get_folder_size(x.clone(), files.clone()));
    }
    return folder_sizes
}

pub fn part1() {
    let raw = utils::read_to_string("./inputs/day7.txt");
    let commands = parse_commands(raw);
    let state = new_state(commands);
    let files: Vec<String> = state.files.iter()
        .map(|(k, _)| k.clone())
        .collect();
    let folders_vec = find_folders(files.clone());
    let folders: HashSet<&String> = HashSet::from_iter(folders_vec.iter());
    let sizes = calculate_folder_sizes(folders, state.files.clone());

    let total: i32 = sizes.into_iter()
        .filter(|(k, v)| v.clone() <= 100000)
        .map(|(_, v)| v)
        .sum();

    println!("{}", total)
}
