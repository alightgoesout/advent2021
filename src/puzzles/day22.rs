use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::ops::RangeInclusive;
use std::str::FromStr;

use super::{input, Puzzle};

lazy_static! {
    static ref INSTRUCTIONS: Vec<Instruction> = input::read_lines_from_file("day22");
}

pub struct Day22;

impl Puzzle for Day22 {
    fn number(&self) -> u8 {
        22
    }

    fn part_one(&self) -> String {
        let mut core = Reactor::new();
        core.execute_all(INSTRUCTIONS.iter());
        format!(
            "Number of activated cubes: {}",
            core.count_activated_cubes()
        )
    }

    fn part_two(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cuboid {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

impl Cuboid {
    fn new(x: RangeInclusive<i64>, y: RangeInclusive<i64>, z: RangeInclusive<i64>) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    On(Cuboid),
    Off(Cuboid),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_REGEX: Regex = Regex::new(r"^(?P<status>on|off) x=(?P<x_start>-?\d+)\.\.(?P<x_end>-?\d+),y=(?P<y_start>-?\d+)\.\.(?P<y_end>-?\d+),z=(?P<z_start>-?\d+)\.\.(?P<z_end>-?\d+)$").unwrap();
        }

        let capture = INSTRUCTION_REGEX.captures(s).unwrap();

        let cuboid = Cuboid::new(
            (capture.name("x_start").unwrap().as_str().parse().unwrap())
                ..=(capture.name("x_end").unwrap().as_str().parse().unwrap()),
            (capture.name("y_start").unwrap().as_str().parse().unwrap())
                ..=(capture.name("y_end").unwrap().as_str().parse().unwrap()),
            (capture.name("z_start").unwrap().as_str().parse().unwrap())
                ..=(capture.name("z_end").unwrap().as_str().parse().unwrap()),
        );

        match capture.name("status").unwrap().as_str() {
            "on" => Ok(Instruction::On(cuboid)),
            _ => Ok(Instruction::Off(cuboid)),
        }
    }
}

#[derive(Debug)]
struct Reactor {
    cubes: [[[bool; 101]; 101]; 101],
}

impl Reactor {
    fn new() -> Self {
        Self {
            cubes: [[[false; 101]; 101]; 101],
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::On(cuboid) => self.update(cuboid, true),
            Instruction::Off(cuboid) => self.update(cuboid, false),
        }
    }

    pub fn execute_all<'a, I: IntoIterator<Item = &'a Instruction>>(&mut self, instructions: I) {
        for instruction in instructions {
            self.execute(instruction)
        }
    }

    fn update(&mut self, cuboid: &Cuboid, status: bool) {
        for x in max(*cuboid.x.start(), -50)..=min(*cuboid.x.end(), 50) {
            for y in max(*cuboid.y.start(), -50)..=min(*cuboid.y.end(), 50) {
                for z in max(*cuboid.z.start(), -50)..=min(*cuboid.z.end(), 50) {
                    self.cubes[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = status;
                }
            }
        }
    }

    fn count_activated_cubes(&self) -> usize {
        self.cubes
            .iter()
            .flat_map(|plane| plane.iter())
            .flat_map(|line| line.iter())
            .filter(|cube| **cube)
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE_INSTRUCTIONS: Vec<Instruction> = input::read_lines(
            "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
"
            .as_bytes()
        );
    }

    #[test]
    fn the_number_of_activated_cubes_after_executing_sample_instructions_should_be_39() {
        let mut core = Reactor::new();

        core.execute_all(SAMPLE_INSTRUCTIONS.iter());

        assert_eq!(core.count_activated_cubes(), 39);
    }
}
