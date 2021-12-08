use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: Vec<NoteLine> = input::read_lines_from_file("day08");
}

pub struct Day8;

impl Puzzle for Day8 {
    fn number(&self) -> u8 {
        8
    }

    fn part_one(&self) -> String {
        format!(
            "Number of 1, 4, 7, and 8 in output: {}",
            count_1_4_7_8s_in_output(&INPUT)
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Sum of all output values: {}",
            INPUT.iter().map(NoteLine::decode).sum::<u32>()
        )
    }
}

fn count_1_4_7_8s_in_output(input: &[NoteLine]) -> usize {
    static UNIQUE_SIZES: [usize; 4] = [2, 3, 4, 7];
    input
        .iter()
        .flat_map(|line| line.output.iter())
        .filter(|digit| UNIQUE_SIZES.contains(&digit.len()))
        .count()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            'g' => Ok(Self::G),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct DisplayDigit(HashSet<Segment>);

impl DisplayDigit {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn contains(&self, other: &DisplayDigit) -> bool {
        other.0.iter().all(|segment| self.0.contains(segment))
    }
}

impl FromStr for DisplayDigit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars().map(Segment::try_from).collect::<Result<_, _>>()?,
        ))
    }
}

impl PartialEq for DisplayDigit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for DisplayDigit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0
            .iter()
            .sorted()
            .for_each(|segment| segment.hash(state));
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NoteLine {
    patterns: Vec<DisplayDigit>,
    output: Vec<DisplayDigit>,
}

impl NoteLine {
    fn decode(&self) -> u32 {
        let mapping = pattern_to_digits_mapping(&self.patterns);
        self.output
            .iter()
            .map(|digit| mapping[&digit])
            .fold(0, |r, d| r * 10 + d)
    }
}

fn pattern_to_digits_mapping(patterns: &[DisplayDigit]) -> HashMap<&DisplayDigit, u32> {
    let one = patterns.iter().find(|digit| digit.len() == 2).unwrap();
    let four = patterns.iter().find(|digit| digit.len() == 4).unwrap();
    let seven = patterns.iter().find(|digit| digit.len() == 3).unwrap();
    let eight = patterns.iter().find(|digit| digit.len() == 7).unwrap();

    let six_length = patterns
        .iter()
        .filter(|digit| digit.len() == 6)
        .collect::<Vec<_>>();
    let nine = *six_length
        .iter()
        .find(|digit| digit.contains(four))
        .unwrap();
    let zero = *six_length
        .iter()
        .find(|digit| **digit != nine && digit.contains(one))
        .unwrap();
    let six = *six_length
        .iter()
        .find(|digit| **digit != zero && **digit != nine)
        .unwrap();

    let five_length = patterns
        .iter()
        .filter(|digit| digit.len() == 5)
        .collect::<Vec<_>>();
    let three = *five_length
        .iter()
        .find(|digit| digit.contains(one))
        .unwrap();
    let five = *five_length
        .iter()
        .find(|digit| **digit != three && nine.contains(digit))
        .unwrap();
    let two = *five_length
        .iter()
        .find(|digit| **digit != three && **digit != five)
        .unwrap();

    [
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ]
    .into()
}

impl FromStr for NoteLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns, output) = s.split_once(" | ").ok_or(())?;
        Ok(Self {
            patterns: parse_display_digits(patterns)?,
            output: parse_display_digits(output)?,
        })
    }
}

fn parse_display_digits(s: &str) -> Result<Vec<DisplayDigit>, ()> {
    s.trim().split(' ').map(DisplayDigit::from_str).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    lazy_static! {
        static ref SAMPLE_LINES: Vec<NoteLine> = input::read_lines(SAMPLE.as_bytes());
    }

    #[test]
    fn decode_should_return_5353_for_first_example() {
        let line: NoteLine =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
                .parse()
                .unwrap();

        assert_eq!(line.decode(), 5353);
    }

    #[test]
    fn sum_of_all_outputs_for_sample_should_be_61229() {
        let sum: u32 = SAMPLE_LINES.iter().map(NoteLine::decode).sum();

        assert_eq!(sum, 61229);
    }
}
