use std::str::FromStr;

use anyhow::{Error, Ok};

use crate::common::{Instruction, Map};

#[derive(Debug)]
struct Puzzle {
    instructions: Vec<Instruction>,
    map: Map,
}
impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions: Result<Vec<Instruction>, Error> = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c.try_into())
            .collect();
        lines.next();
        let rest: String = lines.collect::<Vec<_>>().join("\n");
        let map = Map::from_str(&rest);
        Ok(Self {
            instructions: instructions?,
            map: map?,
        })
    }
}
impl Puzzle {
    pub fn run(&self) -> usize {
        let mut nb_steps = 0;
        let mut next_element = "AAA";
        for instruction in self.instructions.iter().cycle() {
            match instruction {
                Instruction::Left => next_element = self.map.left(next_element),
                Instruction::Right => next_element = self.map.right(next_element),
            }
            nb_steps += 1;
            if next_element == "ZZZ" {
                break;
            }
        }
        nb_steps
    }
}

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    Puzzle::from_str(&data).unwrap().run().to_string()
}
