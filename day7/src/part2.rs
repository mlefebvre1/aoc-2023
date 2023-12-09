use std::str::FromStr;

use crate::common::Puzzle;

const JOKER: usize = 1;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    Puzzle::<JOKER>::from_str(&data).unwrap().run().to_string()
}
