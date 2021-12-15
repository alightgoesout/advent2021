use lazy_static::lazy_static;
use std::cmp::min;
use std::collections::HashMap;
use std::ops::Index;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: RiskLevelMap<100> = input::read_file("day15").as_str().into();
}

pub struct Day15;

impl Puzzle for Day15 {
    fn number(&self) -> u8 {
        15
    }

    fn part_one(&self) -> String {
        format!(
            "Lowest total risk: {}",
            INPUT.lowest_risk_from_start_to_end()
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Lowest total risk for full map: {}",
            INPUT.grow::<500>().lowest_risk_from_start_to_end()
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(usize, usize);

#[derive(Debug, Clone, Eq, PartialEq)]
struct RiskLevelMap<const N: usize>([[u8; N]; N]);

impl<const N: usize> RiskLevelMap<N> {
    fn lowest_risk_from_start_to_end(&self) -> u32 {
        let start = Point(0, 0);
        let end = Point(N - 1, N - 1);
        let mut lowest_risks: HashMap<Point, u32> = [(start, 0)].into();
        let mut visited = [[false; N]; N];

        while !visited[N - 1][N - 1] {
            let (&current_point, &current_risk) = lowest_risks
                .iter()
                .filter(|(&Point(x, y), _)| !visited[y][x])
                .min_by(|(_, r1), (_, r2)| r1.cmp(r2))
                .unwrap();
            for Point(x, y) in self
                .get_neighbors(current_point)
                .filter(|&Point(x, y)| !visited[y][x])
            {
                let new_risk = current_risk + self.0[y][x] as u32;
                lowest_risks
                    .entry(Point(x, y))
                    .and_modify(|r| *r = min(*r, new_risk))
                    .or_insert(new_risk);
            }
            visited[current_point.1][current_point.0] = true;
        }

        lowest_risks[&end]
    }

    fn get_neighbors(&self, Point(x, y): Point) -> impl Iterator<Item = Point> {
        [
            (x > 0).then(|| Point(x - 1, y)),
            (x < N - 1).then(|| Point(x + 1, y)),
            (y > 0).then(|| Point(x, y - 1)),
            (y < N - 1).then(|| Point(x, y + 1)),
        ]
        .into_iter()
        .flatten()
    }

    fn grow<const M: usize>(&self) -> RiskLevelMap<M> {
        let mut map = [[0; M]; M];

        for (y, line) in map.iter_mut().enumerate() {
            for (x, risk) in line.iter_mut().enumerate() {
                *risk = wrap_from_nine_to_one(self.0[y % N][x % N] + (x / N) as u8 + (y / N) as u8);
            }
        }

        RiskLevelMap(map)
    }
}

fn wrap_from_nine_to_one(n: u8) -> u8 {
    if n < 10 {
        n
    } else {
        wrap_from_nine_to_one(n - 9)
    }
}

impl<const N: usize> Index<Point> for RiskLevelMap<N> {
    type Output = u8;

    fn index(&self, Point(x, y): Point) -> &Self::Output {
        &self.0[y][x]
    }
}

impl<const N: usize> From<&str> for RiskLevelMap<N> {
    fn from(input: &str) -> Self {
        let mut map = [[0; N]; N];

        for (y, line) in input.lines().enumerate().take(N) {
            for (x, c) in line.chars().enumerate().take(N) {
                map[y][x] = c.to_digit(10).unwrap() as u8;
            }
        }

        Self(map)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: RiskLevelMap<10> = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            .into();
    }

    #[test]
    fn lowest_path_risk_should_be_40_for_sample() {
        let result = SAMPLE.lowest_risk_from_start_to_end();

        assert_eq!(result, 40);
    }

    #[test]
    fn lowest_path_risk_should_be_315_for_full_sample() {
        let result = SAMPLE.grow::<50>().lowest_risk_from_start_to_end();

        assert_eq!(result, 315);
    }

    #[test]
    fn test() {
        let map: RiskLevelMap<5> = r"99999
19999
19111
11191
99991"
            .into();

        let risk = map.lowest_risk_from_start_to_end();

        assert_eq!(risk, 10);
    }
}
