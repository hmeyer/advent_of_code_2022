use std::io;
use std::collections::{HashSet};
use std::cmp::max;


fn parse_line(l: &str) -> (char, i32) {
    let d = l.chars().next().unwrap();
    let s = l[2..].parse::<i32>().unwrap();
    (d, s)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut rope = Vec::new();
    rope.resize(10, (0_i32, 0_i32));
    let mut short = HashSet::new();
    let mut long = HashSet::new();

    for l in stdin.lines() {
        let (d, s) = parse_line(&l.unwrap());
        println!("{} {}", d, s);
        assert!(s >= 0);
        for _ in 0..s {
            match d {
                'R' => rope[0].1 += 1,
                'L' => rope[0].1 -= 1,
                'U' => rope[0].0 -= 1,
                'D' => rope[0].0 += 1,
                x => panic!("expected direction: {}", x)
            }
            for i in 1..rope.len() {
              let d = (rope[i-1].0 - rope[i].0, rope[i-1].1 - rope[i].1);
              if d.0.abs() > 1 || d.1.abs() > 1 {
                  rope[i].0 += d.0.signum();
                  rope[i].1 += d.1.signum();
              }
            }
            short.insert(rope[1]);
            long.insert(rope[rope.len() - 1]);
        }
    }
    println!("short visited {} positions", short.len());
    println!("long visited {} positions", long.len());
    Ok(())
}
