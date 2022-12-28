use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Display;
use std::io;
use std::ops::RangeInclusive;

struct Cave {
    lines: Vec<u8>,
    jet_pattern: Vec<i8>,
    pattern_pos: usize,
    pub block_index: usize,
    pub full_line: Option<(usize, usize)>,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for l in self.lines.iter().rev() {
            write!(f, "|")?;
            for x in (0..7).rev() {
                write!(f, "{}", if l & 1 << x != 0 { '#' } else { '.' })?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+\nheight: {}", self.height())
    }
}

type Block = [u8; 4];

static BLOCKS: [Block; 5] = [
    [0b00000000, 0b00000000, 0b00000000, 0b00011110],
    [0b00000000, 0b00001000, 0b00011100, 0b00001000],
    [0b00000000, 0b00000100, 0b00000100, 0b00011100],
    [0b00010000, 0b00010000, 0b00010000, 0b00010000],
    [0b00000000, 0b00000000, 0b00011000, 0b00011000],
];

impl Cave {
    fn jet(&mut self) -> i8 {
        let j = self.jet_pattern[self.pattern_pos % self.jet_pattern.len()];
        self.pattern_pos += 1;
        j
    }

    fn height(&self) -> usize {
        if let Some(p) = self.lines.iter().rposition(|l| *l != 0) {
            p + 1
        } else {
            0
        }
    }

    fn block_overlaps(&self, block: &Block, block_pos: usize) -> bool {
        for by in 0..block.len() {
            let li = block_pos + block.len() - by - 1;
            if li < self.lines.len() {
                if block[by] & self.lines[li] != 0 {
                    return true;
                }
            }
        }
        false
    }

    fn settle_block(&mut self, block: &Block, block_pos: usize) {
        for by in (0..block.len()).rev() {
            let li = block_pos + block.len() - by - 1;
            if li >= self.lines.len() {
                self.lines.push(0);
            }
            self.lines[li] |= block[by];
            if self.lines[li] == 0b01111111 {
                self.full_line = Some((
                    self.block_index % BLOCKS.len(),
                    self.pattern_pos % self.jet_pattern.len(),
                ));
            }
        }
    }

    fn next_block(&mut self) -> Block {
        let block = BLOCKS[self.block_index % BLOCKS.len()];
        self.block_index += 1;
        block
    }

    fn drop_next_block(&mut self) {
        let mut block = self.next_block();
        let mut block_pos = self.height() + 3;

        loop {
            let j = self.jet();
            let mut bc = block.clone();
            let mut could_move = true;
            for x in 0..bc.len() {
                match j {
                    -1 => {
                        if bc[x] & 1 == 1 {
                            could_move = false;
                            break;
                        }
                        bc[x] >>= 1;
                    }
                    1 => {
                        if bc[x] & 0b01000000 == 0b01000000 {
                            could_move = false;
                            break;
                        }
                        bc[x] <<= 1;
                    }
                    u => panic!("Undefined jet: {}", u),
                }
            }
            if could_move && self.block_overlaps(&bc, block_pos) {
                could_move = false;
            }
            if !could_move {
                bc = block.clone();
            }

            block = bc.clone();
            if block_pos == 0 {
                break;
            }
            block_pos -= 1;
            if self.block_overlaps(&bc, block_pos) {
                block_pos += 1;
                break;
            }
        }
        self.settle_block(&block, block_pos);
    }
}

fn main() -> io::Result<()> {
    let pattern = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '>' => -1,
            '<' => 1,
            x => panic!("unexpected char {}", x),
        })
        .collect::<Vec<_>>();

    let mut c = Cave {
        lines: vec![],
        jet_pattern: pattern.clone(),
        pattern_pos: 0,
        block_index: 0,
        full_line: None,
    };
    for _ in 0..2022 {
        c.drop_next_block();
    }
    println!("height: {}", c.height());

    let mut c = Cave {
        lines: vec![],
        jet_pattern: pattern,
        pattern_pos: 0,
        block_index: 0,
        full_line: None,
    };
    let mut full_lines: HashMap<(usize, usize), usize> = HashMap::new();
    let mut block_count: usize = 0;
    let total: usize = 1000000000000;
    loop {
        c.drop_next_block();
        block_count += 1;
        if let Some(f) = c.full_line {
            match full_lines.entry(f) {
                Entry::Vacant(o) => {
                    o.insert(block_count);
                }
                Entry::Occupied(o) => {
                    let cycle_length = block_count - o.get();
                    let h_start = c.height();
                    for _ in 0..cycle_length {
                        c.drop_next_block();
                    }
                    let h_cycle = c.height() - h_start;

                    let num_cycles = (total - c.block_index) / cycle_length;
                    let remainder = (total - c.block_index) % cycle_length;
                    let h_start = c.height();
                    for _ in 0..remainder {
                        c.drop_next_block();
                    }
                    let remainder_height = c.height() - h_start;
                    let total_h = h_start + num_cycles * h_cycle + remainder_height;

                    println!("total_h: {}", total_h);
                    break;
                }
            }
            c.full_line = None;
        }
    }
    Ok(())
}
