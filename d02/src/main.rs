use std::io;


fn parse_line(l: &str) -> u8 {
    let l = l.chars().collect::<Vec<_>>();
    let (op, x) = (l[0] as u8 - 'A' as u8, l[2] as u8- 'X' as u8);
    let my = calc_move(op, x);
    calc_outcome(op, my) + 1 + my
}

fn calc_move(op: u8, x: u8) -> u8 {
    (op + x + 2) % 3
}

fn calc_outcome(op: u8, my: u8) -> u8 {
    (my + 4 - op) % 3 * 3
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let total: i32 = stdin.lines().map(|l| parse_line(&l.unwrap()) as i32).sum();

    println!("total: {}", total);

    Ok(())
}