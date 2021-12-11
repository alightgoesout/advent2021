use itertools::Itertools;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use super::Puzzle;

const INPUT: &str = r"4764745784
4643457176
8322628477
7617152546
6137518165
1556723176
2187861886
2553422625
4817584638
3754285662";

pub struct Day11;

impl Puzzle for Day11 {
    fn number(&self) -> u8 {
        11
    }

    fn part_one(&self) -> String {
        let mut octopuses: OctopusGroup<10> = INPUT.parse().unwrap();
        let flashes = octopuses.next_steps(100);
        format!("Total flashes after 100 steps: {}", flashes)
    }

    fn part_two(&self) -> String {
        let mut octopuses: OctopusGroup<10> = INPUT.parse().unwrap();
        let step = find_first_synchronized_flashing_step(&mut octopuses);
        format!("First synchronized flashing step: {}", step)
    }
}

fn find_first_synchronized_flashing_step<const N: usize>(octopuses: &mut OctopusGroup<N>) -> usize {
    let mut i = 0;
    loop {
        i += 1;
        let flashes = octopuses.next_step();
        if flashes == N * N {
            break i;
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct OctopusGroup<const N: usize>([[u8; N]; N]);

impl<const N: usize> OctopusGroup<N> {
    fn next_step(&mut self) -> usize {
        let flashing = (0..N)
            .cartesian_product(0..N)
            .flat_map(|(x, y)| self.increase(x, y))
            .collect::<Vec<_>>();
        flashing.iter().for_each(|&(x, y)| self.0[y][x] = 0);
        flashing.len()
    }

    fn next_steps(&mut self, n: usize) -> usize {
        (0..n).map(|_| self.next_step()).sum()
    }

    fn increase(&mut self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        self.0[y][x] += 1;
        self.flash(x, y)
    }

    fn flash(&mut self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut flashing: HashSet<(usize, usize)> = HashSet::new();
        if self.0[y][x] == 10 {
            flashing.insert((x, y));
            let x_range = x.saturating_sub(1)..=min(x + 1, N - 1);
            let y_range = y.saturating_sub(1)..=min(y + 1, N - 1);
            for (x, y) in x_range.cartesian_product(y_range) {
                if self.0[y][x] < 10 {
                    flashing.extend(self.increase(x, y));
                }
            }
        }
        flashing
    }
}

impl<const N: usize> Display for OctopusGroup<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..N {
            for x in 0..N {
                write!(f, "{}", self.0[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const N: usize> FromStr for OctopusGroup<N> {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut octopuses = [[0; N]; N];
        for (y, line) in input.lines().take(10).enumerate() {
            for (x, c) in line.chars().take(10).enumerate() {
                octopuses[y][x] = c.to_digit(10).unwrap() as u8;
            }
        }
        Ok(Self(octopuses))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_SAMPLE: &str = r"11111
19991
19191
19991
11111";

    const SAMPLE: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_small_sample_after_step_one() {
        let mut octopuses: OctopusGroup<5> = SMALL_SAMPLE.parse().unwrap();

        octopuses.next_step();

        assert_eq!(
            octopuses.to_string().as_str(),
            r"34543
40004
50005
40004
34543
"
        );
    }

    #[test]
    fn test_small_sample_after_step_two() {
        let mut octopuses: OctopusGroup<5> = SMALL_SAMPLE.parse().unwrap();

        octopuses.next_step();
        octopuses.next_step();

        assert_eq!(
            octopuses.to_string().as_str(),
            r"45654
51115
61116
51115
45654
"
        );
    }

    #[test]
    fn test_sample_after_step_one() {
        let mut octopuses: OctopusGroup<10> = SAMPLE.parse().unwrap();

        octopuses.next_step();

        assert_eq!(
            octopuses.to_string().as_str(),
            r"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637
"
        );
    }

    #[test]
    fn test_sample_after_step_two() {
        let mut octopuses: OctopusGroup<10> = SAMPLE.parse().unwrap();

        octopuses.next_step();
        octopuses.next_step();

        assert_eq!(
            octopuses.to_string().as_str(),
            r"8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848
"
        );
    }

    #[test]
    fn total_flashes_after_100_steps_for_sample_should_be_1656() {
        let mut octopuses: OctopusGroup<10> = SAMPLE.parse().unwrap();

        let flashes = octopuses.next_steps(100);

        assert_eq!(flashes, 1656);
    }

    #[test]
    fn first_synchronized_flashing_for_sample_should_be_at_step_195() {
        let mut octopuses: OctopusGroup<10> = SAMPLE.parse().unwrap();

        let step = find_first_synchronized_flashing_step(&mut octopuses);

        assert_eq!(step, 195);
    }
}
