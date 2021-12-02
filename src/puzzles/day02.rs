use lazy_static::lazy_static;
use std::str::FromStr;

use super::input;
use super::Puzzle;

lazy_static! {
    static ref INPUT: Vec<SubmarineInstruction> = input::read_lines_from_file("day02");
}

pub struct Day2;

impl Puzzle for Day2 {
    fn number(&self) -> u8 {
        2
    }

    fn part_one(&self) -> String {
        let position = compute_position_simple(&INPUT);
        format!(
            "Product of final position coordinates: {}",
            position.horizontal * position.depth
        )
    }

    fn part_two(&self) -> String {
        let position = compute_position(&INPUT);
        format!(
            "Product of final position coordinates: {}",
            position.horizontal * position.depth
        )
    }
}

fn compute_position_simple(instructions: &[SubmarineInstruction]) -> Position {
    let mut position = Position::new();
    position.apply_simple_instructions(instructions);
    position
}

fn compute_position(instructions: &[SubmarineInstruction]) -> Position {
    let mut position = Position::new();
    position.apply_instructions(instructions);
    position
}

#[derive(Copy, Clone)]
enum SubmarineInstruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for SubmarineInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, value) = s
            .split_once(' ')
            .ok_or_else(|| format!("Invalid instruction: {}", s))?;
        let value = value.parse::<u32>().map_err(|e| e.to_string())?;
        match instruction {
            "forward" => Ok(Self::Forward(value)),
            "down" => Ok(Self::Down(value)),
            "up" => Ok(Self::Up(value)),
            _ => Err(format!("Invalid instruction: {}", instruction)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Position {
    horizontal: u32,
    depth: u32,
    aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn apply_simple_instruction(&mut self, instruction: SubmarineInstruction) {
        match instruction {
            SubmarineInstruction::Forward(value) => self.horizontal += value,
            SubmarineInstruction::Down(value) => self.depth += value,
            SubmarineInstruction::Up(value) => self.depth -= value,
        }
    }

    pub fn apply_simple_instructions(&mut self, instructions: &[SubmarineInstruction]) {
        instructions
            .iter()
            .for_each(|instruction| self.apply_simple_instruction(*instruction));
    }

    pub fn apply_instruction(&mut self, instruction: SubmarineInstruction) {
        match instruction {
            SubmarineInstruction::Forward(value) => {
                self.horizontal += value;
                self.depth = (self.depth as i32 + self.aim * (value as i32)) as u32;
            }
            SubmarineInstruction::Down(value) => self.aim += value as i32,
            SubmarineInstruction::Up(value) => self.aim -= value as i32,
        }
    }

    pub fn apply_instructions(&mut self, instructions: &[SubmarineInstruction]) {
        instructions
            .iter()
            .for_each(|instruction| self.apply_instruction(*instruction));
    }
}

#[cfg(test)]
mod test {
    use crate::puzzles::input::read_lines;

    use super::*;

    #[test]
    fn day2_part1_sample() {
        let input = r"forward 5
                             down 5
                             forward 8
                             up 3
                             down 8
                             forward 2"
            .to_string();
        let instructions = read_lines(input.as_bytes());

        let result = compute_position_simple(&instructions);

        assert_eq!(result.horizontal, 15);
        assert_eq!(result.depth, 10);
    }

    #[test]
    fn day2_part2_sample() {
        let input = r"forward 5
                             down 5
                             forward 8
                             up 3
                             down 8
                             forward 2"
            .to_string();
        let instructions = read_lines(input.as_bytes());

        let result = compute_position(&instructions);

        assert_eq!(result.horizontal, 15);
        assert_eq!(result.depth, 60);
    }
}
