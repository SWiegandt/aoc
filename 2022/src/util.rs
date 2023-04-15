use core::fmt;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;
use std::str::FromStr;

use regex::Captures;

pub trait ToInputVec {
    fn to_vec(&self) -> Vec<String>;
}

impl ToInputVec for String {
    fn to_vec(&self) -> Vec<String> {
        self.lines().map(|s| s.to_string()).collect()
    }
}

pub fn read_input(day: usize) -> String {
    read_to_string(format!("./inputs/{}.txt", day.to_string())).unwrap()
}

pub fn to_set<T, I>(col: I) -> HashSet<T>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    col.into_iter().collect::<HashSet<_>>()
}

pub fn parse_capture<T>(captures: &Captures, i: usize) -> T
where
    T: FromStr,
    T::Err: fmt::Debug,
{
    captures.get(i).unwrap().as_str().parse().unwrap()
}
