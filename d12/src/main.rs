use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::io;

fn find_starts(m: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut r = Vec::new();
    for (y, l) in m.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == 'a' || *c == 'S' {
                r.push((y as i32, x as i32));
            }
        }
    }
    r
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let map = stdin
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", map);

    let mut q = VecDeque::new();
    let mut v = HashSet::new();
    for p in find_starts(&map) {
        q.push_back((p, 0));
    }

    while !q.is_empty() {
        let (p, steps) = q.pop_front().unwrap();
        if !v.insert(p) {
            continue;
        }
        let mut h = map[p.0 as usize][p.1 as usize];
        if h == 'E' {
            println!("took {} steps", steps);
            break;
        }
        let steps = steps + 1;
        if h == 'S' {
            h = 'a';
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let np = (p.0 + dx, p.1 + dy);
            if np.0 < 0 || np.0 >= map.len() as i32 || np.1 < 0 || np.1 >= map[0].len() as i32 {
                continue;
            }
            let mut dh = map[np.0 as usize][np.1 as usize];
            if dh == 'E' {
                dh = 'z';
            }
            if dh as i32 - h as i32 > 1 {
                continue;
            }
            q.push_back((np, steps));
        }
    }
    Ok(())
}
