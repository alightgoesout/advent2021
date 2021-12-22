use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min, Ordering};
use std::ops::Sub;
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
        let reactor = Reactor::new().execute_all(INSTRUCTIONS.iter().take(20));
        format!(
            "Number of activated cubes in center: {}",
            reactor.count_activated_cubes()
        )
    }

    fn part_two(&self) -> String {
        let reactor = Reactor::new().execute_all(INSTRUCTIONS.iter());
        format!(
            "Total number of activated cubes: {}",
            reactor.count_activated_cubes()
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn contains(&self, value: i64) -> bool {
        self.start <= value && value <= self.end
    }

    fn intersects(&self, other: &Self) -> bool {
        self.contains(other.start) || other.contains(self.start)
    }

    fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn intersects(&self, other: &Self) -> bool {
        self.x.intersects(&other.x) && self.y.intersects(&other.y) && self.z.intersects(&other.z)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            Some(Self {
                x: Range::new(
                    max(self.x.start, other.x.start),
                    min(self.x.end, other.x.end),
                ),
                y: Range::new(
                    max(self.y.start, other.y.start),
                    min(self.y.end, other.y.end),
                ),
                z: Range::new(
                    max(self.z.start, other.z.start),
                    min(self.z.end, other.z.end),
                ),
            })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }
}

impl Sub for Cuboid {
    type Output = Vec<Cuboid>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Some(intersection) = self.intersection(&rhs) {
            let x_ranges = divide_range(self.x, rhs.x);
            let y_ranges = divide_range(self.y, rhs.y);
            let z_ranges = divide_range(self.z, rhs.z);

            x_ranges
                .into_iter()
                .cartesian_product(y_ranges)
                .cartesian_product(z_ranges)
                .map(|((x, y), z)| Self { x, y, z })
                .filter(|cuboid| cuboid != &intersection)
                .collect()
        } else {
            vec![self]
        }
    }
}

fn divide_range(r1: Range, r2: Range) -> Vec<Range> {
    match (r1.start.cmp(&r2.start), r1.end.cmp(&r2.end)) {
        (Ordering::Less, Ordering::Less) => vec![
            Range::new(r1.start, r2.start - 1),
            Range::new(r2.start, r1.end),
        ],
        (Ordering::Less, Ordering::Equal) => vec![Range::new(r1.start, r2.start - 1), r2],
        (Ordering::Less, Ordering::Greater) => vec![
            Range::new(r1.start, r2.start - 1),
            r2,
            Range::new(r2.end + 1, r1.end),
        ],
        (Ordering::Equal, Ordering::Greater) => vec![r2, Range::new(r2.end + 1, r1.end)],
        (Ordering::Greater, Ordering::Greater) => {
            vec![Range::new(r1.start, r2.end), Range::new(r2.end + 1, r1.end)]
        }
        _ => vec![r1],
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

        let cuboid = Cuboid {
            x: Range::new(
                capture.name("x_start").unwrap().as_str().parse().unwrap(),
                capture.name("x_end").unwrap().as_str().parse().unwrap(),
            ),
            y: Range::new(
                capture.name("y_start").unwrap().as_str().parse().unwrap(),
                capture.name("y_end").unwrap().as_str().parse().unwrap(),
            ),
            z: Range::new(
                capture.name("z_start").unwrap().as_str().parse().unwrap(),
                capture.name("z_end").unwrap().as_str().parse().unwrap(),
            ),
        };

        match capture.name("status").unwrap().as_str() {
            "on" => Ok(Instruction::On(cuboid)),
            _ => Ok(Instruction::Off(cuboid)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reactor {
    cuboids: Vec<Cuboid>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            cuboids: Vec::new(),
        }
    }

    fn execute(self, instruction: &Instruction) -> Self {
        match *instruction {
            Instruction::On(cuboid) => {
                let mut cuboids: Vec<_> =
                    self.cuboids.into_iter().flat_map(|c| c - cuboid).collect();
                cuboids.push(cuboid);
                Self { cuboids }
            }
            Instruction::Off(cuboid) => Self {
                cuboids: self.cuboids.into_iter().flat_map(|c| c - cuboid).collect(),
            },
        }
    }

    fn execute_all<'a>(self, instructions: impl Iterator<Item = &'a Instruction>) -> Self {
        instructions
            .into_iter()
            .fold(self, |reactor, instruction| reactor.execute(instruction))
    }

    fn count_activated_cubes(&self) -> usize {
        self.cuboids.iter().map(Cuboid::len).sum()
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
        let reactor = Reactor::new();

        let result = reactor.execute_all(SAMPLE_INSTRUCTIONS.iter());

        assert_eq!(result.count_activated_cubes(), 39);
    }

    #[test]
    fn test_intersection() {
        let c1 = Cuboid {
            x: Range::new(0, 3),
            y: Range::new(0, 3),
            z: Range::new(0, 3),
        };
        let c2 = Cuboid {
            x: Range::new(2, 4),
            y: Range::new(2, 4),
            z: Range::new(2, 4),
        };

        let intersection = c1.intersection(&c2);

        assert_eq!(
            intersection,
            Some(Cuboid {
                x: Range::new(2, 3),
                y: Range::new(2, 3),
                z: Range::new(2, 3),
            })
        );
    }

    #[test]
    fn test_subtraction_with_intersection_on_each_axis() {
        let c1 = Cuboid {
            x: Range::new(0, 3),
            y: Range::new(0, 3),
            z: Range::new(0, 3),
        };
        let c2 = Cuboid {
            x: Range::new(2, 4),
            y: Range::new(2, 4),
            z: Range::new(2, 4),
        };

        let result = c1 - c2;

        assert_eq!(
            result,
            vec![
                Cuboid {
                    x: Range::new(0, 1),
                    y: Range::new(0, 1),
                    z: Range::new(0, 1),
                },
                Cuboid {
                    x: Range::new(0, 1),
                    y: Range::new(0, 1),
                    z: Range::new(2, 3),
                },
                Cuboid {
                    x: Range::new(0, 1),
                    y: Range::new(2, 3),
                    z: Range::new(0, 1),
                },
                Cuboid {
                    x: Range::new(0, 1),
                    y: Range::new(2, 3),
                    z: Range::new(2, 3),
                },
                Cuboid {
                    x: Range::new(2, 3),
                    y: Range::new(0, 1),
                    z: Range::new(0, 1),
                },
                Cuboid {
                    x: Range::new(2, 3),
                    y: Range::new(0, 1),
                    z: Range::new(2, 3),
                },
                Cuboid {
                    x: Range::new(2, 3),
                    y: Range::new(2, 3),
                    z: Range::new(0, 1),
                },
            ]
        );
    }

    #[test]
    fn test_subtraction_empty_result() {
        let c1 = Cuboid {
            x: Range::new(0, 3),
            y: Range::new(0, 3),
            z: Range::new(0, 3),
        };
        let c2 = Cuboid {
            x: Range::new(0, 4),
            y: Range::new(0, 4),
            z: Range::new(0, 4),
        };

        let result = c1 - c2;

        assert!(result.is_empty());
    }
}
