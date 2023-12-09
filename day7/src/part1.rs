use std::str::FromStr;

use crate::common::Puzzle;
const JACK: usize = 11;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    Puzzle::<JACK>::from_str(&data).unwrap().run().to_string()
}
