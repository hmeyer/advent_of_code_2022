use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    
    let mut sums = Vec::new();
    let mut sum = 0;

    for line in stdin.lines() {
        let l = line.unwrap();
        match l.parse::<i32>() {
            Ok(n) => sum += n,
            Err(_) => {
            sums.push(sum);
            sum = 0;
           }
        }
    }
    sums.sort();

    println!("max calories: {}", sums[sums.len()-1]);

    let top3: i32 = sums[sums.len()-3..].iter().sum();
    println!("top3 calories: {}", top3);

    Ok(())
}