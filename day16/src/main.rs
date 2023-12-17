use std::str::FromStr;

use common::Puzzle1;
use util::Cli;

mod common;

fn main() {
    let cli = Cli::get();

    println!("part1={}", part1(&cli.file));
}

fn part1(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let mut puzzle = Puzzle1::from_str(&data).unwrap();
    puzzle.run();
    puzzle.count_energized_tiles().to_string()
}
