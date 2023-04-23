use std::collections::HashMap;

use aoc2022::util::read_input;

fn to_snafu_vec(num: i64, head_exponent: u32) -> Option<Vec<(u32, i64)>> {
    if num.abs() <= 2 {
        return Some(vec![(0, num.signum() * num)]);
    } else if head_exponent == 0 {
        return None;
    }

    for head_factor in vec![0, 1, 2] {
        let head = num.signum() * head_factor * 5i64.pow(head_exponent);

        if let Some(tail_snafu) = to_snafu_vec(num - head, head_exponent - 1) {
            return Some([vec![(head_exponent, num.signum() * head_factor)], tail_snafu].concat());
        }
    }

    None
}

fn to_snafu(num: i64) -> Option<String> {
    let snafu_map: HashMap<u32, i64> = to_snafu_vec(num, (num as f64).log(5.0).ceil() as u32)?
        .into_iter()
        .collect();

    let max_exponent = snafu_map
        .iter()
        .filter(|&(_, factor)| *factor != 0)
        .map(|(exponent, _)| exponent)
        .max()?;

    Some(
        (0..=*max_exponent)
            .rev()
            .map(|exponent| match snafu_map.get(&exponent).unwrap_or(&0) {
                -2 => "=",
                -1 => "-",
                0 => "0",
                1 => "1",
                2 => "2",
                _ => panic!(),
            })
            .collect(),
    )
}

fn from_snafu(snafu: &String) -> i64 {
    let mut decimal = 0;

    for (exponent, factor) in snafu.chars().rev().enumerate() {
        decimal += 5i64.pow(exponent as u32)
            * match factor {
                '=' => -2,
                '-' => -1,
                c => c.to_digit(10).unwrap() as i64,
            }
    }

    decimal
}

fn problem_one(input: &String) -> String {
    let decimals = input.lines().map(|row| from_snafu(&row.to_string()));
    to_snafu(decimals.sum()).unwrap()
}

fn problem_two(input: &String) {}

fn main() {
    let input = read_input(25);
    println!("Problem one: {:?}", problem_one(&input));
    println!("Problem two: {:?}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), "2=-1=0");
    }
}
