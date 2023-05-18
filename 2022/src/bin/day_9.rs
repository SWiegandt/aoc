use std::collections::HashSet;

use aoc2022::util::read_input;

struct Move(char, i32);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos(i32, i32);

fn update_knot(knot: &mut [Pos]) {
    if i32::max((knot[0].0 - knot[1].0).abs(), (knot[0].1 - knot[1].1).abs()) <= 1 {
        return;
    } else if knot[0].0 != knot[1].0 || knot[0].1 != knot[1].1 {
        knot[1].0 += (knot[0].0 - knot[1].0).signum();
        knot[1].1 += (knot[0].1 - knot[1].1).signum();
    } else if (knot[0].0 - knot[1].0).abs() > 1 {
        knot[1].0 += (knot[0].0 - knot[1].0).signum();
    } else if (knot[0].1 - knot[1].1).abs() > 1 {
        knot[1].1 += (knot[0].1 - knot[1].1).signum();
    }
}

fn run_moves(input: &Vec<Move>, mut rope: Vec<Pos>) -> usize {
    let mut seen = HashSet::new();

    for &Move(dir, num) in input {
        for _ in 0..num {
            match dir {
                'L' => rope[0].0 -= 1,
                'R' => rope[0].0 += 1,
                'U' => rope[0].1 += 1,
                'D' => rope[0].1 -= 1,
                _ => unreachable!(),
            };

            for n in 0..rope.len() - 1 {
                update_knot(&mut rope[n..=n + 1]);
            }

            seen.insert(rope.last().unwrap().clone());
        }
    }

    seen.len()
}

fn problem_one(input: &String) -> usize {
    let input = parse_input(&input);
    run_moves(&input, vec![Pos(0, 0); 2])
}

fn problem_two(input: &String) -> usize {
    let input = parse_input(&input);
    run_moves(&input, vec![Pos(0, 0); 10])
}

fn parse_input(input: &String) -> Vec<Move> {
    input
        .lines()
        .map(|row| {
            let (dir, num) = row.split_once(" ").unwrap();
            Move(dir.chars().nth(0).unwrap(), num.parse().unwrap())
        })
        .collect()
}

fn main() {
    let input = read_input(9);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 13);
    }

    #[test]
    fn test_problem_two() {
        let test_input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        assert_eq!(super::problem_two(&test_input.trim().to_string()), 36);
    }
}
