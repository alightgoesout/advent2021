use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: Vec<Line> = input::read_lines_from_file("day05");
}

pub struct Day5;

impl Puzzle for Day5 {
    fn number(&self) -> u8 {
        5
    }

    fn part_one(&self) -> String {
        let ocean_map = map_of_horizontal_and_vertical_lines::<1000>(&INPUT);
        format!(
            "Number of overlapping points without diagonal lines: {}",
            ocean_map.count_overlaps()
        )
    }

    fn part_two(&self) -> String {
        let ocean_map = map_lines::<1000>(&INPUT);
        format!(
            "Number of overlapping points with diagonal lines: {}",
            ocean_map.count_overlaps()
        )
    }
}

fn map_of_horizontal_and_vertical_lines<const N: usize>(lines: &[Line]) -> OceanMap<N> {
    let mut map = OceanMap::default();
    lines
        .iter() /*.filter(|line| line.0.0 == line.1.0 || line.0.1 == line.1.1)*/
        .for_each(|line| {
            if line.start.x == line.end.x {
                map.add_vertical_line(line.start.x, line.start.y, line.end.y);
            } else if line.start.y == line.end.y {
                map.add_horizontal_line(line.start.y, line.start.x, line.end.x);
            }
        });
    map
}

fn map_lines<const N: usize>(lines: &[Line]) -> OceanMap<N> {
    let mut map = OceanMap::default();
    lines
        .iter() /*.filter(|line| line.0.0 == line.1.0 || line.0.1 == line.1.1)*/
        .for_each(|line| {
            if line.start.x == line.end.x {
                map.add_vertical_line(line.start.x, line.start.y, line.end.y);
            } else if line.start.y == line.end.y {
                map.add_horizontal_line(line.start.y, line.start.x, line.end.x);
            } else {
                map.add_diagonal_line(*line);
            }
        });
    map
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(',')
            .ok_or(())
            .and_then(|(x, y)| match (x.parse(), y.parse()) {
                (Ok(x), Ok(y)) => Ok(Position { x, y }),
                _ => Err(()),
            })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Line {
    start: Position,
    end: Position,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" -> ")
            .ok_or(())
            .and_then(|(s, e)| match (s.parse(), e.parse()) {
                (Ok(s), Ok(e)) => Ok(Line { start: s, end: e }),
                _ => Err(()),
            })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct OceanMap<const N: usize>([[u32; N]; N]);

impl<const N: usize> OceanMap<N> {
    fn add_horizontal_line(&mut self, y: usize, from: usize, to: usize) {
        let x_start = min(from, to);
        let x_end = max(from, to);
        for x in x_start..=x_end {
            self.0[y][x] += 1;
        }
    }

    fn add_vertical_line(&mut self, x: usize, from: usize, to: usize) {
        let y_start = min(from, to);
        let y_end = max(from, to);
        for y in y_start..=y_end {
            self.0[y][x] += 1;
        }
    }

    fn add_diagonal_line(&mut self, Line { start, end }: Line) {
        let x_step = if start.x < end.x { 1 } else { -1 };
        let y_step = if start.y < end.y { 1 } else { -1 };
        let mut x = start.x;
        let mut y = start.y;
        while x != end.x && y != end.y {
            self.0[y][x] += 1;
            y = ((y as i32) + y_step) as usize;
            x = ((x as i32) + x_step) as usize;
        }
        self.0[end.y][end.x] += 1;
    }

    fn count_overlaps(&self) -> usize {
        (0..N)
            .cartesian_product(0..N)
            .filter(|&(x, y)| self.0[y][x] > 1)
            .count()
    }
}

impl<const N: usize> Default for OceanMap<N> {
    fn default() -> Self {
        Self([[0; N]; N])
    }
}

impl<const N: usize> Display for OceanMap<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..N {
            for x in 0..N {
                match self.0[y][x] {
                    0 => write!(f, ".")?,
                    n => write!(f, "{}", n)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: Vec<Line> = input::read_lines(
            r"0,9 -> 5,9
              8,0 -> 0,8
              9,4 -> 3,4
              2,2 -> 2,1
              7,0 -> 7,4
              6,4 -> 2,0
              0,9 -> 2,9
              3,4 -> 1,4
              0,0 -> 8,8
              5,5 -> 8,2"
                .as_bytes()
        );
    }

    #[test]
    fn sample_map_with_horizontal_and_vertical_lines() {
        let result = map_of_horizontal_and_vertical_lines::<10>(&SAMPLE);

        assert_eq!(
            result.to_string(),
            r".......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....
"
        );
    }

    #[test]
    fn points_overlap_for_horizontal_and_vertical_lines_should_be_5_for_sample() {
        let result = map_of_horizontal_and_vertical_lines::<10>(&SAMPLE);

        assert_eq!(result.count_overlaps(), 5);
    }

    #[test]
    fn empty_map_with_diagonal_line() {
        let mut ocean_map = OceanMap::<10>::default();

        ocean_map.add_diagonal_line(Line {
            start: Position { x: 8, y: 0 },
            end: Position { x: 0, y: 8 },
        });

        assert_eq!(
            ocean_map.to_string(),
            r"........1.
.......1..
......1...
.....1....
....1.....
...1......
..1.......
.1........
1.........
..........
"
        );
    }

    #[test]
    fn sample_map_with_all_lines() {
        let result = map_lines::<10>(&SAMPLE);

        assert_eq!(
            result.to_string(),
            r"1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....
"
        );
    }
}
