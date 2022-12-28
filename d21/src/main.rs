use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Job {
    I(i64),
    Expr((String, char, String)),
}

fn parse_line(l: &str) -> (String, Job) {
    let name = l[..4].to_string();
    let job = &l[6..];
    let job = match job.len() {
        11 => {
            let n1 = job[..4].to_string();
            let n2 = job[7..].to_string();
            let op = job[5..6].chars().next().unwrap();
            Job::Expr((n1, op, n2))
        }
        _ => Job::I(job.parse().unwrap()),
    };
    (name, job)
}

#[derive(Debug, Clone)]
enum Expr {
    I(i64),
    Human,
    Op(Box<(Expr, char, Expr)>),
}

fn monkey_expr(name: &str, monkeys: &HashMap<String, Job>, tag_human: bool) -> Expr {
    if tag_human && name == "humn" {
        return Expr::Human;
    }
    let m = monkeys.get(name).unwrap();
    match m {
        Job::I(v) => Expr::I(*v),
        Job::Expr((m1, op, m2)) => {
            let v1 = monkey_expr(m1, monkeys, tag_human);
            let v2 = monkey_expr(m2, monkeys, tag_human);
            Expr::Op(Box::new((v1, *op, v2)))
        }
    }
}

fn solve(e: &Expr) -> Option<i64> {
    match e {
        Expr::Human => None,
        Expr::I(i) => Some(*i),
        Expr::Op(o) => match (solve(&o.0), solve(&o.2)) {
            (Some(a), Some(b)) => match o.1 {
                '+' => Some(a + b),
                '-' => Some(a - b),
                '*' => Some(a * b),
                '/' => Some(a / b),
                c => panic!("expected op {}", c),
            },
            _ => None,
        },
    }
}

fn unpack_h(mut a: Expr, b: Expr) -> i64 {
    let mut r;
    if let Some(i) = solve(&a) {
        a = b;
        r = i;
    } else {
        if let Some(i) = solve(&b) {
            r = i;
        } else {
            panic!("neither a nor b are solvable.");
        }
    }
    loop {
        match a {
            Expr::Human => return r,
            Expr::I(x) => panic!("no human in {:?}", x),
            Expr::Op(o) => match o.1 {
                '+' => {
                    if let Some(i) = solve(&o.0) {
                        a = o.2;
                        r -= i;
                    } else if let Some(i) = solve(&o.2) {
                        a = o.0;
                        r -= i;
                    } else {
                        panic!("can't solve either side of +");
                    }
                }
                '-' => {
                    if let Some(i) = solve(&o.0) {
                        a = o.2;
                        r = i - r;
                    } else if let Some(i) = solve(&o.2) {
                        a = o.0;
                        r += i;
                    } else {
                        panic!("can't solve either side of -");
                    }
                }
                '*' => {
                    if let Some(i) = solve(&o.0) {
                        a = o.2;
                        r /= i;
                    } else if let Some(i) = solve(&o.2) {
                        a = o.0;
                        r /= i;
                    } else {
                        panic!("can't solve either side of *");
                    }
                }
                '/' => {
                    if let Some(i) = solve(&o.0) {
                        a = o.2;
                        r = i / r;
                    } else if let Some(i) = solve(&o.2) {
                        a = o.0;
                        r *= i;
                    } else {
                        panic!("can't solve either side of +");
                    }
                }
                x => panic!("expected op {}", x),
            },
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let monkeys = stdin
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect::<HashMap<_, _>>();
    let root = monkey_expr("root", &monkeys, false);
    println!("p1 root: {}", solve(&root).unwrap());

    let (r1, r2) = match monkeys.get("root") {
        Some(Job::Expr((r1, _, r2))) => (r1, r2),
        _ => panic!("root has not expr job"),
    };
    let r1 = monkey_expr(r1, &monkeys, true);
    let r2 = monkey_expr(r2, &monkeys, true);
    println!("p2 human = {}", unpack_h(r1, r2));

    Ok(())
}
