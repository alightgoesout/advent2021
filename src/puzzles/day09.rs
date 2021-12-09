use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::ops::{BitOr, Index};

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: HeightMap<100, 100> = input::read_lines_from_file("day09").into();
}

pub struct Day9;

impl Puzzle for Day9 {
    fn number(&self) -> u8 {
        9
    }

    fn part_one(&self) -> String {
        format!(
            "Sum of the risk levels of all low points: {}",
            INPUT.find_low_points().map(risk_level).sum::<u32>()
        )
    }

    fn part_two(&self) -> String {
        let basins = INPUT.find_basins();
        format!(
            "Product of sizes of three largest basins: {}",
            basins.n_largest(3).map(Basin::len).product::<usize>()
        )
    }
}

fn risk_level((_, value): (Point, u8)) -> u32 {
    value as u32 + 1
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct HeightMap<const X: usize, const Y: usize>([[u8; X]; Y]);

impl<const X: usize, const Y: usize> HeightMap<X, Y> {
    fn is_low_point(&self, Point { x, y }: Point) -> bool {
        let value = self[Point { x, y }];
        (y == 0 || self[Point { x, y: y - 1 }] > value)
            && (y == Y - 1 || self[Point { x, y: y + 1 }] > value)
            && (x == 0 || self[Point { x: x - 1, y }] > value)
            && (x == X - 1 || self[Point { x: x + 1, y }] > value)
    }

    fn find_low_points(&self) -> impl Iterator<Item = (Point, u8)> + '_ {
        (0..X)
            .cartesian_product(0..Y)
            .map(|(x, y)| Point { x, y })
            .filter(|point| self.is_low_point(*point))
            .map(|point| (point, self[point]))
    }

    fn find_basins(&self) -> BasinSet {
        let mut basins = BasinSet::new();

        for y in 0..Y {
            let mut line_basin = Basin::new();
            for x in 0..X {
                let point = Point { x, y };
                match self[point] {
                    9 => {
                        if line_basin.len() > 0 {
                            basins.insert(line_basin);
                            line_basin = Basin::new();
                        }
                    }
                    _ => line_basin.insert(point),
                }
            }
            if line_basin.len() > 0 {
                basins.insert(line_basin);
            }
        }

        basins
    }
}

impl<const X: usize, const Y: usize> From<Vec<String>> for HeightMap<X, Y> {
    fn from(lines: Vec<String>) -> Self {
        let mut map = [[0; X]; Y];
        for (i, line) in lines.iter().enumerate() {
            for (j, location) in line.chars().enumerate() {
                map[i][j] = location.to_digit(10).unwrap() as u8;
            }
        }
        Self(map)
    }
}

impl<const X: usize, const Y: usize> Index<Point> for HeightMap<X, Y> {
    type Output = u8;

    fn index(&self, Point { x, y }: Point) -> &Self::Output {
        &self.0[y][x]
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Basin(HashSet<Point>);

impl Basin {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn insert(&mut self, point: Point) {
        self.0.insert(point);
    }

    fn is_adjacent_under(&self, other: &Basin) -> bool {
        self.0
            .iter()
            .any(|&Point { x, y }| other.0.contains(&Point { x, y: y + 1 }))
    }
}

impl BitOr for Basin {
    type Output = Basin;

    fn bitor(self, rhs: Self) -> Self::Output {
        Basin(&self.0 | &rhs.0)
    }
}

#[derive(Debug, Clone)]
struct BasinSet(Vec<Basin>);

impl BasinSet {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn insert(&mut self, basin: Basin) {
        if let Some(index) = self.0.iter().position(|b| b.is_adjacent_under(&basin)) {
            let existing_basin = self.0.remove(index);
            self.insert(existing_basin | basin);
        } else {
            self.0.push(basin)
        }
    }

    fn n_largest(&self, n: usize) -> impl Iterator<Item = &Basin> + '_ {
        self.0
            .iter()
            .sorted_by(|b1, b2| b2.len().cmp(&b1.len()))
            .take(n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: HeightMap<10, 5> = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ]
        .into();
    }

    #[test]
    fn find_low_points_should_return_four_points_for_sample() {
        let low_points: Vec<_> = SAMPLE.find_low_points().collect();

        assert_eq!(
            low_points,
            vec![
                (Point { x: 1, y: 0 }, 1),
                (Point { x: 2, y: 2 }, 5),
                (Point { x: 6, y: 4 }, 5),
                (Point { x: 9, y: 0 }, 0),
            ]
        );
    }

    #[test]
    fn sum_of_risk_levels_for_sample_should_be_15() {
        let risk_levels_sum: u32 = SAMPLE.find_low_points().map(risk_level).sum();

        assert_eq!(risk_levels_sum, 15);
    }

    #[test]
    fn product_of_three_largest_basins_for_sample_should_be_1134() {
        dbg!(SAMPLE.find_basins());
        let product = SAMPLE
            .find_basins()
            .n_largest(3)
            .map(Basin::len)
            .product::<usize>();

        assert_eq!(product, 1134);
    }
}
