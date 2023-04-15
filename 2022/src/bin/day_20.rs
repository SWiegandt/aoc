use aoc2022::util::read_input;

fn parse_input(input: &String) -> Vec<(usize, i64)> {
    input.lines().map(|row| row.parse().unwrap()).enumerate().collect()
}

fn mix(input: &mut Vec<(usize, i64)>) {
    let input_len = input.len() as i64;

    for i in 0.. {
        if let Some((pos, &(original_pos, num))) = input
            .iter()
            .enumerate()
            .find(|(_, &(original_pos, _))| original_pos == i)
        {
            let end_pos = (if pos as i64 + num >= 0 {
                pos as i64 + num
            } else {
                input_len + (pos as i64 + num) - 1
            })
            .rem_euclid(input_len - 1) as usize;

            if end_pos >= pos {
                for i in pos..end_pos {
                    input[i] = input[i + 1];
                }

                input[end_pos] = (original_pos, num);
            } else {
                for i in (end_pos..pos).rev() {
                    input[i + 1] = input[i]
                }

                input[end_pos] = (original_pos, num);
            }
        } else {
            break;
        }
    }
}

fn grove_sum(input: &Vec<(usize, i64)>) -> i64 {
    let zero_pos = input.iter().position(|&(_, num)| num == 0).unwrap();
    let mut cycle = input.iter().cycle();

    cycle.nth(zero_pos + 1000).unwrap().1 + cycle.nth(999).unwrap().1 + cycle.nth(999).unwrap().1
}

fn problem_one(input: &String) -> i64 {
    let mut input = parse_input(input);
    mix(&mut input);
    grove_sum(&input)
}

fn problem_two(input: &String) -> i64 {
    let mut input = parse_input(input)
        .iter()
        .map(|&(pos, num)| (pos, num * 811589153))
        .collect();

    for _ in 0..10 {
        mix(&mut input);
    }

    grove_sum(&input)
}

fn main() {
    let input = read_input(20);
    println!("Problem one: {:?}", problem_one(&input));
    println!("Problem two: {:?}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
1
2
-3
3
-2
0
4
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 3);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 1623178306);
    }
}
