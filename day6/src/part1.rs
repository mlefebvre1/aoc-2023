use std::{num::ParseIntError, str::FromStr};

use anyhow::Error;

use crate::common::Race;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let puzzle = Puzzle::from_str(&data).unwrap();
    let ans = puzzle.run();
    ans.to_string()
}

#[derive(Debug)]
pub struct Puzzle(Vec<Race>);

impl Puzzle {
    pub fn run(&self) -> usize {
        self.0.iter().map(|race| race.nb_ways()).product()
    }
}

impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let times: Result<Vec<usize>, ParseIntError> = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .filter(|&c| !c.is_empty()) //remove whitespaces only
            .map(|c| c.parse())
            .collect();
        let records: Result<Vec<usize>, ParseIntError> = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace()
            .filter(|&c| !c.is_empty()) //remove whitespaces only
            .map(|c| c.parse())
            .collect();
        Ok(Self(
            times?
                .into_iter()
                .zip(records?)
                .map(|(time, record)| Race::new(time, record))
                .collect(),
        ))
    }
}
