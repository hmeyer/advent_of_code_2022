use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Display;
use std::io;
use std::ops::RangeInclusive;

fn bounds(elves: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    (
        (
            elves.iter().map(|e| e.0).min().unwrap(),
            elves.iter().map(|e| e.0).max().unwrap(),
        ),
        (
            elves.iter().map(|e| e.1).min().unwrap(),
            elves.iter().map(|e| e.1).max().unwrap(),
        ),
    )
}

fn print_elves(elves: &HashSet<(i32, i32)>) {
    let ((x_min, x_max), (y_min, y_max)) = bounds(elves);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if elves.get(&(x, y)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn move_elves(round: usize, elves: &HashSet<(i32, i32)>) -> Option<HashSet<(i32, i32)>> {
    let mut proposals = HashMap::new();
    for e in elves.iter() {
        if let Some(p) = check_and_propose(round, *e, elves) {
            *proposals.entry(p).or_insert(0) += 1;
        }
    }
    if proposals.len() == 0 {
        return None;
    }
    let mut nelves = HashSet::new();
    for e in elves.iter() {
        let mut e = *e;
        if let Some(p) = check_and_propose(round, e, elves) {
            if *proposals.get(&p).unwrap() == 1 {
                e = p;
            }
        }
        assert!(nelves.insert(e));
    }
    Some(nelves)
}

fn check_and_propose(
    round: usize,
    pos: (i32, i32),
    elves: &HashSet<(i32, i32)>,
) -> Option<(i32, i32)> {
    let mut found_others = false;
    for yd in -1..=1 {
        for xd in -1..=1 {
            if xd == 0 && yd == 0 {
                continue;
            }
            if elves.get(&(pos.0 + xd, pos.1 + yd)).is_some() {
                found_others = true;
                break;
            }
        }
        if found_others {
            break;
        }
    }
    if !found_others {
        // If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
        return None;
    }
    for r in round..round + 4 {
        let dir = r % 4;
        if dir < 2 {
            // North or South
            let yd = if dir == 0 { -1 } else { 1 };
            let mut found_others = false;
            for xd in -1..=1 {
                if elves.get(&(pos.0 + xd, pos.1 + yd)).is_some() {
                    found_others = true;
                    break;
                }
            }
            if !found_others {
                return Some((pos.0, pos.1 + yd));
            }
        } else {
            // West or East
            let xd = if dir == 2 { -1 } else { 1 };
            let mut found_others = false;
            for yd in -1..=1 {
                if elves.get(&(pos.0 + xd, pos.1 + yd)).is_some() {
                    found_others = true;
                    break;
                }
            }
            if !found_others {
                return Some((pos.0 + xd, pos.1));
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut elves = HashSet::new();
    for (y, l) in stdin.lines().enumerate() {
        let l = l.unwrap();
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }
    let mut r = 0;
    loop {
        if let Some(nelves) = move_elves(r, &elves) {
            elves = nelves;
        } else {
            break;
        }
        r += 1;
    }
    print_elves(&elves);

    let ((x_min, x_max), (y_min, y_max)) = bounds(&elves);
    let xd = x_max - x_min + 1;
    let yd = y_max - y_min + 1;
    println!(
        "ground covered {} x {} - {} = {}",
        xd,
        yd,
        elves.len(),
        xd * yd - elves.len() as i32
    );
    println!("number of rounds: {}", r + 1);

    Ok(())
}
