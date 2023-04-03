use std::collections::VecDeque;

use regex::Regex;

use crate::util;

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

fn one(monkeys: Vec<Monkey>) -> u64 {
    run_throws(monkeys, 20, 3)
}

fn two(monkeys: Vec<Monkey>) -> u64 {
    run_throws(monkeys, 10000, 1)
}

pub fn run() -> (u64, u64) {
    let input = util::read_input(11).join("\n");

    (one(parse_monkeys(&input)), two(parse_monkeys(&input)))
}
