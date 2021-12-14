use itertools::{Itertools, MinMaxResult};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

use super::{input, Puzzle};

const TEMPLATE: &str = "SVCHKVFKCSHVFNBKKPOC";

lazy_static! {
    static ref POLYMERIZATION: Polymerization =
        input::read_lines_from_file::<InsertionRule>("day14").into();
}

pub struct Day14;

impl Puzzle for Day14 {
    fn number(&self) -> u8 {
        14
    }

    fn part_one(&self) -> String {
        let result = POLYMERIZATION.apply_n_times(TEMPLATE, 10);
        let count = PolymerCount::new(&result).unwrap();
        format!(
            "Difference between most present element ({}) and least present element ({}): {}",
            count.max_char,
            count.min_char,
            count.max_count - count.min_count
        )
    }

    fn part_two(&self) -> String {
        "".to_string()
    }
}

struct PolymerCount {
    min_char: char,
    min_count: usize,
    max_char: char,
    max_count: usize,
}

impl PolymerCount {
    fn new(polymer: &str) -> Result<Self, ()> {
        let minmax = polymer
            .chars()
            .counts()
            .into_iter()
            .minmax_by(|(_, n1), (_, n2)| n1.cmp(n2));
        match minmax {
            MinMaxResult::MinMax(min, max) => Ok(Self {
                min_char: min.0,
                min_count: min.1,
                max_char: max.0,
                max_count: max.1,
            }),
            _ => Err(()),
        }
    }
}

struct Polymerization(HashMap<(char, char), char>);

impl Polymerization {
    fn apply(&self, template: &str) -> String {
        let mut result: String = template
            .chars()
            .tuple_windows()
            .flat_map(|pair| match self.0.get(&pair) {
                Some(c) => vec![pair.0, *c],
                None => vec![pair.0],
            })
            .collect();
        if let Some(last) = template.chars().last() {
            result.push(last)
        }
        result
    }

    fn apply_n_times(&self, template: &str, n: usize) -> String {
        (0..n).fold(template.to_string(), |s, _| self.apply(&s))
    }
}

impl<T: IntoIterator<Item = InsertionRule>> From<T> for Polymerization {
    fn from(rules: T) -> Self {
        Self(
            rules
                .into_iter()
                .map(|rule| {
                    (
                        (rule.first_element, rule.second_element),
                        rule.inserted_element,
                    )
                })
                .collect(),
        )
    }
}

struct InsertionRule {
    first_element: char,
    second_element: char,
    inserted_element: char,
}

impl FromStr for InsertionRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        Ok(Self {
            first_element: bytes[0] as char,
            second_element: bytes[1] as char,
            inserted_element: bytes[6] as char,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: Polymerization = input::read_lines::<InsertionRule, _>(
            "CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"
            .as_bytes()
        )
        .into();
    }

    #[test]
    fn sample_polymerization_should_return_ncnbchb_when_applied_to_nncb_once() {
        assert_eq!(&SAMPLE.apply("NNCB"), "NCNBCHB");
    }

    #[test]
    fn sample_polymerization_should_return_nbccnbbbcbhcb_when_applied_to_nncb_twice() {
        assert_eq!(&SAMPLE.apply_n_times("NNCB", 2), "NBCCNBBBCBHCB");
    }

    #[test]
    fn sample_polymerization_should_return_nbbbcnccnbbnbnbbchbhhbchbb_when_applied_to_nncb_thrice()
    {
        assert_eq!(
            &SAMPLE.apply_n_times("NNCB", 3),
            "NBBBCNCCNBBNBNBBCHBHHBCHB"
        );
    }
}
