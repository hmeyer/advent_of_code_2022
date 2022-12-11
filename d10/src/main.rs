use std::io;
use std::collections::{HashSet};
use std::cmp::max;

#[derive(Debug)]
enum Cmd {
    Addx(i32),
    Noop,
}

fn parse_line(l: &str) -> Cmd {
    if l == "noop" {
        return Cmd::Noop;
    }
    assert!(l.starts_with("addx "));
    let a = l[5..].parse::<i32>().unwrap();
    Cmd::Addx(a)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut x = 1;
    let mut x_log = Vec::new();


    for l in stdin.lines() {
        let cmd = parse_line(&l.unwrap());
        match cmd {
            Cmd::Noop => x_log.push(x),
            Cmd::Addx(a) => {
                x_log.push(x);
                x_log.push(x);
                x += a;
            }
        }
    }
    println!("signal strength: {:?}", x_log.iter().enumerate().skip(19).step_by(40).map(|(c, x)| (c + 1) as i32 * x).sum::<i32>());
    for y in 0..6 {
        for x in 0..40 {
            let xv = x_log[x + y * 40];
            if (xv - x as i32).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    Ok(())
}
