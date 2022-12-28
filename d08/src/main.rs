use std::cmp::max;
use std::collections::HashSet;
use std::io;

fn parse_line(l: &str) -> Vec<i8> {
    l.chars().map(|c| c as i8 - '0' as i8).collect()
}

fn get_vismap(grid: &Vec<Vec<i8>>) -> HashSet<(usize, usize)> {
    let h = grid.len();
    let w = grid[0].len();
    let mut v = HashSet::new();
    for y in 0..h {
        let mut t = -1;
        for x in 0..w {
            if grid[y][x] > t {
                v.insert((y, x));
            }
            t = max(t, grid[y][x]);
        }
        t = -1;
        for x in (0..w).rev() {
            if grid[y][x] > t {
                v.insert((y, x));
            }
            t = max(t, grid[y][x]);
        }
    }

    for x in 0..w {
        let mut t = -1;
        for y in 0..h {
            if grid[y][x] > t {
                v.insert((y, x));
            }
            t = max(t, grid[y][x]);
        }
        t = -1;
        for y in (0..h).rev() {
            if grid[y][x] > t {
                v.insert((y, x));
            }
            t = max(t, grid[y][x]);
        }
    }
    v
}

fn get_scenic_score(grid: &Vec<Vec<i8>>) -> i32 {
    let h = grid.len();
    let w = grid[0].len();
    let mut m = 0;

    for y in 0..h {
        for x in 0..w {
            let mut score = 1;
            let mut ascore = 0;
            for ax in (0..x).rev() {
                ascore += 1;
                if grid[y][ax] >= grid[y][x] {
                    break;
                }
            }
            score *= ascore;

            ascore = 0;
            for ax in x + 1..w {
                ascore += 1;
                if grid[y][ax] >= grid[y][x] {
                    break;
                }
            }
            score *= ascore;

            ascore = 0;
            for ay in (0..y).rev() {
                ascore += 1;
                if grid[ay][x] >= grid[y][x] {
                    break;
                }
            }
            score *= ascore;

            ascore = 0;
            for ay in y + 1..h {
                ascore += 1;
                if grid[ay][x] >= grid[y][x] {
                    break;
                }
            }
            score *= ascore;
            m = max(m, score);
        }
    }
    m
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let grid = stdin
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect::<Vec<_>>();
    let v = get_vismap(&grid);

    println!("num_visible: {}", v.len());
    println!("scenic_score: {}", get_scenic_score(&grid));

    Ok(())
}
