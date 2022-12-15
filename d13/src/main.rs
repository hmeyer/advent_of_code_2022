use std::io;
use std::collections::{VecDeque, HashSet};
use std::cmp::min;
use std::fmt::Display;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    I(i32),
    V(Vec<Packet>),
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::I(i) => write!(f, "{}", i),
            Packet::V(l) => write!(f, "[{}]", l.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(",")),
        }
    }
}

fn cmp_v(la: &[Packet], lb: &[Packet]) -> Ordering {
    for i in 0..min(la.len(), lb.len()) {
        let o = cmp(&la[i], &lb[i]);
        if o != Ordering::Equal {
            return o;
        }
    }
    if la.len() < lb.len() {
        return Ordering::Less;
    }
    if la.len() > lb.len() {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}


fn cmp(a: &Packet, b: &Packet) -> Ordering {
    let r = match (a, b) {
        (Packet::I(ia), Packet::I(ib)) => ia.cmp(ib),
        (Packet::V(la), Packet::V(lb)) => cmp_v(la, lb),
        (Packet::I(ia), Packet::V(lb)) => cmp(&Packet::V(vec![a.clone()]), b),
        (Packet::V(la), Packet::I(ib)) => cmp(a, &Packet::V(vec![b.clone()])),
    };
    r
}

fn parse_list(mut l: &str) -> (Packet, &str) {
    assert_eq!(l[0..1], *"[");
    l = &l[1..];
    let mut res = Vec::new();
    loop {
        if l[0..1] == *"[" {
            let (pr, rest) = parse_list(l);
            res.push(pr);
            l = rest;
        } else if l[0..1] == *"]" {
            l = &l[1..];
            if l.len() > 0 && l[0..1] == *"," {
                l = &l[1..];
            }
            break;
        } else {
            let e = l.find(&[']',',']).unwrap();
            assert!(e >= 1, "{} < 1", e);
            let i = l[..e].parse().unwrap();
            res.push(Packet::I(i));

            let comma = l[e..e+1] == *",";
            l = &l[e..];
            if comma {
                l = &l[1..];
            }
        }
    }
    (Packet::V(res), l)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut i = 0;
    let mut pi = 1;
    let mut sum = 0;
    let mut packets = Vec::new();
    while i < lines.len() {
        let l1 = parse_list(&lines[i]).0;
        let l2 = parse_list(&lines[i+1]).0;
        i += 3;
        // println!("{:?}", l1);
        // println!("{:?}", l2);
        let c = cmp(&l1, &l2);
        println!("{} {:?}", pi, c);
        if c != Ordering::Greater {
            sum += pi;
        }
        pi += 1;
        packets.push(l1);
        packets.push(l2);
    }
    println!("sum of indices of ordered pairs: {}", sum);
    let d0 = Packet::V(vec![Packet::V(vec![Packet::I(2)])]);
    let d1 = Packet::V(vec![Packet::V(vec![Packet::I(6)])]);
    packets.push(d0.clone());
    packets.push(d1.clone());
    packets.sort_by(|a, b| cmp(a,b));
    let d0i = packets.iter().enumerate().find(|(_, p)| **p == d0).map(|(i, _)| i).unwrap() + 1;
    let d1i = packets.iter().enumerate().find(|(_, p)| **p == d1).map(|(i, _)| i).unwrap() + 1;
    println!("dividers at {} and {}: prod: {}", d0i, d1i, d0i * d1i);

    Ok(())
}
