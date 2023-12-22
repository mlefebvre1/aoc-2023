mod common;

use std::str::FromStr;

use common::Puzzle;

fn main() {
    util::run!();
}

fn part1(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let puzzle = Puzzle::from_str(&data).unwrap();
    let ans = puzzle.run(1, 3);
    ans.to_string()
}
fn part2(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let puzzle = Puzzle::from_str(&data).unwrap();
    let ans = puzzle.run(4, 10);
    ans.to_string()
}
