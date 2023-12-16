use std::str::FromStr;

use crate::common::Patterns;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let patterns = Patterns::from_str(&data).unwrap();
    let ans = patterns.reflections_score_smudge();
    ans.to_string()
}
