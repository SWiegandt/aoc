use std::collections::VecDeque;

use regex::Regex;

use aoc2022::util::read_input;

#[derive(Debug, Default)]
struct Monkey {
    items: VecDeque<u64>,
    op: (char, String),
    divisibility: u64,
    true_throw: usize,
    false_throw: usize,
    inspections: u64,
}

impl Monkey {
    fn inspect(&mut self) -> u64 {
        self.inspections += 1;
        let item_val = self.items.pop_front().unwrap();

        let op_val: u64 = match self.op.1.as_str() {
            "old" => item_val,
            v => v.parse().unwrap(),
        };

        let ret = match self.op.0 {
            '*' => item_val * op_val,
            '+' => item_val + op_val,
            _ => unreachable!(),
        };

        ret
    }

    fn test(&self, item_val: u64) -> bool {
        item_val % self.divisibility == 0
    }
}

fn parse_monkeys(input: &String) -> Vec<Monkey> {
    let monkey_re = Regex::new(r"Monkey (\d+):\s+Starting items: (.+)\s+Operation: new = old (.+) (.+)\s+Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)").unwrap();
    let mut monkeys = vec![];

    for m in monkey_re.captures_iter(input.as_str()) {
        monkeys.push(Monkey {
            items: m
                .get(2)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            op: (
                m.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                m.get(4).unwrap().as_str().to_string(),
            ),
            divisibility: m.get(5).unwrap().as_str().parse().unwrap(),
            true_throw: m.get(6).unwrap().as_str().parse().unwrap(),
            false_throw: m.get(7).unwrap().as_str().parse().unwrap(),
            ..Default::default()
        })
    }

    monkeys
}

fn run_throws(mut monkeys: Vec<Monkey>, rounds: usize, denominator: u64) -> u64 {
    let reduce_mod: u64 = monkeys.iter().map(|m| m.divisibility).product();

    for _ in 0..rounds {
        for n in 0..monkeys.len() {
            while !monkeys[n].items.is_empty() {
                let item_val = monkeys[n].inspect() / denominator;

                let throw_to = if monkeys[n].test(item_val) {
                    monkeys[n].true_throw
                } else {
                    monkeys[n].false_throw
                };

                monkeys[throw_to].items.push_back(item_val % reduce_mod);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.inspections.cmp(&m1.inspections));
    monkeys.iter().take(2).map(|m| m.inspections).product()
}

fn problem_one(monkeys: &String) -> u64 {
    run_throws(parse_monkeys(monkeys), 20, 3)
}

fn problem_two(monkeys: &String) -> u64 {
    run_throws(parse_monkeys(monkeys), 10000, 1)
}

fn main() {
    let input = read_input(11);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

mod tests {
    const TEST_INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.to_string()), 10605);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.to_string()), 2713310158);
    }
}
