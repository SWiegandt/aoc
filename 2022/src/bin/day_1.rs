use aoc2022::util::read_input;

fn elf_calories(input: &String) -> Vec<i32> {
    input
        .lines()
        .scan(0, |st, n| {
            let ret;

            (*st, ret) = match n {
                "" => (0, *st),
                _ => (*st + n.parse::<i32>().unwrap(), 0),
            };

            Some(ret)
        })
        .collect()
}

fn problem_one(input: &String) -> i32 {
    let mut calories = elf_calories(input);
    calories.sort_by(|a, b| b.cmp(a));
    calories[0]
}

fn problem_two(input: &String) -> i32 {
    let mut calories = elf_calories(input);
    calories.sort_by(|a, b| b.cmp(a));
    println!("{:?}", calories);
    calories[0..3].iter().sum()
}

fn main() {
    let input = read_input(1);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim_start().to_string()), 24000);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim_start().to_string()), 45000);
    }
}
