use crate::util;

fn one(input: &Vec<String>) -> i32 {
    1
}

fn two(input: &Vec<String>) -> i32 {
    2
}

pub fn run() -> (i32, i32) {
    let input = util::read_input(_);
    (one(&input), two(&input))
}
