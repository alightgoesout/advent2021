use lazy_static::lazy_static;
use std::fmt::{Display, Formatter, Write};

use super::{input, Puzzle};

lazy_static! {
    static ref IMAGE_ENHANCEMENT: [u8; 512] = to_array(parse_image_data("####....#.....##.####..#.##.###.########.##.#..#.##.#...#..##.######..#......#..###.#.##.####.#.#.#....######.###...###.#.###.####..###.......#..#.#.#.#.#.####..####.#..####.#..####..##.#.#.#.###..##..#....#...###.#....###....##.###...##..#..#..#...##...#.#..#..###...####.#.#.###..#.#.#..###.##.##.#..###...#.#.#.##...#...#..#...##..###..###..#...###.#....#.##.#.####...##...##.#.####.#####.##...#######.###..##.#####.##.....####.#######.#.#.##....#...##...#..##.###.######.#######.#.#.#....#..##.###.#..##..##."));
    static ref INPUT: Image = input::read_lines_from_file::<String>("day20").into();
}

pub struct Day20;

impl Puzzle for Day20 {
    fn number(&self) -> u8 {
        20
    }

    fn part_one(&self) -> String {
        let enhanced = INPUT
            .enhance(IMAGE_ENHANCEMENT.as_ref())
            .enhance(IMAGE_ENHANCEMENT.as_ref());
        format!(
            "Number of lit pixels after two enhancements: {}",
            enhanced.count_lit_pixels()
        )
    }

    fn part_two(&self) -> String {
        let enhanced_fifty_times = (0..50).fold(INPUT.clone(), |image, _| {
            image.enhance(IMAGE_ENHANCEMENT.as_ref())
        });
        format!(
            "Number of lit pixels after 50 enhancements: {}",
            enhanced_fifty_times.count_lit_pixels()
        )
    }
}

fn parse_image_data(data: &str) -> impl Iterator<Item = u8> + '_ {
    data.chars().map(|c| match c {
        '.' => 0,
        '#' => 1,
        _ => panic!("Invalid character: {}", c),
    })
}

fn to_array<T: Default + Copy, const N: usize>(data: impl Iterator<Item = T>) -> [T; N] {
    let mut array = [T::default(); N];
    data.take(N).enumerate().for_each(|(i, d)| array[i] = d);
    array
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Image {
    pixels: Vec<Vec<u8>>,
    default: u8,
}

impl Image {
    pub fn enhance(&self, image_enhancement: &[u8]) -> Self {
        let mut new_image = Self {
            pixels: Vec::new(),
            default: match self.default {
                0 => image_enhancement[0],
                _ => image_enhancement[511],
            },
        };

        let width = self.pixels.iter().map(Vec::len).max().unwrap_or(0);
        let height = self.pixels.len();

        for y in 0..height + 2 {
            for x in 0..width + 2 {
                let pixel =
                    image_enhancement[self.get_encoded_value(x as isize - 1, y as isize - 1)];
                if pixel != new_image.default {
                    new_image.insert(x, y, pixel)
                }
            }
        }

        new_image
    }

    fn get_encoded_value(&self, x: isize, y: isize) -> usize {
        self.get_surrounding_pixels(x, y)
            .iter()
            .fold(0, |v, d| v * 2 + *d as usize)
    }

    fn get_surrounding_pixels(&self, x: isize, y: isize) -> [u8; 9] {
        let mut pixels: [u8; 9] = [self.default; 9];
        for i in -1..=1 {
            for j in -1..=1 {
                if y + j >= 0 && y + j < self.pixels.len() as isize {
                    let line = &self.pixels[(y + j) as usize];
                    if x + i >= 0 && x + i < line.len() as isize {
                        let index = (i + 1 + (j + 1) * 3) as usize;
                        pixels[index] = line[(x + i) as usize];
                    }
                }
            }
        }
        pixels
    }

    fn insert(&mut self, x: usize, y: usize, pixel: u8) {
        while self.pixels.len() <= y {
            self.pixels.push(Vec::new());
        }
        let line = self.pixels.get_mut(y).unwrap();
        while line.len() <= x {
            line.push(self.default);
        }
        line[x] = pixel;
    }

    pub fn count_lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .flat_map(|line| line.iter())
            .filter(|p| **p == 1)
            .count()
    }
}

impl From<Vec<String>> for Image {
    fn from(lines: Vec<String>) -> Self {
        Self {
            pixels: lines
                .iter()
                .map(|line| parse_image_data(line).collect())
                .collect(),
            default: 0,
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.pixels.iter().map(Vec::len).max().unwrap_or(0);

        for line in &self.pixels {
            for x in 0..width {
                let char = match line.get(x).unwrap_or(&self.default) {
                    1 => '#',
                    _ => '.',
                };
                f.write_char(char)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: &str = r"#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_parse_and_display_on_sample() {
        let image: Image = input::read_lines(SAMPLE.as_bytes()).into();

        assert_eq!(
            &image.to_string(),
            r"#..#.
#....
##..#
..#..
..###
"
        )
    }

    #[test]
    fn test_sample_after_one_enhancement() {
        let enhancement_data: [u8; 512] = to_array(parse_image_data("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"));
        let image: Image = input::read_lines(SAMPLE.as_bytes()).into();

        let enhanced_image = image.enhance(&enhancement_data);

        assert_eq!(
            &enhanced_image.to_string(),
            r".##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#.
"
        );
    }

    #[test]
    fn test_sample_after_two_enhancements() {
        let enhancement_data: [u8; 512] = to_array(parse_image_data("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"));
        let image: Image = input::read_lines(SAMPLE.as_bytes()).into();

        let enhanced_image = image.enhance(&enhancement_data).enhance(&enhancement_data);

        assert_eq!(
            &enhanced_image.to_string(),
            r".......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..
"
        );
    }

    #[test]
    fn count_lit_pixels_should_return_35_for_sample_enhanced_twice() {
        let enhancement_data: [u8; 512] = to_array(parse_image_data("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"));
        let image: Image = input::read_lines(SAMPLE.as_bytes()).into();

        let enhanced_image = image.enhance(&enhancement_data).enhance(&enhancement_data);

        assert_eq!(enhanced_image.count_lit_pixels(), 35);
    }
}
