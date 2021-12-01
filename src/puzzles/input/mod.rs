use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn read_lines<T>(name: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let file = File::open(format!("src/puzzles/input/{}", name)).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}
