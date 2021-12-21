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
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

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
        Box::new(day08::Day8),
        Box::new(day09::Day9),
        Box::new(day10::Day10),
        Box::new(day11::Day11),
        Box::new(day12::Day12),
        Box::new(day13::Day13),
        Box::new(day14::Day14),
        Box::new(day15::Day15),
        Box::new(day16::Day16),
        Box::new(day17::Day17),
        Box::new(day18::Day18),
        Box::new(day19::Day19),
        Box::new(day20::Day20),
        Box::new(day21::Day21),
    ]
    .into_iter()
    .map(|puzzle| (puzzle.number(), puzzle))
    .collect()
}
