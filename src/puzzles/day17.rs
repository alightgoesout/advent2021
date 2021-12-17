use itertools::Itertools;
use std::cmp::{max, min};
use std::ops::{AddAssign, RangeInclusive};

use super::Puzzle;

const TARGET_X: RangeInclusive<i32> = 265..=287;
const TARGET_Y: RangeInclusive<i32> = -103..=-58;

pub struct Day17;

impl Puzzle for Day17 {
    fn number(&self) -> u8 {
        17
    }

    fn part_one(&self) -> String {
        format!(
            "Maximum height while reaching target: {}",
            triangular_number(*TARGET_Y.start()),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Number of velocities reaching target: {}",
            list_all_hitting_velocities(TARGET_X, TARGET_Y).len(),
        )
    }
}

fn triangular_number(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn list_all_hitting_velocities(
    target_x: RangeInclusive<i32>,
    target_y: RangeInclusive<i32>,
) -> Vec<Velocity> {
    (0..=*target_x.end())
        .cartesian_product(*target_y.start()..=target_y.start().abs())
        .map(|(x, y)| Velocity { x, y })
        .filter(|velocity| check_hit(*velocity, &target_x, &target_y))
        .collect()
}

fn check_hit(
    mut velocity: Velocity,
    target_x: &RangeInclusive<i32>,
    target_y: &RangeInclusive<i32>,
) -> bool {
    let mut position = Position { x: 0, y: 0 };

    while (position.x < *target_x.start() && velocity.x > 0) || position.y > *target_y.end() {
        position += velocity;
        velocity.next_step();
    }

    target_x.contains(&position.x) && target_y.contains(&position.y)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn next_step(&mut self) {
        self.x = if self.x >= 0 {
            max(0, self.x - 1)
        } else {
            min(0, self.x + 1)
        };
        self.y -= 1;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, velocity: Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_hit_for_6_9_on_sample_should_be_true() {
        assert!(check_hit(Velocity { x: 6, y: 9 }, &(20..=30), &(-10..=-5)));
    }

    #[test]
    fn number_of_hitting_velocities_for_sample_should_be_112() {
        let velocities = list_all_hitting_velocities(20..=30, -10..=-5);

        assert_eq!(velocities.len(), 112);
    }
}
