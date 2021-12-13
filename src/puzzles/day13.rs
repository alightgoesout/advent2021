use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use super::{input, Puzzle};

lazy_static! {
    static ref PAGE: Page = input::read_lines_from_file::<Dot>("day13_dots").into();
    static ref INSTRUCTIONS: Vec<FoldInstruction> =
        input::read_lines_from_file("day13_fold_instructions");
}

pub struct Day13;

impl Puzzle for Day13 {
    fn number(&self) -> u8 {
        13
    }

    fn part_one(&self) -> String {
        format!(
            "Number of visible dots after first fold: {}",
            PAGE.clone().fold(INSTRUCTIONS[0]).len(),
        )
    }

    fn part_two(&self) -> String {
        format!("Code:\n{}", PAGE.clone().fold_all(INSTRUCTIONS.clone()))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Dot(usize, usize);

impl FromStr for Dot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;
        Ok(Dot(x.parse().map_err(|_| ())?, y.parse().map_err(|_| ())?))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Page(HashSet<Dot>);

impl Page {
    fn fold(self, instruction: FoldInstruction) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|Dot(x, y)| match instruction {
                    FoldInstruction::VerticalFold(fold) => {
                        if x < fold {
                            Dot(x, y)
                        } else {
                            Dot(2 * fold - x, y)
                        }
                    }
                    FoldInstruction::HorizontalFold(fold) => {
                        if y < fold {
                            Dot(x, y)
                        } else {
                            Dot(x, 2 * fold - y)
                        }
                    }
                })
                .collect(),
        )
    }

    fn fold_all(self, instructions: impl IntoIterator<Item = FoldInstruction>) -> Self {
        instructions
            .into_iter()
            .fold(self, |page, instruction| page.fold(instruction))
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.0.iter().map(|&Dot(x, _)| x).max().unwrap_or(0);
        let height = self.0.iter().map(|&Dot(_, y)| y).max().unwrap_or(0);

        for y in 0..=height {
            for x in 0..=width {
                if self.0.contains(&Dot(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T: IntoIterator<Item = Dot>> From<T> for Page {
    fn from(dots: T) -> Self {
        Self(dots.into_iter().collect())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum FoldInstruction {
    VerticalFold(usize),
    HorizontalFold(usize),
}

const VERTICAL_FOLD_PREFIX: &str = "fold along x=";
const HORIZONTAL_FOLD_PREFIX: &str = "fold along y=";

impl FromStr for FoldInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(x) = s.strip_prefix(VERTICAL_FOLD_PREFIX) {
            Ok(Self::VerticalFold(x.parse().map_err(|_| ())?))
        } else if let Some(y) = s.strip_prefix(HORIZONTAL_FOLD_PREFIX) {
            Ok(Self::HorizontalFold(y.parse().map_err(|_| ())?))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE_PAGE: Page = input::read_lines::<Dot, _>(
            r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0
"
            .as_bytes()
        )
        .into();
    }

    #[test]
    fn sample_after_horizontal_fold_on_7_should_have_17_dots() {
        let result = SAMPLE_PAGE.clone().fold(FoldInstruction::HorizontalFold(7));

        assert_eq!(result.len(), 17);
    }

    #[test]
    fn sample_after_two_folds_should_have_16_dots() {
        let result = SAMPLE_PAGE.clone().fold_all([
            FoldInstruction::HorizontalFold(7),
            FoldInstruction::VerticalFold(5),
        ]);

        assert_eq!(result.len(), 16);
    }
}
