use itertools::Itertools;
use lazy_static::lazy_static;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: Vec<String> = input::read_lines_from_file("day10");
}

pub struct Day10;

impl Puzzle for Day10 {
    fn number(&self) -> u8 {
        10
    }

    fn part_one(&self) -> String {
        format!(
            "Total syntax error score: {}",
            compute_syntax_error_score(&INPUT),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Middle autocomplete score: {}",
            compute_middle_autocomplete_score(&INPUT),
        )
    }
}

fn compute_syntax_error_score(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| parse_line_chunks(line))
        .filter_map(Result::err)
        .filter_map(|e| e.syntax_error_score())
        .sum()
}

fn compute_middle_autocomplete_score(input: &[String]) -> u64 {
    let scores = input
        .iter()
        .map(|line| parse_line_chunks(line))
        .filter_map(Result::err)
        .filter_map(|e| e.autocomplete_score())
        .sorted()
        .collect::<Vec<_>>();
    scores[scores.len() / 2]
}

fn parse_line_chunks(line: &str) -> Result<Vec<Chunk>, ChunkParsingError> {
    let mut chunks = Vec::new();

    let mut opened_chunks = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if let Some(opening_char) = parse_opening_char(c) {
            opened_chunks.push(Chunk::new(opening_char));
        } else if let Some(chunk) = opened_chunks.pop() {
            if c == chunk.closing_char() {
                if let Some(parent_chunk) = opened_chunks.last_mut() {
                    parent_chunk.add_child(chunk);
                } else {
                    chunks.push(chunk);
                }
            } else {
                return Err(ChunkParsingError::wrong_closing(
                    i + 1,
                    chunk.closing_char(),
                    c,
                ));
            }
        } else {
            return Err(ChunkParsingError::wrong_opening(i + 1, c));
        }
    }

    if !opened_chunks.is_empty() {
        Err(ChunkParsingError::Incomplete { opened_chunks })
    } else {
        Ok(chunks)
    }
}

fn parse_opening_char(c: char) -> Option<OpeningChar> {
    match c {
        '(' => Some(OpeningChar::Parenthesis),
        '[' => Some(OpeningChar::Bracket),
        '{' => Some(OpeningChar::Brace),
        '<' => Some(OpeningChar::Chevron),
        _ => None,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OpeningChar {
    Parenthesis,
    Bracket,
    Brace,
    Chevron,
}

impl OpeningChar {
    fn closing_char(&self) -> char {
        match self {
            OpeningChar::Parenthesis => ')',
            OpeningChar::Bracket => ']',
            OpeningChar::Brace => '}',
            OpeningChar::Chevron => '>',
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Chunk {
    opening_char: OpeningChar,
    children: Vec<Chunk>,
}

impl Chunk {
    fn new(opening_char: OpeningChar) -> Self {
        Self {
            opening_char,
            children: Vec::new(),
        }
    }

    fn closing_char(&self) -> char {
        self.opening_char.closing_char()
    }

    fn add_child(&mut self, child: Chunk) {
        self.children.push(child)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ChunkParsingError {
    Incomplete {
        opened_chunks: Vec<Chunk>,
    },
    WrongOpeningChar {
        column: usize,
        actual: char,
    },
    WrongClosingChar {
        column: usize,
        expected: char,
        actual: char,
    },
}

impl ChunkParsingError {
    fn syntax_error_score(&self) -> Option<u32> {
        match self {
            Self::Incomplete { .. } => None,
            Self::WrongOpeningChar { actual, .. } | Self::WrongClosingChar { actual, .. } => {
                Some(char_syntax_error_score(*actual))
            }
        }
    }

    fn autocomplete_score(&self) -> Option<u64> {
        match self {
            Self::Incomplete { opened_chunks } => Some(
                opened_chunks
                    .iter()
                    .rev()
                    .map(Chunk::closing_char)
                    .map(char_autocomplete_score)
                    .fold(0, |total, score| total * 5 + score),
            ),
            _ => None,
        }
    }
}

fn char_syntax_error_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn char_autocomplete_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

impl ChunkParsingError {
    fn wrong_opening(column: usize, actual: char) -> Self {
        Self::WrongOpeningChar { column, actual }
    }

    fn wrong_closing(column: usize, expected: char, actual: char) -> Self {
        Self::WrongClosingChar {
            column,
            expected,
            actual,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: Vec<String> = vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];
    }

    #[test]
    fn parsing_of_first_sample_line_should_return_incomplete() {
        let result = parse_line_chunks(&SAMPLE[0]);

        assert!(matches!(result, Err(ChunkParsingError::Incomplete { .. })));
    }

    #[test]
    fn parsing_of_third_sample_line_should_fail_on_colum_13() {
        let result = parse_line_chunks(&SAMPLE[2]);

        assert_eq!(result, Err(ChunkParsingError::wrong_closing(13, ']', '}')));
    }

    #[test]
    fn syntax_error_score_for_sample_should_be_26397() {
        assert_eq!(compute_syntax_error_score(&SAMPLE), 26397);
    }

    #[test]
    fn autocomplete_score_for_first_sample_line_should_be_288957() {
        let err = parse_line_chunks(&SAMPLE[0]).err().unwrap();

        assert_eq!(err.autocomplete_score(), Some(288957));
    }

    #[test]
    fn middle_autocomplete_score_for_sample_should_be_288957() {
        assert_eq!(compute_middle_autocomplete_score(&SAMPLE), 288957);
    }
}
