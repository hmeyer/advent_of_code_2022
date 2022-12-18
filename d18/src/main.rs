use std::io;
use std::collections::{VecDeque, HashSet, HashMap};
use std::collections::hash_map::Entry;
use std::cmp::{max, min};
use std::fmt::Display;
use std::fmt;
use std::cmp::Ordering;
use std::ops::Range;


fn count_exposed(cubes: &HashSet<(i8, i8, i8)>) -> usize {
    let mut exposed = 0;
    for cube in cubes.iter() {
        for d in [-1, 1] {
            if !cubes.contains(&(cube.0 + d, cube.1, cube.2)) {
                exposed += 1;
            }
            if !cubes.contains(&(cube.0, cube.1 + d, cube.2)) {
                exposed += 1;
            }
            if !cubes.contains(&(cube.0, cube.1, cube.2 + d)) {
                exposed += 1;
            }
        }
    }
    exposed
}


struct Bound {
    pub x: Range<i8>,
    pub y: Range<i8>,
    pub z: Range<i8>,
    pub exterior: HashSet<(i8, i8, i8)>,
}

fn _fill_impl(pos: (i8, i8, i8), droplet: &HashSet<(i8, i8, i8)>, bound: &mut Bound) {
    if !(bound.x.contains(&pos.0) && bound.y.contains(&pos.1) && bound.z.contains(&pos.2)) {
        return;
    }
    if droplet.contains(&pos) {
        return;
    }
    if !bound.exterior.insert(pos) {
        return;
    }
    // println!("{:?}", pos);
    for d in [-1, 1] {
        _fill_impl((pos.0 + d, pos.1, pos.2), droplet, bound);
        _fill_impl((pos.0, pos.1 + d, pos.2), droplet, bound);
        _fill_impl((pos.0, pos.1, pos.2 + d), droplet, bound);
    }
}


fn fill_holes(droplet: &HashSet<(i8, i8, i8)>) -> HashSet<(i8, i8, i8)> {
    let mut bound = Bound{
        x: droplet.iter().map(|c| c.0).min().unwrap() - 1..droplet.iter().map(|c| c.0).max().unwrap() + 2,
        y: droplet.iter().map(|c| c.1).min().unwrap() - 1..droplet.iter().map(|c| c.1).max().unwrap() + 2,
        z: droplet.iter().map(|c| c.2).min().unwrap() - 1..droplet.iter().map(|c| c.2).max().unwrap() + 2,
        exterior: HashSet::new(),
    };
    _fill_impl((bound.x.start, bound.y.start, bound.z.start), droplet, &mut bound);
    println!("bound box: {}", bound.x.len() * bound.y.len() * bound.z.len());
    println!("bound count: {}", bound.exterior.len());
    let mut result = HashSet::new();
    for x in bound.x.clone() {
        for y in bound.y.clone() {
            for z in bound.z.clone() {
                if !bound.exterior.contains(&(x, y, z)) {
                    result.insert((x, y, z));
                }
            }
        }
    }
    println!("droplet count: {}", droplet.len());
    println!("result count: {}", result.len());
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let cubes = stdin.lines().map(|l| {
        let l = l.unwrap();
        let mut c = l.split(',').map(|c| c.parse::<i8>().unwrap());
        (c.next().unwrap(), c.next().unwrap(), c.next().unwrap())
    }).collect::<HashSet<_>>();
    println!("exposed {}", count_exposed(&cubes));
    println!("exposed {}", count_exposed(&fill_holes(&cubes)));


    Ok(())
}
