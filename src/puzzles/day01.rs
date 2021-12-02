use itertools::Itertools;
use lazy_static::lazy_static;

use super::input;
use super::Puzzle;

lazy_static! {
    static ref INPUT: Vec<u32> = input::read_lines_from_file("day01");
}

pub struct Day1;

impl Puzzle for Day1 {
    fn number(&self) -> u8 {
        1
    }

    fn part_one(&self) -> String {
        format!("Number of increases: {}", count_increases(&INPUT))
    }

    fn part_two(&self) -> String {
        format!(
            "Number of increases on a three elements sliding window: {}",
            count_increases(&three_elements_sliding_window(&INPUT))
        )
    }
}

fn count_increases(input: &[u32]) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

fn three_elements_sliding_window(input: &[u32]) -> Vec<u32> {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_increases_should_return_7_for_sample_input() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = count_increases(&input);

        assert_eq!(result, 7);
    }

    #[test]
    fn three_elements_sliding_window_should_return_the_correct_vec_for_sample() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = three_elements_sliding_window(&input);

        assert_eq!(result, vec![607, 618, 618, 617, 647, 716, 769, 792]);
    }
}
