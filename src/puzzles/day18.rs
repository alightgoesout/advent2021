use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::str::FromStr;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: Vec<Number> = input::read_lines_from_file("day18");
}

pub struct Day18;

impl Puzzle for Day18 {
    fn number(&self) -> u8 {
        18
    }

    fn part_one(&self) -> String {
        let result = sum_numbers(INPUT.clone());
        format!("Magnitude of the final number: {}", result.magnitude())
    }

    fn part_two(&self) -> String {
        let result = INPUT
            .iter()
            .cartesian_product(INPUT.iter())
            .map(|(n1, n2)| n1.clone() + n2.clone())
            .map(|n| n.magnitude())
            .max()
            .unwrap();
        format!("Maximum magnitude of sum of two numbers: {}", result)
    }
}

fn sum_numbers(numbers: Vec<Number>) -> Number {
    numbers.into_iter().reduce(|n1, n2| n1 + n2).unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Number {
    Value(u32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    pub fn pair(left: impl Into<Number>, right: impl Into<Number>) -> Self {
        Self::Pair(Box::new(left.into()), Box::new(right.into()))
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            Self::Value(value) => *value,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    pub fn reduce(&mut self) {
        loop {
            if let Some((left_value, number, right_value)) = self.find_pair_to_explode() {
                number.explode(left_value, right_value);
            } else if let Some(number) = self.find_number_to_split() {
                number.split();
            } else {
                break;
            }
        }
    }

    fn find_pair_to_explode(
        &mut self,
    ) -> Option<(Option<&mut u32>, &mut Number, Option<&mut u32>)> {
        let mut to_visit = vec![VecDeque::from([self])];
        let mut last_value = None;
        let mut result = None;
        while let Some(numbers) = to_visit.last_mut() {
            if let Some(number) = numbers.pop_front() {
                if let Self::Value(value) = number {
                    if let Some((left, to_explode)) = result {
                        return Some((left, to_explode, Some(value)));
                    } else {
                        last_value.replace(value);
                    }
                } else if to_visit.len() == 5 && result.is_none() {
                    result = Some((std::mem::take(&mut last_value), number))
                } else if let Self::Pair(left, right) = number {
                    to_visit.push(VecDeque::from([left.as_mut(), right.as_mut()]));
                }
            } else {
                to_visit.pop();
            }
        }
        result.map(|(left, to_explode)| (left, to_explode, None))
    }

    fn find_number_to_split(&mut self) -> Option<&mut Number> {
        let mut to_visit = vec![VecDeque::from([self])];
        while let Some(numbers) = to_visit.last_mut() {
            if let Some(number) = numbers.pop_front() {
                match number {
                    Self::Value(value) => {
                        if *value >= 10 {
                            return Some(number);
                        }
                    }
                    Self::Pair(left, right) => {
                        to_visit.push(VecDeque::from([left.as_mut(), right.as_mut()]));
                    }
                }
            } else {
                to_visit.pop();
            }
        }
        None
    }

    fn explode(&mut self, left_value: Option<&mut u32>, right_value: Option<&mut u32>) {
        let old = std::mem::replace(self, Self::Value(0));
        if let Self::Pair(left, right) = old {
            if let (Some(left_value), Number::Value(value)) = (left_value, *left) {
                *left_value += value;
            }
            if let (Some(right_value), Number::Value(value)) = (right_value, *right) {
                *right_value += value;
            }
        }
    }

    fn split(&mut self) {
        if let Self::Value(v) = *self {
            if v >= 10 {
                *self = Self::pair(v / 2, (v - 1) / 2 + 1)
            }
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut number = Self::pair(self, rhs);
        number.reduce();
        number
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self::Value(value)
    }
}

impl From<Vec<Number>> for Number {
    fn from(mut numbers: Vec<Number>) -> Self {
        Self::pair(numbers.remove(0), numbers.remove(0))
    }
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers: Vec<Vec<Number>> = Vec::new();

        for c in s.chars() {
            match c {
                '[' => {
                    numbers.push(Vec::new());
                }
                ']' => {
                    let pair = numbers.pop().unwrap().into();
                    if let Some(parent) = numbers.last_mut() {
                        parent.push(pair);
                    } else {
                        return Ok(pair);
                    }
                }
                ',' => {}
                _ => {
                    let value = c.to_digit(10).unwrap();
                    numbers.last_mut().unwrap().push(value.into())
                }
            }
        }

        Err(())
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref LARGE_SAMPLE: Vec<Number> = input::read_lines(
            r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
                .as_bytes()
        );
    }

    #[test]
    fn parse_a_pair_of_values() {
        let number: Number = "[1,2]".parse().unwrap();

        assert_eq!(number, Number::pair(1, 2));
    }

    #[test]
    fn parse_complex_sample() {
        let number: Number = "[[[[[9,8],1],2],3],4]".parse().unwrap();

        assert_eq!(
            number,
            Number::pair(
                Number::pair(Number::pair(Number::pair(Number::pair(9, 8), 1), 2), 3),
                4
            )
        );
    }

    #[test]
    fn first_addition_sample() {
        let numbers = vec![
            "[1,1]".parse().unwrap(),
            "[2,2]".parse().unwrap(),
            "[3,3]".parse().unwrap(),
            "[4,4]".parse().unwrap(),
        ];

        let result = sum_numbers(numbers);

        assert_eq!(result, "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap());
    }

    #[test]
    fn test_sum_large_sample() {
        let result = sum_numbers(LARGE_SAMPLE.clone());

        assert_eq!(
            &result.to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }
}
