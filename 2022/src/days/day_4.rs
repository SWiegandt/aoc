use crate::util;

fn parse_range(s: &str) -> Vec<i32> {
    match s.split_once('-').unwrap() {
        (l, r) => (l.parse::<i32>().unwrap()..=r.parse::<i32>().unwrap()).collect(),
    }
}

fn get_range_pair(s: &String) -> (Vec<i32>, Vec<i32>) {
    let (l, r) = s.split_once(',').unwrap();
    (parse_range(l), parse_range(r))
}

fn overlap(l: Vec<i32>, r: Vec<i32>) -> (usize, usize, usize) {
    let l = util::to_set(l);
    let r = util::to_set(r);
    (l.len(), r.len(), util::to_set(l.intersection(&r)).len())
}

fn one(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(get_range_pair)
        .map(|(l, r)| overlap(l, r))
        .filter(|(l, r, i)| l == i || r == i)
        .count()
}

fn two(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(get_range_pair)
        .map(|(l, r)| overlap(l, r))
        .filter(|(_, _, i)| *i > 0)
        .count()
}

pub fn run() -> (usize, usize) {
    let input = util::read_input(4);
    (one(&input), two(&input))
}
