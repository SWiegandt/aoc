use aoc2022::util::{read_input, to_set, ToInputVec};

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
    let l = to_set(l);
    let r = to_set(r);
    (l.len(), r.len(), to_set(l.intersection(&r)).len())
}

fn problem_one(input: &String) -> usize {
    input
        .to_vec()
        .iter()
        .map(get_range_pair)
        .map(|(l, r)| overlap(l, r))
        .filter(|(l, r, i)| l == i || r == i)
        .count()
}

fn problem_two(input: &String) -> usize {
    input
        .to_vec()
        .iter()
        .map(get_range_pair)
        .map(|(l, r)| overlap(l, r))
        .filter(|(_, _, i)| *i > 0)
        .count()
}

fn main() {
    let input = read_input(4);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

mod tests {
    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.to_string()), 2);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.to_string()), 4);
    }
}
