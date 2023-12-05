use anyhow::Result;
use std::str::FromStr;

use crate::common::Card;

struct Cards(Vec<Card>);

impl Cards {
    fn new(data: &str) -> Result<Self> {
        let cards: Result<Vec<Card>> = data.lines().map(Card::from_str).collect();
        Ok(Self(cards?))
    }

    fn visit(&self) -> usize {
        let mut visit_sum = vec![0usize; self.0.len()];
        for card in self.0.iter().rev() {
            let n = card.matches().len();
            let i = card.id();
            visit_sum[i - 1] = 1 + visit_sum[i..i + n].iter().sum::<usize>();
        }
        visit_sum.iter().sum()
    }
}

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let cards = Cards::new(&data).unwrap();
    let ans = cards.visit();
    ans.to_string()
}
