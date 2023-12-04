use std::str::FromStr;

use crate::common::Card;

pub fn run() -> String {
    let data = std::fs::read_to_string("day4/data/day4.txt").unwrap();
    let ans: usize = data
        .lines()
        .map(|line| {
            let card = Card::from_str(line).unwrap();
            let matches = card.matches();

            if !matches.is_empty() {
                2usize.pow(matches.len() as u32 - 1)
            } else {
                0
            }
        })
        .sum();

    ans.to_string()
}
