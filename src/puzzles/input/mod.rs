use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub fn read_lines_from_file<T>(name: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    read_lines(File::open(format!("src/puzzles/input/{}", name)).unwrap())
}

pub fn read_lines<T, R>(reader: R) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
    R: Read,
{
    let buf_reader = BufReader::new(reader);
    buf_reader
        .lines()
        .filter(Result::is_ok)
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}
