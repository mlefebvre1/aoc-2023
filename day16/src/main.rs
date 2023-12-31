use std::str::FromStr;

use common::Puzzle;

mod common;

fn main() {
    util::run!();
}

fn part1(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let mut puzzle = Puzzle::from_str(&data).unwrap();
    puzzle.run(true).to_string()
}

fn part2(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let mut puzzle = Puzzle::from_str(&data).unwrap();
    puzzle.run(false).to_string()
}
