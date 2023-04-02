use regex::Regex;

use crate::util;

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

pub fn run() {
    let addx = Regex::new(r"addx (\-?\d+)").unwrap();

    let input = util::read_input(10)
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

    println!("{signal_sum}");
    println!("{output}");
}
