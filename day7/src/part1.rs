use std::str::FromStr;

use anyhow::Error;

use crate::common::Game;

#[derive(Debug)]
struct Puzzle(Vec<Game>);
impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let puzzle: Result<Vec<Game>, Error> = s.lines().map(Game::from_str).collect();
        Ok(Self(puzzle?))
    }
}
impl Puzzle {
    pub fn run(&mut self) -> usize {
        self.0.sort();
        self.0
            .iter()
            .enumerate()
            .map(|(rank, game)| (rank + 1) * game.bid())
            .sum()
    }
}

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let mut puzzle = Puzzle::from_str(&data).unwrap();
    let ans = puzzle.run();
    ans.to_string()
}
