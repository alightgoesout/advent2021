use itertools::{FoldWhile, Itertools};
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Sub;

use super::{input, Puzzle};

lazy_static! {
    static ref SCANNERS: Vec<Scanner> = parse_scanners(&input::read_file("day19"));
    static ref NORMALIZED_SCANNERS: Vec<Scanner> = normalize_scanners(&SCANNERS);
}

pub struct Day19;

impl Puzzle for Day19 {
    fn number(&self) -> u8 {
        19
    }

    fn part_one(&self) -> String {
        let beacons = get_all_beacons(&NORMALIZED_SCANNERS);
        format!("Total number of beacons: {}", beacons.len())
    }

    fn part_two(&self) -> String {
        format!(
            "Maximum manhattan distance between two scanners: {}",
            find_highest_manhattan_distance(&NORMALIZED_SCANNERS)
        )
    }
}

fn parse_scanners(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut current_beacons = Vec::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        if line.starts_with("---") {
            if !current_beacons.is_empty() {
                scanners.push(Scanner::new(std::mem::take(&mut current_beacons)))
            }
        } else {
            current_beacons.push(line.into());
        }
    }

    if !current_beacons.is_empty() {
        scanners.push(Scanner::new(current_beacons));
    }

    scanners
}

fn normalize_scanners(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut normalized_scanners = scanners.first().cloned().into_iter().collect::<Vec<_>>();
    let mut to_normalize = scanners
        .iter()
        .skip(1)
        .cloned()
        .into_iter()
        .collect::<VecDeque<_>>();

    while let Some(scanner) = to_normalize.pop_front() {
        let result = normalized_scanners
            .iter()
            .find_map(|normalized_scanner| scanner.find_common_beacons(normalized_scanner));
        if let Some(common_beacons) = result {
            let transformation = find_matching_transformation(&common_beacons);
            if let Some((transformation, position)) = transformation {
                normalized_scanners.push(scanner.transform(transformation, position));
                continue;
            }
        }
        to_normalize.push_back(scanner);
    }

    normalized_scanners
}

fn find_matching_transformation(
    common_beacons: &HashMap<Position, Position>,
) -> Option<(&dyn Transformation, Position)> {
    TRANSFORMATIONS.iter().find_map(|transformation| {
        get_scanner_position(common_beacons, transformation.as_ref())
            .map(|position| (transformation.as_ref(), position))
    })
}

fn get_scanner_position(
    common_beacons: &HashMap<Position, Position>,
    transformation: &dyn Transformation,
) -> Option<Position> {
    common_beacons
        .iter()
        .map(|(position, &normalized_position)| {
            transformation.transform(*position) - normalized_position
        })
        .fold_while(None, |scanner_position, diff| match scanner_position {
            Some(position) if position == diff => FoldWhile::Continue(Some(position)),
            Some(_) => FoldWhile::Done(None),
            None => FoldWhile::Continue(Some(diff)),
        })
        .into_inner()
}

fn get_all_beacons(scanners: &[Scanner]) -> HashSet<Position> {
    scanners
        .iter()
        .flat_map(|scanner| scanner.beacons.keys())
        .copied()
        .collect()
}

fn find_highest_manhattan_distance(scanners: &[Scanner]) -> i32 {
    scanners
        .iter()
        .cartesian_product(scanners)
        .map(|(s1, s2)| s1.position.manhattan_distance(&s2.position))
        .max()
        .unwrap()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn manhattan_distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromIterator<i32> for Position {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Self {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        }
    }
}

impl From<&str> for Position {
    fn from(line: &str) -> Self {
        line.split(',')
            .take(3)
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Scanner {
    beacons: HashMap<Position, HashSet<i32>>,
    position: Position,
}

impl Scanner {
    fn new(beacons: Vec<Position>) -> Self {
        Self {
            beacons: beacons
                .iter()
                .map(|beacon| (*beacon, get_distances(beacon, &beacons)))
                .collect(),
            position: Position { x: 0, y: 0, z: 0 },
        }
    }

    fn find_common_beacons(&self, other: &Scanner) -> Option<HashMap<Position, Position>> {
        let common_beacons = self
            .beacons
            .iter()
            .flat_map(|(beacon, distances)| {
                other
                    .beacons
                    .iter()
                    .find(|(_, other_distances)| have_common_distances(distances, other_distances))
                    .map(|(other_beacon, _)| (*beacon, *other_beacon))
            })
            .collect::<HashMap<_, _>>();

        (common_beacons.len() >= 12).then(|| common_beacons)
    }

    fn transform(&self, transformation: &dyn Transformation, position: Position) -> Self {
        Self {
            beacons: self
                .beacons
                .iter()
                .map(|(p, d)| (transformation.transform(*p) - position, d.clone()))
                .collect(),
            position,
        }
    }
}

fn get_distances(beacon: &Position, beacons: &[Position]) -> HashSet<i32> {
    beacons
        .iter()
        .filter(|b| beacon != *b)
        .map(|b| square_distance(beacon, b))
        .collect()
}

fn square_distance(b1: &Position, b2: &Position) -> i32 {
    (b1.x - b2.x).pow(2) + (b1.y - b2.y).pow(2) + (b1.z - b2.z).pow(2)
}

fn have_common_distances(d1: &HashSet<i32>, d2: &HashSet<i32>) -> bool {
    d1.iter().filter(|d| d2.contains(*d)).take(11).count() == 11
}

trait Transformation: Sync {
    fn transform(&self, position: Position) -> Position;
}

impl<F> Transformation for F
where
    F: Fn(Position) -> Position + Sync,
{
    fn transform(&self, position: Position) -> Position {
        self(position)
    }
}

lazy_static! {
    static ref TRANSFORMATIONS: [Box<dyn Transformation>; 24] = [
        Box::new(|Position { x, y, z }| Position { x, y, z }),
        Box::new(|Position { x, y, z }| Position { x, y: -z, z: y }),
        Box::new(|Position { x, y, z }| Position { x, y: -y, z: -z }),
        Box::new(|Position { x, y, z }| Position { x, y: z, z: -y }),
        Box::new(|Position { x, y, z }| Position { x: -y, y: x, z }),
        Box::new(|Position { x, y, z }| Position { x: z, y: x, z: y }),
        Box::new(|Position { x, y, z }| Position { x: y, y: x, z: -z }),
        Box::new(|Position { x, y, z }| Position { x: -z, y: x, z: -y }),
        Box::new(|Position { x, y, z }| Position { x: -x, y: -y, z }),
        Box::new(|Position { x, y, z }| Position {
            x: -x,
            y: -z,
            z: -y
        }),
        Box::new(|Position { x, y, z }| Position { x: -x, y, z: -z }),
        Box::new(|Position { x, y, z }| Position { x: -x, y: z, z: y }),
        Box::new(|Position { x, y, z }| Position { x: y, y: -x, z }),
        Box::new(|Position { x, y, z }| Position { x: z, y: -x, z: -y }),
        Box::new(|Position { x, y, z }| Position {
            x: -y,
            y: -x,
            z: -z
        }),
        Box::new(|Position { x, y, z }| Position { x: -z, y: -x, z: y }),
        Box::new(|Position { x, y, z }| Position { x: -z, y, z: x }),
        Box::new(|Position { x, y, z }| Position { x: y, y: z, z: x }),
        Box::new(|Position { x, y, z }| Position { x: z, y: -y, z: x }),
        Box::new(|Position { x, y, z }| Position { x: -y, y: -z, z: x }),
        Box::new(|Position { x, y, z }| Position {
            x: -z,
            y: -y,
            z: -x
        }),
        Box::new(|Position { x, y, z }| Position { x: -y, y: z, z: -x }),
        Box::new(|Position { x, y, z }| Position { x: z, y, z: -x }),
        Box::new(|Position { x, y, z }| Position { x: y, y: -z, z: -x }),
    ];
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SAMPLE: Vec<Scanner> = parse_scanners(&input::read_file("day19_sample"));
    }

    #[test]
    fn first_two_scanners_of_sample_should_have_12_common_beacons() {
        let common_beacons = SAMPLE[0].find_common_beacons(&SAMPLE[1]).unwrap();

        assert_eq!(common_beacons.len(), 12);
    }

    #[test]
    fn transformation_to_normalize_second_scanner() {
        let common_beacons = SAMPLE[0].find_common_beacons(&SAMPLE[1]).unwrap();

        let transformation = find_matching_transformation(&common_beacons);

        assert!(transformation.is_some())
    }

    #[test]
    fn normalize_first_two_scanners_from_sample() {
        let normalized_scanners = normalize_scanners(&SAMPLE[0..=1]);

        let expected: Vec<Position> = vec![
            "-618,-824,-621".into(),
            "-537,-823,-458".into(),
            "-447,-329,318".into(),
            "404,-588,-901".into(),
            "544,-627,-890".into(),
            "528,-643,409".into(),
            "-661,-816,-575".into(),
            "390,-675,-793".into(),
            "423,-701,434".into(),
            "-345,-311,381".into(),
            "459,-707,401".into(),
            "-485,-357,347".into(),
        ];
        assert!(expected
            .iter()
            .all(|beacon| normalized_scanners[0].beacons.contains_key(beacon)));
        assert!(expected
            .iter()
            .all(|beacon| normalized_scanners[1].beacons.contains_key(beacon)));
    }

    #[test]
    fn sample_should_have_79_beacons() {
        let normalized_scanners = normalize_scanners(&SAMPLE);
        let beacons = get_all_beacons(&normalized_scanners);

        assert_eq!(beacons.len(), 79);
    }

    #[test]
    fn find_highest_manhattan_distance_should_return_3621_for_sample() {
        let normalized_scanners = normalize_scanners(&SAMPLE);

        let result = find_highest_manhattan_distance(&normalized_scanners);

        assert_eq!(result, 3621);
    }
}
