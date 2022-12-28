use std::collections::HashSet;
use std::io;

#[derive(Debug, Clone)]
struct Coord {
    value: i64,
    prev: usize,
    next: usize,
}

fn mix(coords: &mut [Coord], n: usize) -> i64 {
    for _ in 0..n {
        for i in (0..coords.len()) {
            let c = coords[i].clone();
            let steps = c.value.rem_euclid((coords.len() - 1) as i64);
            if steps == 0 {
                continue;
            }
            coords[c.prev].next = c.next;
            coords[c.next].prev = c.prev;
            let mut ni = i;
            for _ in 0..=steps {
                ni = coords[ni].next;
            }
            coords[i].next = ni;
            coords[i].prev = coords[ni].prev;
            let prev = coords[ni].prev;
            coords[prev].next = i;
            coords[ni].prev = i;
            // check(&coords);
        }
    }
    let mut zero_idx = coords.iter().position(|c| c.value == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|p| {
            let mut i = zero_idx;
            for _ in 0..*p {
                i = coords[i].next;
            }
            coords[i].value
        })
        .sum()
}

fn cmix(coords: &Vec<(usize, i64)>, n: usize) -> i64 {
    let mut mix_buffer = coords.clone();
    let cycle_len = (coords.len() - 1) as i64;
    for _ in 0..n {
        for (orig_pos, number) in coords.iter() {
            let from_idx = mix_buffer
                .iter()
                .position(|x| *x == (*orig_pos, *number))
                .unwrap();
            let to_idx = (from_idx as i64 + number).rem_euclid(cycle_len);
            let t = mix_buffer.remove(from_idx);
            mix_buffer.insert(to_idx as usize, t);
        }
    }
    let zero_idx = mix_buffer.iter().position(|x| x.1 == 0).unwrap();
    return [1000, 2000, 3000]
        .iter()
        .map(|o| mix_buffer[(zero_idx + o) % mix_buffer.len()].1)
        .sum();
}

fn csolve(numbers: &[i64], n: usize) -> i64 {
    let coords: Vec<(usize, i64)> = numbers.iter().map(|i| *i).enumerate().collect::<Vec<_>>();
    cmix(&coords, n)
}

fn solve(numbers: &[i64], n: usize) -> i64 {
    let mut coords = numbers
        .iter()
        .enumerate()
        .map(|(i, n)| Coord {
            value: *n,
            prev: if i == 0 { 0 } else { (i - 1) as usize },
            next: i + 1,
        })
        .collect::<Vec<_>>();
    let num_coords = coords.len();
    coords[0].prev = num_coords - 1;
    coords[num_coords - 1].next = 0;
    let coords = coords;

    let mut coords1 = coords.clone();
    return mix(&mut coords1, n);
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let numbers = stdin
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("p1: {}", solve(&numbers, 1));

    let numbers = numbers.iter().map(|n| n * 811589153).collect::<Vec<_>>();
    println!("p2: {}", solve(&numbers, 10));

    Ok(())
}
