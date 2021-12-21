use std::cmp::max;
use std::collections::HashMap;

use super::Puzzle;

pub struct Day21;

const PLAYER_ONE_STARTING_POSITION: usize = 7;
const PLAYER_TWO_STARTING_POSITION: usize = 8;

impl Puzzle for Day21 {
    fn number(&self) -> u8 {
        21
    }

    fn part_one(&self) -> String {
        let mut game = DiceGame::new(
            DeterministicDie::<100>::new(),
            PLAYER_ONE_STARTING_POSITION,
            PLAYER_TWO_STARTING_POSITION,
        );
        game.play();
        format!(
            "Score of losing player multiplied by number of rounds: {}",
            game.players.iter().map(|p| p.score).min().unwrap() * game.rolls
        )
    }

    fn part_two(&self) -> String {
        let mut explorer = UniverseExplorer::new();
        let (w1, w2) = explorer.get_wins(
            Player::new(PLAYER_ONE_STARTING_POSITION),
            Player::new(PLAYER_TWO_STARTING_POSITION),
            0,
        );
        format!("Most wins: {}", max(w1, w2))
    }
}

trait Die {
    fn roll(&mut self) -> usize;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DeterministicDie<const N: usize>(usize);

impl<const N: usize> DeterministicDie<N> {
    fn new() -> Self {
        Self(N)
    }
}

impl<const N: usize> Die for DeterministicDie<N> {
    fn roll(&mut self) -> usize {
        self.0 += 1;
        if self.0 > N {
            self.0 = 1;
        }
        self.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DiceGame<D: Die> {
    die: D,
    rolls: usize,
    players: [Player; 2],
    status: GameStatus,
}

impl<D: Die> DiceGame<D> {
    fn new(die: D, player_one_position: usize, player_two_position: usize) -> Self {
        Self {
            die,
            rolls: 0,
            players: [
                Player::new(player_one_position),
                Player::new(player_two_position),
            ],
            status: GameStatus::Playing(0),
        }
    }

    fn play(&mut self) {
        while self.status != GameStatus::Over {
            self.play_next_player_turn();
        }
    }

    fn play_next_player_turn(&mut self) {
        if let GameStatus::Playing(p) = self.status {
            let roll = self.roll();

            let player = &mut self.players[p];
            *player = player.move_pawn(roll);

            if player.score >= 1000 {
                self.status = GameStatus::Over;
            } else {
                self.status = GameStatus::Playing((p + 1) % 2)
            }
        }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 3;
        self.die.roll() + self.die.roll() + self.die.roll()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn move_pawn(self, roll: usize) -> Self {
        let mut position = self.position + roll;
        while position > 10 {
            position -= 10;
        }
        Self {
            position,
            score: self.score + position,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GameStatus {
    Playing(usize),
    Over,
}

const DIRAC_DICE_ROLLS: [usize; 27] = [
    3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
];

struct UniverseExplorer {
    cache: HashMap<(Player, Player, u8), (usize, usize)>,
}

impl UniverseExplorer {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn get_wins(&mut self, player1: Player, player2: Player, current_player: u8) -> (usize, usize) {
        if let Some(wins) = self.cache.get(&(player1, player2, current_player)) {
            *wins
        } else {
            let wins = self.compute_wins(player1, player2, current_player);
            self.cache.insert((player1, player2, current_player), wins);
            wins
        }
    }

    fn compute_wins(
        &mut self,
        player1: Player,
        player2: Player,
        current_player: u8,
    ) -> (usize, usize) {
        if player1.score >= 21 {
            return (1, 0);
        }
        if player2.score >= 21 {
            return (0, 1);
        }

        let mut wins = (0, 0);

        for roll in DIRAC_DICE_ROLLS {
            let new_wins = if current_player == 0 {
                self.get_wins(player1.move_pawn(roll), player2, 1)
            } else {
                self.get_wins(player1, player2.move_pawn(roll), 0)
            };
            wins = (wins.0 + new_wins.0, wins.1 + new_wins.1);
        }

        wins
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn losing_score_should_be_745_and_nb_rolls_should_be_993_when_starting_positions_are_4_and_8_and_using_deterministic_die(
    ) {
        let mut game = DiceGame::new(DeterministicDie::<100>::new(), 4, 8);

        game.play();

        assert_eq!(game.rolls, 993);
        assert_eq!(game.players[1].score, 745);
    }
}
