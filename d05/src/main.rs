use std::collections::HashSet;
use std::io;
use std::ops::RangeInclusive;

fn parse_crates(l: &str) -> Vec<char> {
    let mut r = Vec::new();
    let mut c = l.chars();
    loop {
        if c.next() == None {
            break;
        }
        r.push(c.next().unwrap());
        c.next();
        c.next();
    }
    r
}

#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_move(l: &str) -> Move {
    let parts = l.split(" ").collect::<Vec<_>>();
    assert!(parts[0] == "move");
    assert!(parts[2] == "from");
    assert!(parts[4] == "to");
    Move {
        num: parts[1].parse::<usize>().unwrap(),
        from: parts[3].parse::<usize>().unwrap(),
        to: parts[5].parse::<usize>().unwrap(),
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let mut stacks = Vec::new();

    loop {
        let l = lines.next().unwrap().unwrap();
        if l.starts_with(" 1 ") {
            break;
        }
        let crates = parse_crates(&l);
        if stacks.len() == 0 {
            // first iteration.
            for _ in 0..crates.len() {
                stacks.push(Vec::new());
            }
        }
        for (i, c) in crates.into_iter().enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    println!("{:?}", stacks);
    for i in 0..stacks.len() {
        stacks[i].reverse();
    }
    println!("{:?}", stacks);
    lines.next();
    for l in lines {
        let l = l.unwrap();
        let m = parse_move(&l);
        println!("{:?}", m);
        let mut crane = Vec::new();
        for _ in 0..m.num {
            let c = stacks[m.from - 1].pop().unwrap();
            crane.push(c);
        }
        crane.reverse();
        for c in crane {
            stacks[m.to - 1].push(c);
        }
        println!("{:?}", stacks);
    }
    for mut s in stacks {
        print!("{}", s.pop().unwrap());
    }
    println!();

    Ok(())
}
