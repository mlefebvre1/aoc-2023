use std::str::FromStr;

use crate::common::Almanac;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let almanac = Almanac::from_str(&data).unwrap();
    let locations = almanac.transform_seeds();
    let ans = locations.iter().min().unwrap();
    ans.to_string()
}
