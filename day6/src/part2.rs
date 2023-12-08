use std::str::FromStr;

use anyhow::Error;

use crate::common::Race;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let puzzle = Puzzle::from_str(&data).unwrap();
    let ans = puzzle.run();
    ans.to_string()
}

#[derive(Debug)]
pub struct Puzzle(Race);

impl Puzzle {
    pub fn run(&self) -> usize {
        self.0.nb_ways()
    }
}

impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let time: usize = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .replace(' ', "")
            .parse()?;

        let record: usize = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .replace(' ', "")
            .parse()?;
        Ok(Self(Race::new(time, record)))
    }
}
