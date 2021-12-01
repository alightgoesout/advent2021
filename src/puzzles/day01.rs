use lazy_static::lazy_static;

use super::input;
use super::Puzzle;

lazy_static! {
    static ref INPUT: Vec<u32> = input::read_lines("day01");
}

pub struct Day1;

impl Puzzle for Day1 {
    fn number(&self) -> u8 {
        1
    }

    fn part_one(&self) -> String {
        format!("Number of increases: {}", count_increases(&INPUT))
    }

    fn part_two(&self) -> String {
        format!(
            "Number of increases on a three elements sliding window: {}",
            count_increases(&three_elements_sliding_window(&INPUT))
        )
    }
}

fn count_increases(input: &[u32]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

fn three_elements_sliding_window(input: &[u32]) -> Vec<u32> {
    input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect()
}
