use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: CaveMap = input::read_lines_from_file::<Edge>("day12").into();
}

pub struct Day12;

impl Puzzle for Day12 {
    fn number(&self) -> u8 {
        12
    }

    fn part_one(&self) -> String {
        format!(
            "Paths that visit small caves at most once: {}",
            INPUT.compute_all_paths_visiting_small_caves_once().len()
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Paths that visit one small cave twice: {}",
            INPUT
                .compute_all_paths_visiting_one_small_cave_twice()
                .len()
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Node {
    Start,
    End,
    BigCave(String),
    SmallCave(String),
}

impl Node {
    fn is_small_cave(&self) -> bool {
        matches!(self, Node::SmallCave(_))
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            s if is_all_uppercase(s) => Ok(Self::BigCave(s.to_string())),
            s => Ok(Self::SmallCave(s.to_string())),
        }
    }
}

fn is_all_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Edge {
    from: Node,
    to: Node,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = s.split_once('-') {
            Ok(Self {
                from: from.parse()?,
                to: to.parse()?,
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CaveMap(HashMap<Node, Vec<Node>>);

impl CaveMap {
    fn compute_all_paths(&self, is_visitable: fn(&Node, &[&Node]) -> bool) -> Vec<Vec<&Node>> {
        let mut paths = Vec::new();

        let mut current_path = Vec::new();
        let mut to_visit = vec![vec![&Node::Start]];
        while let Some(next_nodes) = to_visit.last_mut() {
            if let Some(next) = next_nodes.pop() {
                current_path.push(next);
                if let Some(visitable_nodes) =
                    self.find_next_visitable_nodes(&current_path, is_visitable)
                {
                    to_visit.push(visitable_nodes);
                } else if next == &Node::End {
                    paths.push(current_path.clone());
                    current_path.pop();
                } else {
                    current_path.pop();
                }
            } else {
                current_path.pop();
                to_visit.pop();
            }
        }

        paths
    }

    fn find_next_visitable_nodes(
        &self,
        current_path: &[&Node],
        is_visitable: fn(&Node, &[&Node]) -> bool,
    ) -> Option<Vec<&Node>> {
        current_path
            .last()
            .and_then(|last_node| self.0.get(*last_node))
            .map(|visitable_nodes| {
                visitable_nodes
                    .iter()
                    .filter(|n| is_visitable(n, current_path))
                    .collect::<Vec<_>>()
            })
            .filter(|visitable_nodes| !visitable_nodes.is_empty())
    }

    fn compute_all_paths_visiting_small_caves_once(&self) -> Vec<Vec<&Node>> {
        self.compute_all_paths(|node, current_path| {
            !node.is_small_cave() || !current_path.iter().contains(&node)
        })
    }

    fn compute_all_paths_visiting_one_small_cave_twice(&self) -> Vec<Vec<&Node>> {
        self.compute_all_paths(|node, current_path| {
            !node.is_small_cave()
                || !current_path.iter().contains(&node)
                || current_path
                    .iter()
                    .filter(|n| n.is_small_cave())
                    .all_unique()
        })
    }
}

impl From<Vec<Edge>> for CaveMap {
    fn from(edges: Vec<Edge>) -> Self {
        let mut map = HashMap::new();

        for edge in edges {
            if edge.from != Node::End && edge.to != Node::Start {
                map.entry(edge.from.clone())
                    .or_insert_with(Vec::new)
                    .push(edge.to.clone());
            }
            if edge.from != Node::Start && edge.to != Node::End {
                map.entry(edge.to).or_insert_with(Vec::new).push(edge.from);
            }
        }

        Self(map)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref SMALL_SAMPLE: CaveMap = input::read_lines::<Edge, _>(
            r"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"
            .as_bytes()
        )
        .into();
        static ref MEDIUM_SAMPLE: CaveMap = input::read_lines::<Edge, _>(
            r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"
            .as_bytes()
        )
        .into();
        static ref LARGE_SAMPLE: CaveMap = input::read_lines::<Edge, _>(
            r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"
            .as_bytes()
        )
        .into();
    }

    #[test]
    fn compute_all_paths_visiting_small_caves_once_should_return_10_paths_for_small_sample() {
        assert_eq!(
            SMALL_SAMPLE
                .compute_all_paths_visiting_small_caves_once()
                .len(),
            10
        );
    }

    #[test]
    fn compute_all_paths_visiting_small_caves_once_should_return_19_paths_for_medium_sample() {
        assert_eq!(
            MEDIUM_SAMPLE
                .compute_all_paths_visiting_small_caves_once()
                .len(),
            19
        );
    }

    #[test]
    fn compute_all_paths_visiting_small_caves_once_should_return_226_paths_for_large_sample() {
        assert_eq!(
            LARGE_SAMPLE
                .compute_all_paths_visiting_small_caves_once()
                .len(),
            226
        );
    }

    #[test]
    fn compute_all_paths_visiting_one_small_cave_twice_return_36_paths_for_small_sample() {
        assert_eq!(
            SMALL_SAMPLE
                .compute_all_paths_visiting_one_small_cave_twice()
                .len(),
            36
        );
    }

    #[test]
    fn compute_all_paths_visiting_one_small_cave_twice_return_103_paths_for_medium_sample() {
        assert_eq!(
            MEDIUM_SAMPLE
                .compute_all_paths_visiting_one_small_cave_twice()
                .len(),
            103
        );
    }

    #[test]
    fn compute_all_paths_visiting_one_small_cave_twice_return_3509_paths_for_large_sample() {
        assert_eq!(
            LARGE_SAMPLE
                .compute_all_paths_visiting_one_small_cave_twice()
                .len(),
            3509
        );
    }
}
