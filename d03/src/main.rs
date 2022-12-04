use std::io;
use std::collections::HashSet;


fn char_to_prio(c: char) -> usize {
    match c {
        x if x >= 'a' && x <= 'z' => x as usize - 'a' as usize + 1,
        x if x >= 'A' && x <= 'Z' => x as usize - 'A' as usize + 27,
        _ => panic!("wow"),
    }
}


fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let lines = stdin.lines().map(|l| l.unwrap().chars().collect::<HashSet<_>>()).collect::<Vec<_>>();
    assert!(lines.len() % 3 == 0);
    let mut sum = 0;
    for n in (0..lines.len()).step_by(3) {
        let m = lines[n].intersection(&lines[n+1]).map(|c| *c).collect::<HashSet<_>>();
        let m = m.intersection(&lines[n+2]).collect::<Vec<_>>();
        assert!(m.len() == 1);
        sum += char_to_prio(*m[0]);
    }

    println!("sum: {}", sum);

    Ok(())
}