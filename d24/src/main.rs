use std::io;
use std::collections::{VecDeque, HashSet, HashMap, BinaryHeap};
use std::cmp::{max, min};
use std::fmt::Display;
use std::fmt;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

fn parse_blizzards(lines: &[String]) -> (Vec2, HashSet<(char, i32, i32)>) {
    let lines = lines[1..lines.len()-1].into_iter().collect::<Vec<_>>();
    let lines = lines.into_iter().map(|l| l[1..l.len()-1].to_string()).collect::<Vec<_>>();
    let mut blizzards = HashSet::new();
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                blizzards.insert((c, x as i32, y as i32));
            }
        }
    }
    (Vec2 {x: lines[0].len() as i32, y: lines.len() as i32}, blizzards)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct State {
    neg_time: i32,
    p: Vec2,
}

impl State {
    fn new(time: i32, p: Vec2) -> Self {
        State { neg_time: -time, p }
    }
    fn time(&self) -> i32 {
        -self.neg_time
    }
}

fn has_blizzard(s: &State, size: Vec2, blizzards: &HashSet<(char, i32, i32)>) -> bool {
    blizzards.get(&('>', (s.p.x - s.time()).rem_euclid(size.x), s.p.y)).is_some() ||
    blizzards.get(&('<', (s.p.x + s.time()).rem_euclid(size.x), s.p.y)).is_some() ||
    blizzards.get(&('v', s.p.x, (s.p.y - s.time()).rem_euclid(size.y))).is_some() ||
    blizzards.get(&('^', s.p.x, (s.p.y + s.time()).rem_euclid(size.y))).is_some()
}

fn reconstruct(mut p: State, came_from: &HashMap<State, State>) -> Vec<Vec2> {
    let mut r = Vec::new();
    r.push(p.p);
    while let Some(c) = came_from.get(&p) {
        r.push(c.p);
        p = *c;
    }
    r.reverse();
    r
}

fn dijkstra(start: State, goal: Vec2, size: Vec2, blizzards: &HashSet<(char, i32, i32)>) -> Vec<Vec2> {
    let lcm = match size {
        Vec2 { x: 5, y: 5 } => 25,
        Vec2 { x: 6, y: 4 } => 12,
        Vec2 { x: 120, y: 25 } => 600,
        _ => panic!("unexpected size: {:?}", size),
    };
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();
    dist.insert((start.p, start.time() % lcm), start.time());
    heap.push(start);

    let mut came_from = HashMap::new();

    while let Some(current) = heap.pop() {
        if current.p == goal {
            return reconstruct(current, &came_from);
        }

        if let Some(d) = dist.get(&(current.p, current.time() % lcm)) {
            if current.time() > *d {
                continue;
            }
        }
        for yd in -1..=1_i32 {
            for xd in -1..=1_i32 {
                if xd.abs() + yd.abs() > 1 {
                    continue;
                }
                let next_pos = Vec2 { x: current.p.x + xd, y: current.p.y + yd };
                if next_pos.x < 0 || next_pos.x == size.x || next_pos.y < 0 || next_pos.y == size.y {
                    // Allow entry && exit
                    if !(next_pos.x == 0 && next_pos.y == -1) && !(next_pos.x == size.x - 1 && next_pos.y == size.y) {
                        // Must not leave the map.
                        continue;
                    }
                }
                let next = State::new(current.time() + 1, next_pos);
                if has_blizzard(&next, size, blizzards) {
                    continue;
                }
                let d = dist.entry((next.p, next.time() % lcm)).or_insert(i32::MAX);
                if next.time() < *d {
                    came_from.insert(next, current);
                    heap.push(next);
                    *d = next.time();
                }
            }
        }

    }
    panic!("did not find a path!");
}

fn get_blizzard(s: &State, size: Vec2, blizzards: &HashSet<(char, i32, i32)>) -> char {
    let mut b = Vec::new();
    if blizzards.get(&('>', (s.p.x - s.time()).rem_euclid(size.x), s.p.y)).is_some() {
        b.push('>');
    }
    if blizzards.get(&('<', (s.p.x + s.time()).rem_euclid(size.x), s.p.y)).is_some() {
        b.push('<');
    }
    if blizzards.get(&('v', s.p.x, (s.p.y - s.time()).rem_euclid(size.y))).is_some() {
        b.push('v');
    }
    if blizzards.get(&('^', s.p.x, (s.p.y + s.time()).rem_euclid(size.y))).is_some() {
        b.push('^');
    }
    match b.len() {
        0 => '.',
        1 => b[0],
        l => std::char::from_digit(l as u32, 10).unwrap(),
    }
}

fn print_map(size: Vec2, e: Vec2, t: i32, blizzards: &HashSet<(char, i32, i32)>) {
    print!("#{}", if e == (Vec2 { x: 0, y: -1 }) { 'E' } else { '.' });
    for _ in 0..size.x {
        print!("#");
    }
    println!("");
    for y in 0..size.y {
        print!("#");
        for x in 0..size.x {
            let b = get_blizzard(&State::new(t, Vec2 { x, y }), size, blizzards);
            if e == (Vec2 { x, y }) {
                if b != '.' {
                    panic!("Collision at {:?} time: {}", (x, y), t);
                }
                    print!("E");
            } else {
                print!("{}", b);
            }
        }
        println!("#");
    }
    for _ in 0..size.x {
        print!("#");
    }
    println!(".#");
}

fn print_path(size: Vec2, path: &[Vec2], blizzards: &HashSet<(char, i32, i32)>) {
    for (t, p) in path.iter().enumerate() {
        println!("Minute {}:", t);
        print_map(size, *p, t as i32, blizzards);
        println!("");
    }
}



fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let (size, blizzards) = parse_blizzards(&stdin.lines().map(|l| l.unwrap()).collect::<Vec<_>>());
    
    let start = Vec2 { x: 0, y: -1};
    let goal = Vec2 { x: size.x - 1, y: size.y - 1};
    let p1 = dijkstra(State::new(0, start), goal, size, &blizzards);
    // print_path(size, &p, &blizzards);
    println!("start->goal takes {} Minutes", p1.len());


    let start = Vec2 { x: size.x - 1, y: size.y};
    let goal = Vec2 { x: 0, y: 0};
    let p2 = dijkstra(State::new(p1.len() as i32, start), goal, size, &blizzards);
    println!("start<-goal takes {} Minutes", p2.len());

    let start = Vec2 { x: 0, y: -1};
    let goal = Vec2 { x: size.x - 1, y: size.y - 1};
    let p3 = dijkstra(State::new((p1.len() + p2.len()) as i32, start), goal, size, &blizzards);
    println!("start->goal takes {} Minutes", p3.len());

    println!("total time: {} + {} + {} = {}", p1.len(), p2.len(), p3.len(), p1.len() + p2.len() + p3.len());

    Ok(())
}
