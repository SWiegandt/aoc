use aoc2022::util::{read_input, ToInputVec};
use regex::Regex;

enum Inst {
    Noop,
    Addx(i32),
}

fn run_cycle(cycle: &mut i32, x: &i32, signal_sum: &mut i32) -> char {
    let pixel = *cycle % 40;
    *cycle += 1;

    if *cycle == 20 || (*cycle - 20) % 40 == 0 {
        *signal_sum += *cycle * *x;
    }

    if *x - 1 <= pixel && pixel <= *x + 1 {
        '#'
    } else {
        '.'
    }
}

fn problem_one_and_two(input: &String) -> (i32, String) {
    let addx = Regex::new(r"addx (\-?\d+)").unwrap();

    let input = input
        .to_vec()
        .iter()
        .map(|row| {
            if row == "noop" {
                Inst::Noop
            } else if let Some(captures) = addx.captures(row) {
                Inst::Addx(captures.get(1).unwrap().as_str().parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<_>>();

    let mut x = 1;
    let mut cycle = 0;
    let mut signal_sum = 0;
    let mut crt = vec![];

    for inst in input {
        crt.push(run_cycle(&mut cycle, &x, &mut signal_sum));

        if let Inst::Addx(v) = inst {
            crt.push(run_cycle(&mut cycle, &x, &mut signal_sum));
            x += v;
        }
    }

    let output = crt
        .chunks(40)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    (signal_sum, output)
}

fn main() {
    let input = read_input(10);
    let (signal_sum, output) = problem_one_and_two(&input);
    println!("Problem one: {}", signal_sum);
    println!("Problem two: \n{}", output);
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one_and_two(&TEST_INPUT.trim().to_string()).0, 13140);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(
            super::problem_one_and_two(&TEST_INPUT.trim().to_string()).1,
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            .trim()
        );
    }
}
