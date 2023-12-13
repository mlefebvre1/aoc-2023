use std::str::FromStr;

use anyhow::Error;

use crate::common::Row;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let rows: Result<Vec<Row>, Error> = data.lines().map(Row::from_str).collect();
    let ans: usize = rows
        .unwrap()
        .iter()
        .map(|row| row.nb_arrangements())
        .inspect(|nb| println!("{nb}"))
        .sum();
    ans.to_string()
}
