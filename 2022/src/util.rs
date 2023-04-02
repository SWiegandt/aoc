use std::hash::Hash;
use std::{collections::HashSet, fs::File, io::BufRead};

pub fn read_input(day: usize) -> Vec<String> {
    let path = format!("./inputs/{}.txt", day.to_string());
    let file = File::open(path).unwrap();

    std::io::BufReader::new(file).lines().map(|row| row.unwrap()).collect()
}

pub fn to_set<T, I>(col: I) -> HashSet<T>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    col.into_iter().collect::<HashSet<_>>()
}
