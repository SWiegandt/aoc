use aoc2022::util::read_input;

fn parse_input(input: &String) -> _ {}

fn problem_one(input: &String) -> _ {}

fn problem_two(input: &String) -> _ {}

fn main() {
    let input = read_input(_);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.to_string()), _);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.to_string()), _);
    }
}
