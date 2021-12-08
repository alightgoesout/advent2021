use lazy_static::lazy_static;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: Vec<u8> = input::read_lines_from_file::<String>("day06")[0]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
}

pub struct Day6;

impl Puzzle for Day6 {
    fn number(&self) -> u8 {
        6
    }

    fn part_one(&self) -> String {
        let mut school = LanternfishSchool::new(&INPUT);
        school.next_days(80);
        format!("Number of lanternfish after 80 days: {}", school.len())
    }

    fn part_two(&self) -> String {
        let mut school = LanternfishSchool::new(&INPUT);
        school.next_days(256);
        format!("Number of lanternfish after 256 days: {}", school.len())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct LanternfishSchool([usize; 9]);

impl LanternfishSchool {
    fn new(fishes: &[u8]) -> Self {
        let mut school = [0; 9];
        for (i, s) in school.iter_mut().enumerate() {
            *s = fishes.iter().filter(|&&n| n == i as u8).count()
        }
        Self(school)
    }

    fn next_day(&mut self) {
        let nb_reproducing = self.0[0];
        for i in 1..9 {
            self.0[i - 1] = self.0[i];
        }
        self.0[6] += nb_reproducing;
        self.0[8] = nb_reproducing;
    }

    fn next_days(&mut self, days: u32) {
        (0..days).for_each(|_| self.next_day())
    }

    fn len(&self) -> usize {
        self.0.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: Vec<u8> = vec![3, 4, 3, 1, 2];
    }

    #[test]
    fn number_of_fishes_after_80_days_should_be_5934_for_sample() {
        let mut school = LanternfishSchool::new(&SAMPLE);

        school.next_days(80);

        assert_eq!(school.len(), 5934);
    }
}
