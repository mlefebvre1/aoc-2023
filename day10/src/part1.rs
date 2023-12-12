use std::str::FromStr;

use crate::common::Diagram;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let diagram = Diagram::from_str(&data).unwrap();
    let visit = diagram.run();
    let farthest = visit.len() / 2;
    farthest.to_string()
}
