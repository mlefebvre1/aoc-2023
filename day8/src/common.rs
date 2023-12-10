use anyhow::{anyhow, Error};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("can't convert char to instruction")),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    right: String,
    left: String,
}

#[derive(Debug)]
pub struct Map(HashMap<String, Node>);
impl Map {
    pub fn left(&self, element: &str) -> &str {
        &self.0[element].left
    }
    pub fn right(&self, element: &str) -> &str {
        &self.0[element].right
    }
}
impl AsRef<HashMap<String, Node>> for Map {
    fn as_ref(&self) -> &HashMap<String, Node> {
        &self.0
    }
}
impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //AAA = (BBB, BBB)
        let map = s
            .lines()
            .map(|line| {
                let mut sp = line.split('=');
                let entry = sp.next().unwrap().trim().to_string();
                let node_sp = sp.next().unwrap().replace(['(', ')'], "");
                let mut node_sp = node_sp.split(',');
                let left = node_sp.next().unwrap().trim().to_string();
                let right = node_sp.next().unwrap().trim().to_string();
                let node = Node { right, left };
                (entry, node)
            })
            .collect();
        Ok(Self(map))
    }
}
