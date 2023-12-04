use std::{num::ParseIntError, str::FromStr};

use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

impl Card {
    pub fn matches(&self) -> Vec<usize> {
        self.my_numbers
            .iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .copied()
            .collect()
    }
    pub fn score(&self) -> usize {
        let matches = self.matches();
        if !matches.is_empty() {
            2usize.pow(matches.len() as u32 - 1)
        } else {
            0
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let mut s = s.split(':');
        let card_raw = s
            .next()
            .ok_or(anyhow!("failed to get 'Card X' substring"))?;
        let card_id = card_raw
            .split_whitespace()
            .nth(1)
            .ok_or(anyhow!("failed to get card id§"))?
            .parse::<usize>()?;

        let numbers = s.next().unwrap();
        let mut numbers_split = numbers.split('|');

        let winning_numbers: Result<Vec<usize>, ParseIntError> = numbers_split
            .next()
            .ok_or(anyhow!("failed to get winning numbers"))?
            .split_whitespace()
            .filter(|n| !n.is_empty())
            .map(|n| n.parse())
            .collect();

        let my_numbers: Result<Vec<usize>, ParseIntError> = numbers_split
            .next()
            .ok_or(anyhow!("failed to get winning numbers"))?
            .split_whitespace()
            .filter(|n| !n.is_empty())
            .map(|n| n.parse())
            .collect();

        Ok(Self {
            id: card_id,
            winning_numbers: winning_numbers?,
            my_numbers: my_numbers?,
        })
    }
}
