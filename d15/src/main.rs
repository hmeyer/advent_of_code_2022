use std::io;
use std::collections::{VecDeque, HashSet};
use std::cmp::{max, min};
use std::fmt::Display;
use std::fmt;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

fn parse_line(mut l: &str) -> ((i64, i64), (i64, i64)) {
    l = &l[12..];
    let e = l.find(|c: char| c != '-' && !c.is_numeric()).unwrap();
    let sx = l[..e].parse::<i64>().unwrap();
    l = &l[e+4..];
    let e = l.find(|c: char| c != '-' && !c.is_numeric()).unwrap();
    let sy = l[..e].parse::<i64>().unwrap();
    l = &l[e+25..];
    let e = l.find(|c: char| c != '-' && !c.is_numeric()).unwrap();
    let bx = l[..e].parse::<i64>().unwrap();
    l = &l[e+4..];
    let by = l.parse::<i64>().unwrap();
    ((sx, sy), (bx, by))
}

fn count_free(row: i64, sensors: &[((i64, i64), i64)], beacons: &[(i64, i64)]) -> usize {
    let mut free = HashSet::new();
    for (s, d) in sensors.iter() {
        let d = d - (s.1 - row).abs();
        if d >= 0 {
            for x in s.0 - d..=s.0 + d {
                free.insert(x);
            }
        }
    }
    for b in beacons.iter() {
        if b.1 == row {
            free.take(&b.0);
        }
    }
    free.len()
}

fn empty_spot(row: i64, sensors: &[((i64, i64), i64)], r: RangeInclusive<i64>) -> Option<i64> {
    let mut ranges = Vec::new();
    ranges.push(r);

    for (s, d) in sensors.iter() {
        let d = d - (s.1 - row).abs();
        if d >= 0 {
            let punch = s.0 - d..=s.0 + d;
            let mut nranges: Vec<RangeInclusive<i64>> = Vec::new();
            for r in ranges {
                let upper = *r.start()..=min(*r.end(), punch.start() - 1);
                if !upper.is_empty() {
                    nranges.push(upper);
                }

                let lower = max(punch.end() + 1, *r.start())..=*r.end();
                if !lower.is_empty() {
                    nranges.push(lower);
                }

            }
            ranges = nranges;
        }
    }
    if ranges.len() == 0 {
        return None;
    }
    assert!(ranges.len() == 1);
    assert_eq!(ranges[0].start(), ranges[0].end());
    Some(*ranges[0].start())
}

fn find_empty(sensors: &[((i64, i64), i64)], r: RangeInclusive<i64>) -> (i64, i64) {
    let mut e = Vec::new();
    for y in r.clone() {
        if let Some(x) = empty_spot(y, sensors, r.clone()) {
            e.push((x, y));
        }
    }
    assert_eq!(1, e.len());
    return e[0];
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut lines = stdin.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for l in lines {
        let (s, b) = parse_line(&l);
        let d = (s.0 - b.0).abs() + (s.1 - b.1).abs();
        sensors.push((s, d));
        beacons.push(b);
    }

    let row = 10;
    println!("row {}: {}", row, count_free(row, &sensors, &beacons));
    let row = 2000000;
    println!("row {}: {}", row, count_free(row, &sensors, &beacons));

    let r = 0..=4000000;
    let e = find_empty(&sensors, r.clone());
    println!("empty_spot in {:?}: {:?} freq: {}", r, e, e.0 * 4000000 + e.1);

    Ok(())
}
