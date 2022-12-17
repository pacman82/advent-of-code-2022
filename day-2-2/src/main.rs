//! --- Day 2: Rock Paper Scissors ---
//!
//! The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the
//! snack storage, a giant Rock Paper Scissors tournament is already in progress. Rock Paper
//! Scissors is a game between two players. Each game contains many rounds; in each round, the
//! players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a
//! winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper
//! defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//! Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle
//! input) that they say will be sure to help you win. "The first column is what your opponent is
//! going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the
//! Elf is called away to help with someone's tent.
//!
//! The second column, you reason, must be what you should play in response: X for Rock, Y for
//! Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have
//! been carefully chosen. The winner of the whole tournament is the player with the highest score.
//! Your total score is the sum of your scores for each round. The score for a single round is the
//! score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score
//! for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
//!
//! Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the
//! score you would get if you were to follow the strategy guide. For example, suppose you were
//! given the following strategy guide:
//!
//! A Y
//! B X
//! C Z
//!
//! This strategy guide predicts and recommends the following:
//!
//!     In the first round, your opponent will choose Rock (A), and you should choose Paper (Y).
//! This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
//!     In the second round, your opponent will choose Paper (B), and you should choose Rock (X).
//! This ends in a loss for you with a score of 1 (1 + 0).     The third round is a draw with both
//! players choosing Scissors, giving you a score of 3 + 3 = 6.
//!
//! In this example, if you were to follow the strategy guide, you would get a total score of 15 (8
//! + 1 + 6). What would your total score be if everything goes exactly according to your strategy
//! guide?
//!
//! --- Part Two ---
//!
//! The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column
//! says how the round needs to end: X means you need to lose, Y means you need to end the round in
//! a draw, and Z means you need to win. Good luck!" The total score is still calculated in the same
//! way, but now you need to figure out what shape to choose so the round ends as indicated. The
//! example above now goes like this:
//!
//!     In the first round, your opponent will choose Rock (A), and you need the round to end in a
//! draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.     In the second round,
//! your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 =
//! 1.     In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 +
//! 6 = 7.
//!
//! Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total
//! score of 12. Following the Elf's instructions for the second column, what would your total score
//! be if everything goes exactly according to your strategy guide?

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use anyhow::Error;

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("input.txt")?);
    let points = accumulate_points(input);
    println!("{points}");
    Ok(())
}

fn accumulate_points(mut input: impl BufRead) -> u64 {
    let mut buffer = Vec::new();
    let strategy_guide = iter::from_fn(|| {
        buffer.clear();
        input.read_until(b'\n', &mut buffer).unwrap();
        Strategy::from_bytes(&buffer)
    });
    strategy_guide.map(Strategy::points).sum()
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn points(self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn beats(self, other: Shape) -> bool {
        matches!(
            (self, other),
            (Shape::Rock, Shape::Scissors)
                | (Shape::Paper, Shape::Rock)
                | (Shape::Scissors, Shape::Paper)
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Loose,
}

impl Outcome {
    fn versus(you: Shape, opponent: Shape) -> Outcome {
        if you.beats(opponent) {
            Outcome::Win
        } else if opponent.beats(you) {
            Outcome::Loose
        } else {
            Outcome::Draw
        }
    }

    fn points(self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loose => 0,
        }
    }
}

#[derive(Clone, Copy)]
struct Strategy {
    opponent: Shape,
    desired: Outcome,
}

impl Strategy {
    // Inupt e.g. b"A X" for Rock, Rock
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= 3 {
            let opponent = match bytes[0] {
                b'A' => Shape::Rock,
                b'B' => Shape::Paper,
                b'C' => Shape::Scissors,
                _ => return None,
            };

            let you = match bytes[2] {
                b'X' => Outcome::Loose,
                b'Y' => Outcome::Draw,
                b'Z' => Outcome::Win,
                _ => return None,
            };
            Some(Self {
                desired: you,
                opponent,
            })
        } else {
            None
        }
    }

    fn points(self) -> u64 {
        self.desired.points() + desired_shape(self.opponent, self.desired).points()
    }
}

fn desired_shape(opponent: Shape, desired: Outcome) -> Shape {
    [Shape::Rock, Shape::Paper, Shape::Scissors]
        .into_iter()
        .find(|&candidate| Outcome::versus(candidate, opponent) == desired)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{accumulate_points, Strategy};

    #[test]
    fn points_strategy() {
        let score = |bytes| Strategy::from_bytes(bytes).unwrap().points();
        assert_eq!(score(b"A Y"), 4);
        assert_eq!(score(b"B X"), 1);
        assert_eq!(score(b"C Z"), 7);
    }

    #[test]
    fn rock_paper_scissors_points_strategy_guide() {
        let guide = "\
            A Y\n\
            B X\n\
            C Z\n\
        ";

        let actual = accumulate_points(guide.as_bytes());

        assert_eq!(12, actual)
    }
}
