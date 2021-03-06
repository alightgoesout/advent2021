use std::env;

mod puzzles;

fn read_day_from_args() -> Option<u8> {
    env::args().nth(1).and_then(|arg| arg.parse().ok())
}

fn main() {
    let puzzles = puzzles::puzzles();
    if let Some(puzzle) = read_day_from_args().and_then(|day| puzzles.get(&day)) {
        puzzle.execute()
    }
}
