use std::io;
use std::collections::{VecDeque, HashSet, HashMap};
use std::collections::hash_map::Entry;
use std::cmp::{max, min};
use std::fmt::Display;
use std::fmt;
use std::cmp::Ordering;
use std::ops::Range;


fn consume_i(l: &str) -> Option<(u8, &str)> {
    if let Some(e) = l.find(|c: char| c != '-' && !c.is_numeric()) {
        let i = l[..e].parse().unwrap();
        Some((i, &l[e..]))
    } else {
        None
    }
}

fn consume_string<'a, 'b>(string: &'a str, l: &'b str) -> &'b str {
    assert_eq!(string, &l[..string.len()]);
    &l[string.len()..]
}

#[derive(Debug)]
struct Blueprint {
    pub ore_ore: u8,
    pub clay_ore: u8,
    pub obsidian_ore: u8,
    pub obsidian_clay: u8,
    pub geode_ore: u8,
    pub geode_obsidian: u8,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Inv {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
    r_ore: u8,
    r_clay: u8,
    r_obsidian: u8,
    r_geode: u8,
}

impl Blueprint {
    fn from_line(mut l: &str) -> Blueprint {
        l = consume_string("Blueprint ", l);
        (_, l) = consume_i(l).unwrap();
        l = consume_string(": Each ore robot costs ", l);
        let ore_ore;
        (ore_ore, l) = consume_i(l).unwrap();
        l = consume_string(" ore. Each clay robot costs ", l);
        let clay_ore;
        (clay_ore, l) = consume_i(l).unwrap();
        l = consume_string(" ore. Each obsidian robot costs ", l);
        let obsidian_ore;
        (obsidian_ore, l) = consume_i(l).unwrap();
        l = consume_string(" ore and ", l);
        let obsidian_clay;
        (obsidian_clay, l) = consume_i(l).unwrap();
        l = consume_string(" clay. Each geode robot costs ", l);
        let geode_ore;
        (geode_ore, l) = consume_i(l).unwrap();
        l = consume_string(" ore and ", l);
        let geode_obsidian;
        (geode_obsidian, l) = consume_i(l).unwrap();
        _ = consume_string(" obsidian.", l);
        Blueprint {
            ore_ore,
            clay_ore,
            obsidian_ore, 
            obsidian_clay,
            geode_ore,
            geode_obsidian,
        }
    }
    fn crack_geodes(&self, cache: &mut HashMap<(u8, Inv), u8>, time_left: u8, inv: &Inv) -> u8 {
        if time_left == 1 {
            return inv.geodes + inv.r_geode;
        }
        if let Some(b) = cache.get(&(time_left, inv.clone())) {
            return *b;
        }
        let mut ninv = inv.clone();
        ninv.ore += ninv.r_ore;
        ninv.clay += ninv.r_clay;
        ninv.obsidian += ninv.r_obsidian;
        ninv.geodes += ninv.r_geode;
        let nttl = time_left - 1;
        let mut best = 0;
        if inv.ore >= self.geode_ore && inv.obsidian >= self.geode_obsidian {
            let mut cinv = ninv.clone();
            cinv.ore -= self.geode_ore;
            cinv.obsidian -= self.geode_obsidian;
            cinv.r_geode += 1;
            best = max(best, self.crack_geodes(cache, nttl, &cinv));
        } else if inv.ore >= self.obsidian_ore && inv.clay >= self.obsidian_clay {
            let mut cinv = ninv.clone();
            cinv.ore -= self.obsidian_ore;
            cinv.clay -= self.obsidian_clay;
            cinv.r_obsidian += 1;
            best = max(best, self.crack_geodes(cache, nttl, &cinv));
        } else {
            if inv.ore >= self.clay_ore {
                let mut cinv = ninv.clone();
                cinv.ore -= self.clay_ore;
                cinv.r_clay += 1;
                best = max(best, self.crack_geodes(cache, nttl, &cinv));
            }
            if inv.ore >= self.ore_ore {
                let mut cinv = ninv.clone();
                cinv.ore -= self.ore_ore;
                cinv.r_ore += 1;
                best = max(best, self.crack_geodes(cache, nttl, &cinv));
            }
            best = max(best, self.crack_geodes(cache, nttl, &ninv));
        }
        cache.insert((time_left, inv.clone()), best);
        best
    }
}



fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let blueprints = stdin.lines().map(|l| Blueprint::from_line(&l.unwrap())).collect::<Vec<_>>();
    let inv = Inv{ r_ore: 1, ..Default::default() };
    let mut qsum = 0;
    for (i, b) in blueprints.iter().enumerate() {
        let i = i + 1;
        let mut cache = HashMap::new();
        let geodes = b.crack_geodes(&mut cache, 24, &inv);
        let q = geodes as usize * i;
        println!("Blueprint {} geodes {} quality level {}", i, geodes, q);
        qsum += q;
    }
    println!("qsum: {}", qsum);
    let mut vg = Vec::new();
    for (i, b) in blueprints.iter().enumerate().take(3) {
        let i = i + 1;
        let mut cache = HashMap::new();
        let geodes = b.crack_geodes(&mut cache, 32, &inv);
        println!("Blueprint {} geodes {}", i, geodes);
        vg.push(geodes as i32);
    }
    println!("gprod: {}", vg.iter().fold(1, |acc, x| acc * x));
    Ok(())
}
