use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Display;
use std::io;
use std::ops::RangeInclusive;

fn consume_u(l: &str) -> Option<(usize, &str)> {
    let f = l.chars().next().unwrap();
    if !f.is_numeric() {
        None
    } else {
        let e;
        if let Some(f) = l.find(|c: char| c != '-' && !c.is_numeric()) {
            e = f;
        } else {
            e = l.len();
        }
        Some((l[..e].parse::<usize>().unwrap(), &l[e..]))
    }
}

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
    facing: usize,
}

impl Pos {
    fn l(&mut self) {
        if self.facing == 0 {
            self.facing = 3;
        } else {
            self.facing -= 1;
        }
    }
    fn r(&mut self) {
        self.facing = (self.facing + 1).rem_euclid(4);
    }
}

fn facing_pos(map: &[Vec<char>], mut p: Pos) -> Pos {
    match p.facing {
        0 => {
            if map[p.y].len() - 1 > p.x {
                p.x += 1;
            } else {
                p.x = map[p.y].iter().position(|c| *c != ' ').unwrap();
            }
        }
        2 => {
            if p.x == 0 || map[p.y][p.x - 1] == ' ' {
                p.x = map[p.y].len() - 1;
            } else {
                p.x -= 1;
            }
        }
        1 => {
            if map.len() - 1 > p.y && map[p.y + 1].len() > p.x && map[p.y + 1][p.x] != ' ' {
                p.y += 1;
            } else {
                p.y = map
                    .iter()
                    .position(|l| l.len() > p.x && l[p.x] != ' ')
                    .unwrap();
            }
        }
        3 => {
            if p.y == 0 || map[p.y - 1][p.x] == ' ' {
                p.y = map
                    .iter()
                    .rposition(|l| l.len() > p.x && l[p.x] != ' ')
                    .unwrap();
            } else {
                p.y -= 1;
            }
        }
        _ => panic!("unexpected facing: {}", p.facing),
    };
    p
}

fn cube_facing_pos(map: &[Vec<char>], mut p: Pos) -> Pos {
    let fl = 50;
    assert_eq!(map.len(), 4 * fl);
    match p.facing {
        0 => {
            if map[p.y].len() - 1 > p.x {
                p.x += 1;
            } else {
                if p.y < fl {
                    p.y = 3 * fl - 1 - p.y;
                    p.x = 2 * fl - 1;
                    p.facing = 2;
                } else if p.y < 2 * fl {
                    p.x = fl + p.y;
                    p.y = fl - 1;
                    p.facing = 3;
                } else if p.y < 3 * fl {
                    p.y = 3 * fl - p.y - 1;
                    p.x = 3 * fl - 1;
                    p.facing = 2;
                } else {
                    p.x = p.y - 2 * fl;
                    p.y = 3 * fl - 1;
                    p.facing = 3;
                }
            }
        }
        1 => {
            if map.len() - 1 > p.y && map[p.y + 1].len() > p.x && map[p.y + 1][p.x] != ' ' {
                p.y += 1;
            } else {
                if p.x < fl {
                    p.x += 2 * fl;
                    p.y = 0;
                } else if p.x < 2 * fl {
                    p.y = p.x + 2 * fl;
                    p.x = fl - 1;
                    p.facing = 2;
                } else {
                    p.y = p.x - fl;
                    p.x = 2 * fl - 1;
                    p.facing = 2;
                }
            }
        }
        2 => {
            if p.x != 0 && map[p.y][p.x - 1] != ' ' {
                p.x -= 1;
            } else {
                if p.y < fl {
                    p.y = 3 * fl - 1 - p.y;
                    p.x = 0;
                    p.facing = 0;
                } else if p.y < 2 * fl {
                    p.x = p.y - fl;
                    p.y = 2 * fl;
                    p.facing = 1;
                } else if p.y < 3 * fl {
                    p.y = 3 * fl - 1 - p.y;
                    p.x = fl;
                    p.facing = 0;
                } else {
                    p.x = p.y - 2 * fl;
                    p.y = 0;
                    p.facing = 1;
                }
            }
        }
        3 => {
            if p.y != 0 && map[p.y - 1][p.x] != ' ' {
                p.y -= 1;
            } else {
                if p.x < fl {
                    p.y = p.x + fl;
                    p.x = fl;
                    p.facing = 0;
                } else if p.x < 2 * fl {
                    p.y = p.x + 2 * fl;
                    p.x = 0;
                    p.facing = 0;
                } else {
                    p.x = p.x - 2 * fl;
                    p.y = 4 * fl - 1;
                }
            }
        }
        _ => panic!("unexpected facing: {}", p.facing),
    };
    p
}

fn draw(map: &[Vec<char>]) {
    for l in map.iter() {
        for c in l {
            print!("{}", c);
        }
        println!("");
    }
}

fn draw_pos(map: &mut [Vec<char>], pos: &Pos) {
    map[pos.y][pos.x] = match pos.facing {
        0 => '>',
        1 => 'v',
        2 => '<',
        3 => '^',
        _ => panic!("unexpected facing: {}", pos.facing),
    };
}

fn walk(map: &[Vec<char>], mut path: &str, cube: bool) -> Pos {
    let startX = map[0].iter().position(|c| *c == '.').unwrap();
    let mut pos = Pos {
        x: startX,
        y: 0,
        facing: 0,
    };

    let mut cmap = map.iter().map(|l| l.clone()).collect::<Vec<_>>();

    while !path.is_empty() {
        match &path[..1] {
            "L" => {
                pos.l();
                path = &path[1..];
                continue;
            }
            "R" => {
                pos.r();
                path = &path[1..];
                continue;
            }
            _ => {}
        }
        let n = consume_u(&path).unwrap();
        path = n.1;
        let n = n.0;
        draw_pos(&mut cmap, &pos);
        for _ in 0..n {
            let f;
            if cube {
                f = cube_facing_pos(map, pos.clone());
            } else {
                f = facing_pos(map, pos.clone());
            }
            if map[f.y][f.x] != '#' {
                pos = f;
            }
            draw_pos(&mut cmap, &pos);
        }
    }
    draw(&cmap);
    pos
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let map = lines[..lines.len() - 2]
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let path = &lines[lines.len() - 1];

    let pos = walk(&map, &path, false);
    println!(
        "p1 final {:?} Password = {}",
        pos,
        1000 * (pos.y + 1) + 4 * (pos.x + 1) + pos.facing
    );

    // Walk a bit on an empty map.
    // let map = map.iter().map(|l| l.iter().map(|c| if *c == ' ' {' '} else {'.'}).collect::<Vec<_>>()).collect::<Vec<_>>();
    // let path = "25R10L90";

    let pos = walk(&map, &path, true);
    println!(
        "p2 final {:?} Password = {}",
        pos,
        1000 * (pos.y + 1) + 4 * (pos.x + 1) + pos.facing
    );

    Ok(())
}
