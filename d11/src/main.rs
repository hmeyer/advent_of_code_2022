use std::io;
use std::collections::{VecDeque};
use std::cmp::max;

#[derive(Debug)]
enum Op {
    Mul(u64),
    Add(u64),
    Square,
}


#[derive(Debug)]
struct Monkey {
    pub inspections: usize,
    pub items: VecDeque<u64>,
    pub op: Op,
    pub test_div: u64,
    pub target_true: usize,
    pub target_false: usize,
}

fn parse_monkey(lines: &mut dyn Iterator<Item=String>) -> Option<Monkey> {
    if lines.next().is_none() {
        return None;
    }
    let starting = lines.next().unwrap();
    assert!(starting.starts_with("  Starting items: "));
    let items = starting[18..].split(", ").map(|i| i.parse::<u32>().unwrap().into()).collect();

    let op = lines.next().unwrap();
    assert!(op.starts_with("  Operation: new = old "));
    let operator = op.chars().nth(23).unwrap();
    let operand = &op[25..];
    let mut op = Op::Square;
    if operand == "old" {
        assert!(operator == '*');
    } else {
        let operand = operand.parse::<u32>().unwrap().into();
        match operator {
            '+' => op = Op::Add(operand),
            '*' => op = Op::Mul(operand),
            x => panic!("unexpected operator: {}", x),
        }
    }


    let test_div = lines.next().unwrap();
    assert!(test_div.starts_with("  Test: divisible by "));
    let test_div = test_div[21..].parse::<u32>().unwrap().into();

    let target_true = lines.next().unwrap();
    assert!(target_true.starts_with("    If true: throw to monkey "));
    let target_true = target_true[29..].parse::<usize>().unwrap();
    
    let target_false = lines.next().unwrap();
    assert!(target_false.starts_with("    If false: throw to monkey "));
    let target_false = target_false[30..].parse::<usize>().unwrap();

    Some(Monkey{
        inspections: 0,
        items,
        op,
        test_div,
        target_true,
        target_false
    })
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lines().map(|l| l.unwrap());

    let mut monkeys = Vec::new();
    loop {
        if let Some(m) = parse_monkey(&mut lines) {
            monkeys.push(m);
            lines.next();
        } else {
            break;
        }
    }

    let all_divs = monkeys.iter().map(|m| m.test_div).fold(1, |prod, i| prod * i);
    println!("all_divs: {:?}", all_divs);

    for round in 1..=10000 {
        for m in 0..monkeys.len() {
            // println!("  Mokey {}:", m);
            while !monkeys[m].items.is_empty() {
                monkeys[m].inspections += 1;
                let mut i = monkeys[m].items.pop_front().unwrap();
                // println!("    Mokey inspects an item with worry level of {}.", i);
                match monkeys[m].op {
                    Op::Add(x) => i += x,
                    Op::Mul(x) => i *= x,
                    Op::Square => i *= i,
                }
                // println!("    New worry level after {:?} is {}.", monkeys[m].op, i);
                // i /= 3;
                // println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", i);
                i = i % all_divs;
                let mut target = monkeys[m].target_false;
                if i % monkeys[m].test_div == 0 {
                    // println!("    Current worry level is not divisible by {}.", monkeys[m].test_div);
                    target = monkeys[m].target_true;
                }
                // println!("    Item with worry level 500 is thrown to monkey {}.", target);
                monkeys[target].items.push_back(i);
            }
        }
        if round == 1 || round == 20 || round % 1000 == 0 {
            // println!("Inventory after round {}:", round);
            // for m in 0..monkeys.len() {
            //     println!("  Monkey {}: {:?}", m, monkeys[m].items);
            // }
            println!("== After round {} ==", round);
            for m in 0..monkeys.len() {
                println!("Monkey {} inspected items {} times.", m, monkeys[m].inspections);
            }
        }
    }
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    println!("Inspections: {:?}", inspections);
    inspections.sort();
    println!("Business level: {}", inspections[inspections.len() - 1] * inspections[inspections.len() - 2]);
    

    Ok(())
}
