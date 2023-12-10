use std::str::FromStr;

use crate::common::Report;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let report = Report::from_str(&data).unwrap();
    report.run_extrapolate_first().to_string()
}
