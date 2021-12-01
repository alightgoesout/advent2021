use std::env;

mod input;
mod puzzles;

fn read_day_from_args() -> Option<u8> {
    env::args().nth(1).and_then(|arg| arg.parse().ok())
}

fn main() {
    let puzzles = puzzles::puzzles();
    read_day_from_args()
        .and_then(|day| puzzles.get(&day))
        .map(|puzzle| puzzle.execute());
}
