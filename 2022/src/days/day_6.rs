use crate::util;

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

fn one(input: &String) -> usize {
    find_signal_pos(input, 4)
}

fn two(input: &String) -> usize {
    find_signal_pos(input, 14)
}

pub fn run() -> (usize, usize) {
    let input = &util::read_input(6)[0];
    (one(&input), two(&input))
}
