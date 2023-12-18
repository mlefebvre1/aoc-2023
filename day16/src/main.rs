use std::str::FromStr;

use common::Puzzle;
use util::Cli;

mod common;

fn main() {
    let cli = Cli::get();

    println!("part1={}", part1(&cli.file));
    println!("part2={}", part2(&cli.file));
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
