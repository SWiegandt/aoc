use aoc2022::util::{read_input, to_set, ToInputVec};

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
        .map(|sack| to_set(sack.chars()))
        .reduce(|acc, sack| to_set(acc.intersection(&sack).copied()))?
        .into_iter()
        .next()
}

fn problem_one(input: &String) -> i32 {
    input
        .to_vec()
        .iter()
        .map(split_sack)
        .map(|(l, r)| find_duplicate(vec![l, r]))
        .map(|c| priority(c.unwrap()))
        .sum()
}

fn problem_two(input: &String) -> i32 {
    input
        .to_vec()
        .chunks(3)
        .map(|chunk| find_duplicate(chunk.to_vec()))
        .map(|c| priority(c.unwrap()))
        .sum()
}

fn main() {
    let input = read_input(3);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 157);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 70);
    }
}
