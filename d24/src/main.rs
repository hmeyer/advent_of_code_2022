use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io;

fn parse_blizzards(lines: &[String]) -> (Vec2, HashSet<(char, i32, i32)>) {
    let lines = lines[1..lines.len() - 1].into_iter().collect::<Vec<_>>();
    let lines = lines
        .into_iter()
        .map(|l| l[1..l.len() - 1].to_string())
        .collect::<Vec<_>>();
    let mut blizzards = HashSet::new();
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                blizzards.insert((c, x as i32, y as i32));
            }
        }
    }
    (
        Vec2 {
            x: lines[0].len() as i32,
            y: lines.len() as i32,
        },
        blizzards,
    )
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct State {
    // Rusts BinaryHeap is a Max-Heap. But in Dijkstra we want to retrieve the
    // next node with minimal time. So we store the negative time instead, b/c -max(-time) = min(time).
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
    blizzards
        .get(&('>', (s.p.x - s.time()).rem_euclid(size.x), s.p.y))
        .is_some()
        || blizzards
            .get(&('<', (s.p.x + s.time()).rem_euclid(size.x), s.p.y))
            .is_some()
        || blizzards
            .get(&('v', s.p.x, (s.p.y - s.time()).rem_euclid(size.y)))
            .is_some()
        || blizzards
            .get(&('^', s.p.x, (s.p.y + s.time()).rem_euclid(size.y)))
            .is_some()
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

fn possible_moves<'a>(
    current: State,
    size: Vec2,
    blizzards: &'a HashSet<(char, i32, i32)>,
) -> impl std::iter::Iterator<Item = State> + 'a {
    let mut n = 0;
    std::iter::from_fn(move || {
        for i in n..5 {
            let (xd, yd): (i32, i32) = match i {
                0 | 1 => (0, (i & 1) * 2 - 1),
                2 | 3 => ((i & 1) * 2 - 1, 0),
                4 => (0, 0),
                _ => panic!("{} should never be greater than 4!", i),
            };
            let next_pos = Vec2 {
                x: current.p.x + xd,
                y: current.p.y + yd,
            };
            // Must not leave the map.
            if (next_pos.x < 0 || next_pos.x == size.x || next_pos.y < 0 || next_pos.y == size.y) &&
                // Allow entry
                !(next_pos.x == 0 && next_pos.y == -1) &&
                // Allow exit
                !(next_pos.x == size.x - 1 && next_pos.y == size.y)
            {
                continue;
            }
            let next = State::new(current.time() + 1, next_pos);
            if has_blizzard(&next, size, blizzards) {
                continue;
            }
            n = i + 1;
            return Some(next);
        }
        n = 5;
        None
    })
}

fn lcm(a: i32, b: i32) -> i32 {
    match (a, b) {
        (5, 5) => 25,
        (6, 4) => 12,
        (120, 25) => 600,
        _ => unimplemented!("for args: {}, {}", a, b),
    }
}

fn dijkstra(
    start: State,
    goal: Vec2,
    size: Vec2,
    blizzards: &HashSet<(char, i32, i32)>,
) -> Vec<Vec2> {
    let lcm = lcm(size.x, size.y);
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
        for next in possible_moves(current, size, blizzards) {
            let d = dist.entry((next.p, next.time() % lcm)).or_insert(i32::MAX);
            if next.time() < *d {
                came_from.insert(next, current);
                heap.push(next);
                *d = next.time();
            }
        }
    }
    panic!("did not find a path!");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct FringeItem {
    // Rusts BinaryHeap is a Max-Heap. But in A* we want to retrieve the
    // next node with the minimal cost estimate.
    // So we store the negative time instead, b/c -max(-cost_estimate) = min(cost_estimate).
    neg_cost_estimate: i32,
    state: State,
}

impl FringeItem {
    fn new(state: State, goal: Vec2) -> Self {
        let manhattan = (state.p.x - goal.x).abs() + (state.p.y - goal.y).abs();
        FringeItem {
            neg_cost_estimate: -(state.time() + manhattan),
            state,
        }
    }
}

fn a_star(
    start: State,
    goal: Vec2,
    size: Vec2,
    blizzards: &HashSet<(char, i32, i32)>,
) -> Vec<Vec2> {
    let lcm = lcm(size.x, size.y);
    let mut fringe = BinaryHeap::new();
    let mut dist = HashMap::new();
    dist.insert((start.p, start.time() % lcm), start.time());
    fringe.push(FringeItem::new(start, goal));

    let mut came_from = HashMap::new();

    while let Some(current) = fringe.pop() {
        if current.state.p == goal {
            return reconstruct(current.state, &came_from);
        }
        for neighbor in possible_moves(current.state, size, blizzards) {
            let d = dist
                .entry((neighbor.p, neighbor.time() % lcm))
                .or_insert(i32::MAX);
            if neighbor.time() < *d {
                came_from.insert(neighbor, current.state);
                *d = neighbor.time();
                fringe.push(FringeItem::new(neighbor, goal));
            }
        }
    }
    panic!("did not find a path!");
}

fn get_blizzard(s: &State, size: Vec2, blizzards: &HashSet<(char, i32, i32)>) -> char {
    let mut b = Vec::new();
    if blizzards
        .get(&('>', (s.p.x - s.time()).rem_euclid(size.x), s.p.y))
        .is_some()
    {
        b.push('>');
    }
    if blizzards
        .get(&('<', (s.p.x + s.time()).rem_euclid(size.x), s.p.y))
        .is_some()
    {
        b.push('<');
    }
    if blizzards
        .get(&('v', s.p.x, (s.p.y - s.time()).rem_euclid(size.y)))
        .is_some()
    {
        b.push('v');
    }
    if blizzards
        .get(&('^', s.p.x, (s.p.y + s.time()).rem_euclid(size.y)))
        .is_some()
    {
        b.push('^');
    }
    match b.len() {
        0 => '.',
        1 => b[0],
        l => std::char::from_digit(l as u32, 10).unwrap(),
    }
}

fn print_map(size: Vec2, e: Vec2, t: i32, blizzards: &HashSet<(char, i32, i32)>) {
    print!(
        "#{}",
        if e == (Vec2 { x: 0, y: -1 }) {
            'E'
        } else {
            '.'
        }
    );
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

    // let search = dijkstra;
    let search = a_star;

    let start = Vec2 { x: 0, y: -1 };
    let goal = Vec2 {
        x: size.x - 1,
        y: size.y - 1,
    };
    let p1 = search(State::new(0, start), goal, size, &blizzards);
    // print_path(size, &p, &blizzards);
    println!("start->goal takes {} Minutes", p1.len());

    let start = Vec2 {
        x: size.x - 1,
        y: size.y,
    };
    let goal = Vec2 { x: 0, y: 0 };
    let p2 = search(State::new(p1.len() as i32, start), goal, size, &blizzards);
    println!("start<-goal takes {} Minutes", p2.len());

    let start = Vec2 { x: 0, y: -1 };
    let goal = Vec2 {
        x: size.x - 1,
        y: size.y - 1,
    };
    let p3 = search(
        State::new((p1.len() + p2.len()) as i32, start),
        goal,
        size,
        &blizzards,
    );
    println!("start->goal takes {} Minutes", p3.len());

    println!(
        "total time: {} + {} + {} = {}",
        p1.len(),
        p2.len(),
        p3.len(),
        p1.len() + p2.len() + p3.len()
    );

    Ok(())
}
