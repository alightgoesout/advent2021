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
        let min_max = POLYMERIZATION.apply(TEMPLATE, 10);
        format!(
            "Difference between most present element ({}) and least present element ({}) after 10 iterations: {}",
            min_max.max_element,
            min_max.min_element,
            min_max.max_count - min_max.min_count
        )
    }

    fn part_two(&self) -> String {
        let min_max = POLYMERIZATION.apply(TEMPLATE, 40);
        format!(
            "Difference between most present element ({}) and least present element ({}) after 40 iterations: {}",
            min_max.max_element,
            min_max.min_element,
            min_max.max_count - min_max.min_count
        )
    }
}

/*struct PolymerCount {
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
}*/

struct Polymerization(HashMap<(char, char), char>);

impl Polymerization {
    fn apply(&self, template: &str, n: usize) -> ElementMinMax {
        let mut pair_count = PairCount::from(template);

        for _ in 0..n {
            let mut new_pair_count = PairCount::new();
            for ((c1, c2), count) in pair_count {
                if let Some(c) = self.0.get(&(c1, c2)) {
                    new_pair_count.add_pair(c1, *c, count);
                    new_pair_count.add_pair(*c, c2, count);
                } else {
                    new_pair_count.add_pair(c1, c2, count);
                }
            }
            pair_count = new_pair_count;
        }

        pair_count.count_elements(template.chars().next()).unwrap()
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct PairCount(HashMap<(char, char), usize>);

impl PairCount {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn from(template: &str) -> Self {
        Self(template.chars().tuple_windows().counts())
    }

    fn add_pair(&mut self, c1: char, c2: char, count: usize) {
        let current_count = self.0.entry((c1, c2)).or_insert(0);
        *current_count += count;
    }

    fn count_elements(&self, first_element: Option<char>) -> Result<ElementMinMax, ()> {
        let mut all_count = first_element
            .map(|c| [(c, 1)].into())
            .unwrap_or_else(HashMap::new);

        for ((_, c), n) in &self.0 {
            let count = all_count.entry(*c).or_insert(0);
            *count += n;
        }

        let minmax = all_count
            .into_iter()
            .minmax_by(|(_, n1), (_, n2)| n1.cmp(n2));
        match minmax {
            MinMaxResult::MinMax(min, max) => Ok(ElementMinMax {
                min_element: min.0,
                min_count: min.1,
                max_element: max.0,
                max_count: max.1,
            }),
            _ => Err(()),
        }
    }
}

impl IntoIterator for PairCount {
    type Item = ((char, char), usize);
    type IntoIter = std::collections::hash_map::IntoIter<(char, char), usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct ElementMinMax {
    min_element: char,
    min_count: usize,
    max_element: char,
    max_count: usize,
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
    fn test_sample_after_10_iterations() {
        let min_max = SAMPLE.apply("NNCB", 10);

        assert_eq!(
            min_max,
            ElementMinMax {
                min_element: 'H',
                min_count: 161,
                max_element: 'B',
                max_count: 1749,
            }
        )
    }
}
