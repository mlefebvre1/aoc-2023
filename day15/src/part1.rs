use std::str::FromStr;

use crate::common::Step;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let ans: usize = data
        .split(',')
        .map(|ss| Step::from_str(ss).unwrap().inner())
        .sum();
    ans.to_string()
}
