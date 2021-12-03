use lazy_static::lazy_static;

use super::input;
use super::Puzzle;

lazy_static! {
    static ref INPUT: Vec<String> = input::read_lines_from_file("day03");
}

pub struct Day3;

impl Puzzle for Day3 {
    fn number(&self) -> u8 {
        3
    }

    fn part_one(&self) -> String {
        let digit_count = DigitCount::count(&INPUT);
        format!(
            "Power consumption: {}",
            digit_count.gamma_rate() * digit_count.epsilon_rate()
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Life support rating: {}",
            oxygen_generator_rating(&INPUT) * co2_scrubber_rating(&INPUT)
        )
    }
}

struct DigitCount(Vec<(u32, u32)>);

impl DigitCount {
    fn count(input: &[String]) -> Self {
        let size = input.get(0).map(|s| s.len()).unwrap_or(0);
        let initial_count = Self((0..size).map(|_| (0, 0)).collect());
        input
            .iter()
            .fold(initial_count, |count, number| count.add(number))
    }

    fn add(self, number: &str) -> Self {
        let count = self
            .0
            .into_iter()
            .zip(number.chars())
            .map(|((z, o), c)| match c {
                '0' => (z + 1, o),
                _ => (z, o + 1),
            })
            .collect();
        Self(count)
    }

    fn most_common_bits(&self) -> Vec<u32> {
        self.0
            .iter()
            .map(|(z, o)| if z > o { 0 } else { 1 })
            .collect()
    }

    fn least_common_bits(&self) -> Vec<u32> {
        self.0
            .iter()
            .map(|(z, o)| if o > z { 0 } else { 1 })
            .collect()
    }

    fn gamma_rate(&self) -> u32 {
        self.most_common_bits().to_decimal()
    }

    fn epsilon_rate(&self) -> u32 {
        self.least_common_bits().to_decimal()
    }
}

fn oxygen_generator_rating(input: &[String]) -> u32 {
    find_number(input, |(z, o)| if z > o { b'0' } else { b'1' })
}

fn co2_scrubber_rating(input: &[String]) -> u32 {
    find_number(input, |(z, o)| if z <= o { b'0' } else { b'1' })
}

fn find_number<F: Fn((u32, u32)) -> u8>(input: &[String], get_bit: F) -> u32 {
    let mut position = 0;
    let mut numbers = input.to_vec();
    while numbers.len() > 1 {
        let bits_count = count_bits(position, &numbers);
        numbers = filter(get_bit(bits_count), position, numbers);
        position += 1;
    }
    numbers[0].to_decimal()
}

fn count_bits(position: usize, numbers: &[String]) -> (u32, u32) {
    numbers
        .iter()
        .fold((0, 0), |(z, o), n| match n.as_bytes()[position] {
            b'0' => (z + 1, o),
            _ => (z, o + 1),
        })
}

fn filter(bit: u8, position: usize, numbers: Vec<String>) -> Vec<String> {
    numbers
        .into_iter()
        .filter(|n| n.as_bytes()[position] == bit)
        .collect()
}

trait ToDecimal {
    fn to_decimal(&self) -> u32;
}

impl ToDecimal for Vec<u32> {
    fn to_decimal(&self) -> u32 {
        to_decimal(self.iter().copied())
    }
}

impl ToDecimal for String {
    fn to_decimal(&self) -> u32 {
        to_decimal(self.chars().map(|c| match c {
            '0' => 0,
            _ => 1,
        }))
    }
}

fn to_decimal<T: Iterator<Item = u32>>(iter: T) -> u32 {
    iter.reduce(|g, d| g * 2 + d).unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
    }

    #[test]
    fn gamma_rate_should_be_22_for_sample() {
        assert_eq!(DigitCount::count(&SAMPLE).gamma_rate(), 22);
    }

    #[test]
    fn epsilon_rate_should_be_9_for_sample() {
        assert_eq!(DigitCount::count(&SAMPLE).epsilon_rate(), 9);
    }

    #[test]
    fn oxygen_generator_rating_should_be_23_for_sample() {
        assert_eq!(oxygen_generator_rating(&SAMPLE), 23);
    }

    #[test]
    fn co2_scrubber_rating_should_be_10_for_sample() {
        assert_eq!(co2_scrubber_rating(&SAMPLE), 10);
    }
}
