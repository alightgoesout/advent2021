use std::collections::HashMap;
use std::time::Instant;

mod input;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub trait Puzzle {
    fn number(&self) -> u8;

    fn part_one(&self) -> String;

    fn part_two(&self) -> String;

    fn execute(&self) {
        let start = Instant::now();
        println!("{}:1 — {}", self.number(), self.part_one());
        println!("{}:2 — {}", self.number(), self.part_two());
        let duration = start.elapsed();
        println!("Done in {}ms", duration.as_millis());
    }
}

pub fn puzzles() -> HashMap<u8, Box<dyn Puzzle>> {
    [
        Box::new(day01::Day1) as Box<dyn Puzzle>,
        Box::new(day02::Day2),
        Box::new(day03::Day3),
        Box::new(day04::Day4),
        Box::new(day05::Day5),
        Box::new(day06::Day6),
        Box::new(day07::Day7),
    ]
    .into_iter()
    .map(|puzzle| (puzzle.number(), puzzle))
    .collect()
}
