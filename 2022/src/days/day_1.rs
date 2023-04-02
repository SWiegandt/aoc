use crate::util;

pub fn elf_calories() -> Vec<i32> {
    util::read_input(1)
        .iter()
        .scan(0, |st, n| {
            let ret;

            (*st, ret) = match n.as_str() {
                "" => (0, *st),
                _ => (*st + n.parse::<i32>().unwrap(), 0),
            };

            Some(ret)
        })
        .collect()
}

pub fn run() -> (i32, i32) {
    let mut calories = elf_calories();
    calories.sort_by(|a, b| b.cmp(a));
    (calories[0], calories[0..3].iter().sum())
}
