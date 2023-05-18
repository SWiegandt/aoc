use aoc2022::util::read_input;

fn find_signal_pos(input: &String, signal_length: usize) -> usize {
    let input_chars = input.chars().collect::<Vec<_>>();
    let mut n = 0;

    'outer: while n < input.len() {
        let buf = &input_chars[n..n + signal_length];

        for (m, ch) in buf.iter().enumerate() {
            if buf[m + 1..].iter().position(|c| c == ch).is_some() {
                n += m + 1;
                continue 'outer;
            }
        }

        break;
    }

    n + signal_length
}

fn problem_one(input: &String) -> usize {
    find_signal_pos(input, 4)
}

fn problem_two(input: &String) -> usize {
    find_signal_pos(input, 14)
}

fn main() {
    let input = read_input(6);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 7);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 19);
    }
}
