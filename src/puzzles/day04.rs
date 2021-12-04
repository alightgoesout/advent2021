use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::min;
use std::fmt::{Display, Formatter};

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: Vec<String> = input::read_lines_from_file("day04");
    static ref DRAWN_NUMBERS: Vec<u32> = read_drawn_numbers(&INPUT[0]);
    static ref BOARDS: Vec<Board<5>> = read_boards(&INPUT[1..]);
}

pub struct Day4;

impl Puzzle for Day4 {
    fn number(&self) -> u8 {
        4
    }

    fn part_one(&self) -> String {
        let (winning_board, last_drawn_number) =
            play_to_first_win(&DRAWN_NUMBERS, &BOARDS).unwrap();
        let final_score = winning_board.unmarked_numbers().iter().sum::<u32>() * last_drawn_number;
        format!("Score of first winning board: {}", final_score)
    }

    fn part_two(&self) -> String {
        let (winning_board, last_drawn_number) = play_to_end(&DRAWN_NUMBERS, &BOARDS).unwrap();
        let final_score = winning_board.unmarked_numbers().iter().sum::<u32>() * last_drawn_number;
        format!("Score of last winning board: {}", final_score)
    }
}

fn read_drawn_numbers(numbers: &str) -> Vec<u32> {
    numbers.split(',').map(|s| s.parse().unwrap()).collect()
}

fn read_boards<const N: usize>(input: &[String]) -> Vec<Board<N>> {
    let mut boards = Vec::new();
    for i in 0..(input.len() / N) {
        boards.push(Board::new(&input[(i * N)..((i + 1) * N)]))
    }
    boards
}

fn play_to_first_win<const N: usize>(
    drawn_numbers: &[u32],
    boards: &[Board<N>],
) -> Option<(Board<N>, u32)> {
    let mut boards = boards.iter().copied().collect::<Vec<_>>();
    for number in drawn_numbers {
        boards.iter_mut().for_each(|board| board.mark(*number));
        if let Some(winning_board) = boards.iter().find(|board| board.has_won()) {
            return Some((*winning_board, *number));
        }
    }
    None
}

fn play_to_end<const N: usize>(
    drawn_numbers: &[u32],
    boards: &[Board<N>],
) -> Option<(Board<N>, u32)> {
    let mut result = None;
    let mut boards = boards.iter().copied().collect::<Vec<_>>();
    for number in drawn_numbers {
        boards.iter_mut().for_each(|board| board.mark(*number));
        let (winning_boards, remaining_boards) =
            boards.into_iter().partition(|board| board.has_won());
        boards = remaining_boards;
        if let Some(winning_board) = winning_boards.first() {
            result = Some((*winning_board, *number))
        }
    }
    result
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct Square {
    number: u32,
    marked: bool,
}

impl Square {
    #[cfg(test)]
    fn new(number: u32) -> Self {
        Self {
            number,
            marked: false,
        }
    }

    #[cfg(test)]
    fn marked(number: u32) -> Self {
        Self {
            number,
            marked: true,
        }
    }

    fn mark(&mut self) {
        self.marked = true
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.marked {
            write!(f, "[{: >2}]", self.number)
        } else {
            write!(f, " {: >2} ", self.number)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Board<const N: usize>([[Square; N]; N]);

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
}

impl<const N: usize> Board<N> {
    fn new(numbers: &[String]) -> Self {
        let mut board = [[Square::default(); N]; N];
        for y in 0..min(numbers.len(), N) {
            for (x, s) in WHITESPACE.split(&numbers[y]).take(N).enumerate() {
                board[y][x].number = s.parse().unwrap();
            }
        }
        Self(board)
    }

    fn mark(&mut self, number: u32) {
        self.0
            .iter_mut()
            .flat_map(|line| line.iter_mut())
            .filter(|square| square.number == number)
            .for_each(Square::mark);
    }

    fn unmarked_numbers(&self) -> Vec<u32> {
        self.0
            .iter()
            .flat_map(|line| {
                line.iter()
                    .filter_map(|square| (!square.marked).then(|| square.number))
            })
            .collect()
    }

    fn has_won(&self) -> bool {
        self.0
            .iter()
            .any(|line| line.iter().all(|square| square.marked))
            || (0..N).any(|x| (0..N).all(|y| self.0[y][x].marked))
    }
}

impl<const N: usize> Display for Board<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.0 {
            let formatted_line = line.iter().map(ToString::to_string).join(" ");
            writeln!(f, "{}", formatted_line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::puzzles::input::read_lines;

    #[test]
    fn board_has_won_should_return_true_when_a_line_is_fully_marked() {
        let board = Board([
            [Square::new(19), Square::new(47), Square::new(13)],
            [Square::marked(69), Square::marked(53), Square::marked(15)],
            [Square::new(21), Square::new(39), Square::new(70)],
        ]);

        assert!(board.has_won())
    }

    #[test]
    fn board_has_won_should_return_true_when_a_column_is_fully_marked() {
        let board = Board([
            [Square::marked(19), Square::new(47), Square::new(13)],
            [Square::marked(69), Square::new(53), Square::new(15)],
            [Square::marked(21), Square::new(39), Square::new(70)],
        ]);

        assert!(board.has_won())
    }

    #[test]
    fn board_has_won_should_return_false_when_no_column_or_line_is_fully_marked() {
        let board = Board([
            [Square::marked(19), Square::new(47), Square::marked(13)],
            [Square::new(69), Square::new(53), Square::new(15)],
            [Square::marked(21), Square::new(39), Square::new(70)],
        ]);

        assert!(!board.has_won())
    }

    lazy_static! {
        static ref SAMPLE: String =
            r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
                .to_string();
    }

    #[test]
    fn score_of_first_winning_board_for_sample_should_be_1924() {
        let lines: Vec<String> = read_lines(SAMPLE.as_bytes());
        let drawn_numbers = read_drawn_numbers(&lines[0]);
        let boards = read_boards::<5>(&lines[1..]);

        let (winning_board, last_drawn_number) =
            play_to_first_win(&drawn_numbers, &boards).unwrap();

        assert_eq!(
            winning_board.unmarked_numbers().iter().sum::<u32>() * last_drawn_number,
            4512
        );
    }

    #[test]
    fn score_of_last_winning_board_for_sample_should_be_1924() {
        let lines: Vec<String> = read_lines(SAMPLE.as_bytes());
        let drawn_numbers = read_drawn_numbers(&lines[0]);
        let boards = read_boards::<5>(&lines[1..]);

        let (winning_board, last_drawn_number) = play_to_end(&drawn_numbers, &boards).unwrap();

        assert_eq!(
            winning_board.unmarked_numbers().iter().sum::<u32>() * last_drawn_number,
            1924
        );
    }
}
