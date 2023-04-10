use aoc2022::util::{read_input, ToInputVec};

use Move::*;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_num(n: i32) -> Self {
        match n {
            0 => Rock,
            1 => Paper,
            2 => Scissors,
            _ => todo!(),
        }
    }
}

fn to_move(s: &str) -> Move {
    match s {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissors,
        _ => todo!(),
    }
}

fn to_result(move_str: &str, result: &str) -> (Move, Move) {
    match (to_move(move_str), result) {
        (mv_enum, "X") => (mv_enum, Move::from_num((mv_enum as i32 - 1).rem_euclid(3))),
        (mv_enum, "Y") => (mv_enum, mv_enum),
        (mv_enum, "Z") => (mv_enum, Move::from_num((mv_enum as i32 + 1).rem_euclid(3))),
        _ => todo!(),
    }
}

fn game_result(l: Move, r: Move) -> i32 {
    let result = match (l, r) {
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        _ if l == r => 3,
        _ => 0,
    };

    result + r as i32 + 1
}

fn sum_results<F: Fn(&str, &str) -> (Move, Move)>(input: &Vec<String>, f: F) -> i32 {
    input
        .iter()
        .map(|game| {
            let mut moves = game.split(' ');
            let (l, r) = f(moves.next().unwrap(), moves.next().unwrap());

            game_result(l, r)
        })
        .sum()
}

fn problem_one(input: &String) -> i32 {
    sum_results(&input.to_vec(), |l, r| (to_move(l), to_move(r)))
}

fn problem_two(input: &String) -> i32 {
    sum_results(&input.to_vec(), to_result)
}

fn main() {
    let input = read_input(2);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.to_string()), 15);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.to_string()), 12);
    }
}
