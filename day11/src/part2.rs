use std::str::FromStr;

use crate::common::Image;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let image = Image::from_str(&data).unwrap();
    let ans = image.run::<1000000>();
    ans.to_string()
}
