use std::io;
use std::collections::HashSet;
use std::ops::RangeInclusive;


fn parse_line(l: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut ranges = l.split(",").map(|s| parse_range(s));
    let result = ranges.next().unwrap();
    let result = (result, ranges.next().unwrap());
    assert!(ranges.next().is_none());
    result
}

fn parse_range(s: &str) -> RangeInclusive<i32> {
    let mut num = s.split("-").map(|n| n.parse::<i32>().unwrap());
    let start = num.next().unwrap();
    let end = num.next().unwrap();
    assert!(num.next().is_none());
    start..=end
}

fn fully_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    (a.contains(b.start()) && a.contains(b.end())) || (b.contains(a.start()) && b.contains(a.end()))
}

fn some_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let ranges = stdin.lines().map(|l| parse_line(&l.unwrap())).collect::<Vec<_>>();
    let full = ranges.iter().filter(|(a, b)| fully_overlap(a, b)).count();
    let some = ranges.iter().filter(|(a, b)| some_overlap(a, b)).count();

    println!("full: {:?}", full);
    println!("some: {:?}", some);

    Ok(())
}