use std::str::FromStr;

use common::Puzzle;

mod common;

fn main() {
    util::run!();
}

fn part1(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let puzzle = Puzzle::from_str(&data).unwrap();
    let ans = puzzle.run();

    ans.to_string()
}
fn part2(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let puzzle = Puzzle::from_str(&data).unwrap();
    let ans = puzzle.explore();
    ans.to_string()
}
