use std::collections::{HashSet, VecDeque};
use std::io;
use std::ops::RangeInclusive;

fn parse_line(l: &str) -> usize {
    let mut b = VecDeque::new();
    const L: usize = 14;

    for (i, c) in l.chars().enumerate() {
        if i >= L {
            b.pop_front();
        }
        b.push_back(c);
        let mut s = HashSet::new();
        for x in b.iter() {
            s.insert(x);
        }
        if s.len() == L {
            return i + 1;
        }
    }
    panic!("damn!");
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for l in stdin.lines() {
        println!("{}", parse_line(&l.unwrap()));
    }
    Ok(())
}
