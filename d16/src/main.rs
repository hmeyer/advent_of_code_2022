use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Display;
use std::io;
use std::ops::RangeInclusive;

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

#[derive(Debug, Clone)]
struct ITunnels {
    pub valves: HashMap<u8, i64>,
    pub paths: HashMap<u8, Vec<u8>>,
}

impl ITunnels {
    fn new(tunnels: &HashMap<&str, (i64, Vec<&str>)>) -> ITunnels {
        let mut v = tunnels.keys().collect::<Vec<_>>();
        v.sort();
        let v2i: HashMap<&str, u8> = v
            .into_iter()
            .enumerate()
            .map(|(i, v)| (*v, i as u8))
            .collect();
        let valves = tunnels
            .iter()
            .map(|(v, flow_and_paths)| (*v2i.get(v).unwrap(), flow_and_paths.0))
            .collect();
        let paths = tunnels
            .iter()
            .map(|(v, flow_and_paths)| (
                    *v2i.get(v).unwrap(),
                    flow_and_paths.1.iter().map(|x| *v2i.get(x).unwrap()).collect(),
                )).collect();
        ITunnels { valves, paths }
    }
    fn best_path(
        &self,
        cache: &mut HashMap<(String, u8, u8), i64>,
        opens: &HashSet<u8>,
        position: u8,
        time_left: u8,
    ) -> i64 {
        let mut open_sig = opens.iter().collect::<Vec<_>>();
        open_sig.sort();
        let open_sig = format!("{:?}", open_sig);
        if let Some(best) = cache.get(&(open_sig.clone(), position, time_left)) {
            return *best;
        }
        if time_left == 0 {
            return 0;
        }
        let open_flow: i64 = opens.iter().map(|p| *self.valves.get(p).unwrap()).sum();
        let mut best = 0;
        if opens.get(&position).is_none() && *self.valves.get(&position).unwrap() > 0 {
            let mut nopens = opens.clone();
            nopens.insert(position);

            best = max(
                best,
                open_flow + self.best_path(cache, &nopens, position, time_left - 1),
            );
        }
        for dest in self.paths.get(&position).unwrap() {
            best = max(
                best,
                open_flow + self.best_path(cache, opens, *dest, time_left - 1),
            );
        }
        cache.insert((open_sig, position, time_left), best);
        best
    }
    fn best_path2(
        &self,
        cache: &mut HashMap<(u64, (u8, u8), u8), i64>,
        opens: u64,
        mut trail: (&mut Vec<u8>, &mut Vec<u8>),
        time_left: u8,
        all_valves: u64,
    ) -> i64 {
        if time_left == 0 {
            return 0;
        }
        let p = (trail.0[trail.0.len() - 1], trail.1[trail.1.len() - 1]);
        let ps = if p.0 < p.1 { (p.0, p.1) } else { (p.1, p.0) };
        if let Some(best) = cache.get(&(opens, ps, time_left)) {
            return *best;
        }
        let open_flow: i64 = self
            .valves
            .iter()
            .map(|(v, f)| if (opens & 1 << v) == 0 { 0 } else { *f })
            .sum();

        let mut best = 0;

        if all_valves == opens {
            best = open_flow * time_left as i64;
        } else {
            if (opens & 1 << p.0) == 0 && *self.valves.get(&p.0).unwrap() > 0 {
                let nopens = opens | 1 << p.0;
                if (opens & 1 << p.1) == 0 && *self.valves.get(&p.1).unwrap() > 0 {
                    let nnopens = nopens | 1 << p.1;
                    best = max(
                        best,
                        open_flow
                            + self.best_path2(
                                cache,
                                nnopens,
                                (&mut vec![p.0], &mut vec![p.1]),
                                time_left - 1,
                                all_valves,
                            ),
                    );
                }
                for dest in self.paths.get(&p.1).unwrap() {
                    if trail.1.iter().position(|x| x == dest).is_some() {
                        continue;
                    }
                    trail.1.push(*dest);
                    best = max(
                        best,
                        open_flow
                            + self.best_path2(
                                cache,
                                nopens,
                                (&mut vec![p.0], trail.1),
                                time_left - 1,
                                all_valves,
                            ),
                    );
                    trail.1.pop();
                }
            }
            if (opens & 1 << p.1) == 0 && *self.valves.get(&p.1).unwrap() > 0 {
                let nopens = opens | 1 << p.1;
                for dest in self.paths.get(&p.0).unwrap() {
                    if trail.0.iter().position(|x| x == dest).is_some() {
                        continue;
                    }
                    trail.0.push(*dest);
                    best = max(
                        best,
                        open_flow
                            + self.best_path2(
                                cache,
                                nopens,
                                (trail.0, &mut vec![p.1]),
                                time_left - 1,
                                all_valves,
                            ),
                    );
                    trail.0.pop();
                }
            }
            for d0 in self.paths.get(&p.0).unwrap() {
                if trail.0.iter().position(|x| x == d0).is_some() {
                    continue;
                }
                trail.0.push(*d0);
                for d1 in self.paths.get(&p.1).unwrap() {
                    if trail.1.iter().position(|x| x == d1).is_some() {
                        continue;
                    }
                    trail.1.push(*d1);
                    best = max(
                        best,
                        open_flow
                            + self.best_path2(
                                cache,
                                opens,
                                (&mut trail.0, &mut trail.1),
                                time_left - 1,
                                all_valves,
                            ),
                    );
                    trail.1.pop();
                }
                trail.0.pop();
            }
        }
        cache.insert((opens, ps, time_left), best);
        if cache.len() % 1_000_000 == 0 {
            println!("cache len: {}M", cache.len() / 1_000_000);
        }
        best
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let lines = stdin.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut t = HashMap::new();
    for l in lines.iter() {
        let (room, flow_and_paths) = parse_line(l);
        t.insert(room, flow_and_paths);
    }

    let t = ITunnels::new(&t);

    let mut cache = HashMap::new();
    println!("single {}", t.best_path(&mut cache, &HashSet::new(), 0, 30));

    let mut cache = HashMap::new();
    let all_valves: u64 = t
        .valves
        .iter()
        .filter(|(_, v)| **v > 0)
        .map(|(k, _)| k.clone())
        .fold(0_u64, |sum, x| sum | 1 << x);
    println!(
        "double {}",
        t.best_path2(
            &mut cache,
            0,                            /*opens*/
            (&mut vec![0], &mut vec![0]), /*start pos*/
            26,
            all_valves
        )
    );

    Ok(())
}
