use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;

fn consume_i64(l: &str) -> Option<(i64, &str)> {
    if let Some(e) = l.find(|c: char| c != '-' && !c.is_numeric()) {
        let i = l[..e].parse::<i64>().unwrap();
        Some((i, &l[e..]))
    } else {
        None
    }
}

fn consume_string<'a, 'b>(string: &'a str, l: &'b str) -> &'b str {
    assert_eq!(string, &l[..string.len()]);
    &l[string.len()..]
}

fn maybe_consume_string<'a, 'b>(string: &'a str, l: &'b str) -> &'b str {
    if l.starts_with(string) {
        &l[string.len()..]
    } else {
        l
    }
}

fn parse_line<'a>(l: &'a str) -> (&'a str, (i64, Vec<&'a str>)) {
    let l = consume_string("Valve ", l);
    let v = &l[..2];
    let l = consume_string(" has flow rate=", &l[2..]);
    let (f, l) = consume_i64(l).unwrap();
    let l = maybe_consume_string("; tunnels lead to valves ", l);
    let l = maybe_consume_string("; tunnel leads to valve ", l);
    let p = l.split(", ").collect::<Vec<_>>();
    (v, (f, p))
}

fn compute_distances<'a>(tunnels: &HashMap<&'a str, (i64, Vec<&'a str>)>) -> HashMap<(&'a str, &'a str), i64> {
    // Using Floyd's algorithm to compute shortest distances between any nodes.
    let mut d: HashMap<(&'a str, &'a str), i64> = tunnels.iter().map(|(a, (_, bs))| bs.iter().map(|b| ((*a, *b), 1))).flatten().collect();
    for mid in tunnels.keys() {
        for from in tunnels.keys() {
            if let Some(from_d) = d.get(&(from, mid)).cloned() {
                for to in tunnels.keys() {
                    if let Some(to_d) = d.get(&(mid, to)).cloned() {
                        let total_d = d.entry((from, to)).or_insert(i64::MAX);
                        *total_d = min(from_d + to_d, *total_d);
                    }
                }
            }
        }
    }
    d
}

fn best_path<'a>(cache: &mut HashMap<(&'a str, i64, Vec<&'a str>, bool), i64>, pos: &'a str, time_left: i64, distances: &HashMap<(&str, &str), i64>, flows: &HashMap<&'a str, i64>, with_elephant: bool) -> i64 {
    let mut flow_rooms = flows.iter().map(|(r, _)| *r).collect::<Vec<_>>();
    flow_rooms.sort();
    if let Some(c) = cache.get(&(pos, time_left, flow_rooms.clone(), with_elephant)) {
        return *c;
    }
    let result = max(flows.iter().map(|(t, f)| {
            let mut fc = flows.clone();
            fc.remove(t);
            let tl = time_left - distances.get(&(pos, t)).unwrap() - 1;
            if tl > 0 {
                tl * f + best_path(cache, t, tl, distances, &fc, with_elephant)
            } else {
                0
            }
        }).max().unwrap_or(0),
        // If the elephant is allowed, see if the elefant can do with the remaining rooms.
        if with_elephant { best_path(cache, "AA", 26, distances, flows, false) } else { 0 });
    cache.insert((pos, time_left, flow_rooms, with_elephant), result);
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let lines = stdin.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut t = HashMap::new();
    for l in lines.iter() {
        let (room, flow_and_paths) = parse_line(l);
        t.insert(room, flow_and_paths);
    }
    let d = compute_distances(&t);
    let relevant_rooms = t.iter().filter(|(_, (f, _))| *f > 0).map(|(a, (f, _))| (*a, *f)).collect::<HashMap<_, _>>();
    println!("single {}", best_path(&mut HashMap::new(), "AA", 30, &d, &relevant_rooms, false));
    println!("double {}", best_path(&mut HashMap::new(), "AA", 26, &d, &relevant_rooms, true));

    Ok(())
}
