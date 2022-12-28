use std::collections::{HashMap, VecDeque};
use std::io;

enum Line {
    Cd(String),
    CdDotDot,
    FileSize(usize),
}

fn parse_line(l: &str) -> Option<Line> {
    let mut parts = l.split(" ");
    let first = parts.next().unwrap();
    match first {
        "$" => match parts.next().unwrap() {
            "ls" => None,
            "cd" => match parts.next().unwrap() {
                ".." => Some(Line::CdDotDot),
                x => Some(Line::Cd(x.to_string())),
            },
            x => panic!("unknown command: {}", x),
        },
        "dir" => None,
        _ => Some(Line::FileSize(first.parse::<usize>().unwrap())),
    }
}

fn add_sizes(dir_sizes: &mut HashMap<String, usize>, path: &Vec<String>, size: usize) {
    let mut p = String::new();
    for x in path {
        p = format!("{}/{}", p, x);
        let path_entry = dir_sizes.entry(p.clone()).or_default();
        *path_entry += size;
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    let mut path: Vec<String> = Vec::new();
    for l in stdin.lines() {
        if let Some(c) = parse_line(&l.unwrap()) {
            match c {
                Line::Cd(x) => path.push(x),
                Line::CdDotDot => {
                    path.pop();
                }
                Line::FileSize(s) => add_sizes(&mut dir_sizes, &path, s),
            }
        }
    }
    println!("dir_sizes: {:?}", dir_sizes);
    let mut sum_100000 = 0;
    for s in dir_sizes.values() {
        if *s < 100000 {
            sum_100000 += s;
        }
    }
    println!("sum_100000: {}", sum_100000);

    let total: usize = *dir_sizes.get("//").unwrap();
    let free = 70000000 - total;
    let required = 30000000 - free;
    println!("required: {}", required);
    let mut min_size = total;
    for s in dir_sizes.values() {
        if *s >= required && *s < min_size {
            min_size = *s;
        }
    }
    println!("min_size: {}", min_size);

    Ok(())
}
