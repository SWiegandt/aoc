use crate::util;

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 65 + 27,
        _ => 0,
    }
}

fn split_sack(sack: &String) -> (String, String) {
    match sack.split_at(sack.len() / 2) {
        (l, r) => (String::from(l), String::from(r)),
    }
}

fn find_duplicate<T: IntoIterator<Item = String>>(sacks: T) -> Option<char> {
    sacks
        .into_iter()
        .map(|sack| util::to_set(sack.chars()))
        .reduce(|acc, sack| util::to_set(acc.intersection(&sack).copied()))?
        .into_iter()
        .next()
}

fn one(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(split_sack)
        .map(|(l, r)| find_duplicate(vec![l, r]))
        .map(|c| priority(c.unwrap()))
        .sum()
}

fn two(input: &Vec<String>) -> i32 {
    input
        .chunks(3)
        .map(|chunk| find_duplicate(chunk.to_vec()))
        .map(|c| priority(c.unwrap()))
        .sum()
}

pub fn run() -> (i32, i32) {
    let input = util::read_input(3);
    (one(&input), two(&input))
}
