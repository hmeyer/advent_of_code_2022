use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::Display;
use std::io;

fn parse_num(l: &str) -> (i32, &str) {
    if let Some(p) = l.find(|c: char| !c.is_numeric()) {
        (l[..p].parse::<i32>().unwrap(), &l[p..])
    } else {
        (l.parse::<i32>().unwrap(), &l[l.len()..])
    }
}

fn parse_path(mut l: &str) -> Vec<(i32, i32)> {
    let mut p = Vec::new();
    while l.len() > 0 {
        let (x, r) = parse_num(l);
        l = &r[1..];
        let (y, r) = parse_num(l);
        if r.len() > 0 {
            l = &r[4..];
        } else {
            l = r;
        }
        p.push((y, x));
    }
    p
}

fn draw_path(grid: &mut HashSet<(i32, i32)>, path: &[(i32, i32)]) {
    for i in 1..path.len() {
        let mut s = path[i - 1];
        let e = path[i];
        let steps = max((s.0 - e.0).abs(), (s.1 - e.1).abs());
        for _ in 0..=steps {
            grid.insert(s);
            s.0 += (e.0 - s.0).signum();
            s.1 += (e.1 - s.1).signum();
        }
    }
}

fn drop(grid: &mut HashSet<(i32, i32)>, lowest: i32) -> bool {
    let mut p = (0, 500);
    loop {
        if p.0 == lowest {
            return false;
        }

        if grid.get(&(p.0 + 1, p.1)).is_none() {
            p = (p.0 + 1, p.1);
        } else if grid.get(&(p.0 + 1, p.1 - 1)).is_none() {
            p = (p.0 + 1, p.1 - 1);
        } else if grid.get(&(p.0 + 1, p.1 + 1)).is_none() {
            p = (p.0 + 1, p.1 + 1);
        } else {
            grid.insert(p);
            return true;
        }
    }
}

fn drop_floor(grid: &mut HashSet<(i32, i32)>, floor: i32) -> bool {
    let mut p = (0, 500);
    if grid.get(&p).is_some() {
        return false;
    }

    loop {
        if p.0 == floor - 1 {
            grid.insert(p);
            return true;
        }
        if grid.get(&(p.0 + 1, p.1)).is_none() {
            p = (p.0 + 1, p.1);
        } else if grid.get(&(p.0 + 1, p.1 - 1)).is_none() {
            p = (p.0 + 1, p.1 - 1);
        } else if grid.get(&(p.0 + 1, p.1 + 1)).is_none() {
            p = (p.0 + 1, p.1 + 1);
        } else {
            grid.insert(p);
            return true;
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut grid = HashSet::new();
    for l in lines {
        draw_path(&mut grid, &parse_path(&l));
    }

    let lowest = *grid.iter().map(|(y, x)| y).max().unwrap();
    println!("lowest {}", lowest);

    let mut grains_to_abyss = 0;
    while drop(&mut grid, lowest) {
        grains_to_abyss += 1;
    }
    println!("grains till abyss {}", grains_to_abyss);

    let mut grains_to_floor = grains_to_abyss;
    while drop_floor(&mut grid, lowest + 2) {
        grains_to_floor += 1;
    }
    println!("grains till floor {}", grains_to_floor);
    Ok(())
}
