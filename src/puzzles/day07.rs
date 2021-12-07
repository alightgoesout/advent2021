use lazy_static::lazy_static;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: Vec<u32> = input::read_lines_from_file::<String>("day07")[0]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
}

pub struct Day7;

impl Puzzle for Day7 {
    fn number(&self) -> u8 {
        7
    }

    fn part_one(&self) -> String {
        let alignment_position = compute_alignment_position(&INPUT);
        let fuel = compute_fuel_to_align_at_position(&INPUT, alignment_position);
        format!("Minimum fuel to align: {}", fuel)
    }

    fn part_two(&self) -> String {
        let (_, fuel) = min_search_from_mean_with_new_fuel_consumption_model(&INPUT);
        format!(
            "Minimum fuel to align with new fuel consumption model: {}",
            fuel
        )
    }
}

fn compute_alignment_position(input: &[u32]) -> u32 {
    let mut positions = input.to_vec();
    positions.sort_unstable();
    positions
        .get((positions.len() + 1) / 2 - 1)
        .copied()
        .unwrap_or(0)
}

fn compute_fuel_to_align_at_position(input: &[u32], position: u32) -> u32 {
    input
        .iter()
        .map(|p| {
            if *p < position {
                position - p
            } else {
                p - position
            }
        })
        .sum()
}

fn mean(input: &[u32]) -> u32 {
    let sum: u32 = input.iter().sum();
    (sum as f64 / (input.len() as f64)).round() as u32
}

fn min_search_from_mean_with_new_fuel_consumption_model(input: &[u32]) -> (u32, u32) {
    let mut res = {
        let mean = mean(input);
        (
            mean,
            compute_fuel_to_align_at_position_with_new_fuel_consumption_model(input, mean),
        )
    };
    loop {
        let before = (
            res.0 - 1,
            compute_fuel_to_align_at_position_with_new_fuel_consumption_model(input, res.0 - 1),
        );
        let after = (
            res.0 + 1,
            compute_fuel_to_align_at_position_with_new_fuel_consumption_model(input, res.0 + 1),
        );
        if before.1 >= res.1 && after.1 >= res.1 {
            break;
        } else if before.1 >= res.1 {
            res = after
        } else {
            res = before
        }
    }
    res
}

fn compute_fuel_to_align_at_position_with_new_fuel_consumption_model(
    input: &[u32],
    position: u32,
) -> u32 {
    input
        .iter()
        .map(|p| {
            if *p < position {
                position - p
            } else {
                p - position
            }
        })
        .map(triangle_number)
        .sum()
}

fn triangle_number(n: u32) -> u32 {
    (1..=n).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: Vec<u32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    }

    #[test]
    fn alignment_position_for_sample_should_be_2() {
        assert_eq!(compute_alignment_position(&SAMPLE), 2);
    }

    #[test]
    fn fuel_consumption_to_align_sample_to_position_2_should_be_37() {
        assert_eq!(compute_fuel_to_align_at_position(&SAMPLE, 2), 37);
    }

    #[test]
    fn fuel_consumption_to_align_sample_to_position_2_should_be_206_for_new_fuel_consumption_model()
    {
        assert_eq!(
            compute_fuel_to_align_at_position_with_new_fuel_consumption_model(&SAMPLE, 2),
            206
        );
    }

    #[test]
    fn fuel_consumption_to_align_sample_to_position_5_should_be_168_for_new_fuel_consumption_model()
    {
        assert_eq!(
            compute_fuel_to_align_at_position_with_new_fuel_consumption_model(&SAMPLE, 5),
            168
        );
    }

    #[test]
    fn test_min_search_from_mean_with_new_fuel_consumption_model() {
        assert_eq!(
            min_search_from_mean_with_new_fuel_consumption_model(&SAMPLE),
            (5, 168)
        );
    }
}
