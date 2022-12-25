use std::io;

fn snafu2dec(l: &str) -> u64 {
    let mut res: i64 = 0;
    for c in l.chars() {
        res *= 5;
        res += match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("unexpected digit {}", c),
        }
    }
    res as u64
}

fn dec2snafu(mut num: u64) -> String {
    let mut res = Vec::new();
    while num > 0 {
        let digit = (num % 5) as u32;
        if digit < 3 {
            res.push(std::char::from_digit(digit, 10).unwrap());
        } else {
            if digit == 3 {
                res.push('=');
                num += 2;
            } else {
                res.push('-');
                num += 1;
            }
        }
        num /= 5;
    }
    res.reverse();
    res.into_iter().collect()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut s = 0;
    for l in stdin.lines().map(|l| l.unwrap()) {
        let d = snafu2dec(&l);
        s += d;
        println!("{:10} {:5}", l, d);
    }

    println!("sum = {}  snafu sum = {}", s, dec2snafu(s));

    Ok(())
}
