use aoc2022::util::read_input;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Add => String::from("+"),
            Op::Subtract => String::from("-"),
            Op::Multiply => String::from("*"),
            Op::Divide => String::from("/"),
        }
    }
}

impl Op {
    fn run(&self, v: i64, w: i64) -> i64 {
        match self {
            Op::Add => v + w,
            Op::Subtract => v - w,
            Op::Multiply => v * w,
            Op::Divide => v / w,
        }
    }

    fn from_str(s: &str) -> Op {
        match s {
            "+" => Op::Add,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
enum Monkey {
    Expression(Op, String, String),
    Value(i64),
}

fn evaluate(monkeys: &mut HashMap<String, Monkey>, name: &String) -> i64 {
    let monkey = monkeys.get(name).unwrap().clone();

    match monkey {
        Monkey::Expression(op, m1, m2) => {
            let val = op.run(evaluate(monkeys, &m1), evaluate(monkeys, &m2));
            monkeys.insert(name.clone(), Monkey::Value(val));
            val
        }
        Monkey::Value(val) => val,
    }
}

fn reduce_expr(monkeys: &mut HashMap<String, Monkey>, monkey: &String) -> String {
    if monkey == "humn" {
        String::from("humn")
    } else {
        match monkeys.get(monkey).unwrap().clone() {
            Monkey::Value(val) => val.to_string(),
            Monkey::Expression(op, left, right) => {
                format!(
                    "({}{}{})",
                    reduce_expr(monkeys, &left),
                    op.to_string(),
                    reduce_expr(monkeys, &right)
                )
            }
        }
    }
}

fn parse_input(input: &String) -> HashMap<String, Monkey> {
    let re = Regex::new(r"(.+): (((.+) ([+\-*/]) (.+))|(\d+))").unwrap();

    input
        .lines()
        .map(|row| {
            let captures = re.captures(row).unwrap();

            (
                captures.get(1).unwrap().as_str().to_string(),
                if let Some(n) = captures.get(7) {
                    Monkey::Value(n.as_str().parse().unwrap())
                } else {
                    Monkey::Expression(
                        Op::from_str(captures.get(5).unwrap().as_str()),
                        captures.get(4).unwrap().as_str().to_string(),
                        captures.get(6).unwrap().as_str().to_string(),
                    )
                },
            )
        })
        .collect()
}

fn problem_one(input: &String) -> i64 {
    let mut monkeys = parse_input(input);
    evaluate(&mut monkeys, &"root".to_string())
}

fn problem_two(input: &String) {
    let mut monkeys = parse_input(input);

    if let Monkey::Expression(_, left, right) = monkeys.get(&"root".to_string()).unwrap().clone() {
        println!(
            "{} = {}",
            reduce_expr(&mut monkeys, &left),
            reduce_expr(&mut monkeys, &right)
        );
    }
}

fn main() {
    let input = read_input(21);
    println!("Problem one: {:?}", problem_one(&input));
    println!("Problem two: {:?}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 152);
    }

    // #[test]
    // fn test_problem_two() {
    //     assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 301);
    // }
}
