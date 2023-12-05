use std::str::FromStr;

use crate::common::Card;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let ans: usize = data
        .lines()
        .map(|line| {
            let card = Card::from_str(line).unwrap();
            card.score()
        })
        .sum();

    ans.to_string()
}
