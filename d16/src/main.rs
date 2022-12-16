use std::io;
use std::collections::{VecDeque, HashSet, HashMap};
use std::cmp::{max, min};
use std::fmt::Display;
use std::fmt;
use std::cmp::Ordering;
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


#[derive(Debug, Clone)]
struct Tunnels {
    pub valves: HashMap<String, i64>,
    pub paths: HashMap<String, Vec<String>>,
}

fn parse_line(l: &str, tunnels: &mut Tunnels) {
    let l = consume_string("Valve ", l);
    let v = l[..2].to_string();
    let l = consume_string(" has flow rate=", &l[2..]);
    let (f, l) = consume_i64(l).unwrap();
    let l = maybe_consume_string("; tunnels lead to valves ", l);
    let l = maybe_consume_string("; tunnel leads to valve ", l);
    let p = l.split(", ").map(|x| x.to_string()).collect::<Vec<_>>();
    tunnels.valves.insert(v.clone(), f);
    tunnels.paths.insert(v, p);
}


#[derive(Debug, Clone)]
struct ITunnels {
    pub valves: HashMap<u8, i64>,
    pub paths: HashMap<u8, Vec<u8>>,
}

impl ITunnels {
    fn new(tunnels: &Tunnels) -> ITunnels {
        let mut v = tunnels.valves.keys().collect::<Vec<_>>();
        v.sort();
        let v2i: HashMap<String, u8> = v.into_iter().enumerate().map(|(i, v)| (v.clone(), i as u8)).collect();
        let valves = tunnels.valves.iter().map(|(v, f)| (*v2i.get(v).unwrap(), *f)).collect();
        let paths = tunnels.paths.iter().map(|(v, p)| (
            *v2i.get(v).unwrap(),
            p.iter().map(|x| *v2i.get(x).unwrap()).collect())).collect();
        ITunnels {
            valves,
            paths,
        }
    }
    fn best_path(&self, cache: &mut HashMap<(String, u8, u8), i64>, opens: &HashSet<u8>, position: u8, time_left: u8) -> i64 {
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
            
            best = max(best, open_flow + self.best_path(cache, &nopens, position, time_left - 1));
        }
        for dest in self.paths.get(&position).unwrap() {
            best = max(best, open_flow + self.best_path(cache, opens, *dest, time_left - 1));
        }
        cache.insert((open_sig, position, time_left), best);
        best
    }
    fn best_path2(&self, cache: &mut HashMap<(u64, (u8, u8), u8), i64>, opens: u64, mut p: (u8, u8), time_left: u8, all_valves: u64) -> i64 {
        if time_left == 0 {
            return 0;
        }
        if p.0 > p.1 {
            (p.0, p.1) = (p.1, p.0);
        }
        if let Some(best) = cache.get(&(opens, p, time_left)) {
            return *best;
        }
        let open_flow: i64 = self.valves.iter().map(|(v, f)| if (opens & 1 << v) == 0 { 0 } else { *f }).sum();

        let mut best = 0;

        if all_valves == opens {
            best = open_flow * time_left as i64;
        } else {
        
            if (opens & 1 << p.0) == 0 && *self.valves.get(&p.0).unwrap() > 0 {
                let nopens = opens | 1 << p.0;
                if (opens & 1 << p.1) == 0 && *self.valves.get(&p.1).unwrap() > 0 {
                    let nnopens = nopens | 1 << p.1;
                    best = max(best, open_flow + self.best_path2(cache, nnopens, p, time_left - 1, all_valves));
                }
                for dest in self.paths.get(&p.1).unwrap() {
                    best = max(best, open_flow + self.best_path2(cache, nopens, (p.0, *dest), time_left - 1, all_valves));
                }
            }
            if (opens & 1 << p.1) == 0 && *self.valves.get(&p.1).unwrap() > 0 {
                let nopens = opens | 1 << p.1;
                for dest in self.paths.get(&p.0).unwrap() {
                    best = max(best, open_flow + self.best_path2(cache, nopens, (*dest, p.1), time_left - 1, all_valves));
                }
            }
            for d1 in self.paths.get(&p.0).unwrap() {
                for d2 in self.paths.get(&p.1).unwrap() {
                    best = max(best, open_flow + self.best_path2(cache, opens, (*d1, *d2), time_left - 1, all_valves));
                }
            }
        }
        cache.insert((opens, p, time_left), best);
        if cache.len() % 1_000_000 == 0 {
            println!("cache len: {}M", cache.len() / 1_000_000);
        }
        best
    }    
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let lines = stdin.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut t = Tunnels { valves: HashMap::new(), paths: HashMap::new() };
    for l in lines.iter() {
        parse_line(l, &mut t);
    }

    let t = ITunnels::new(&t);



    let mut cache = HashMap::new();
    println!("single {}", t.best_path(&mut cache, &HashSet::new(), 0, 30));

    let mut cache = HashMap::new();
    let all_valves: u64 = t.valves.iter().filter(|(_, v)| **v > 0).map(|(k, _)| k.clone()).fold(0_u64, |sum, x| sum | 1 << x);
    println!("double {}", t.best_path2(&mut cache, 0/*opens*/, (0, 0) /*start pos*/, 26, all_valves));

    Ok(())
}
