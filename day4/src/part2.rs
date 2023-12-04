use anyhow::Result;
use std::str::FromStr;

use crate::common::Card;

pub fn run() -> String {
    let data = std::fs::read_to_string("day4/data/day4tmp.txt").unwrap();
    let cards: Result<Vec<Card>> = data.lines().map(Card::from_str).collect();
    let cards_and_matches: Vec<(Card, Vec<usize>)> = cards
        .unwrap()
        .into_iter()
        .map(|card| {
            let matches = card.matches();
            (card, matches)
        })
        .collect();

    let card_scores: Vec<usize> = cards_and_matches
        .iter()
        .map(|(_, matches)| {
            if !matches.is_empty() {
                2usize.pow(matches.len() as u32 - 1)
            } else {
                0
            }
        })
        .collect();
    println!("{card_scores:?}");

    for (card, matches) in cards_and_matches.iter() {}
    // let ans: usize = cards_and_matches
    //     .iter()
    //     .map(|(card, matches)| {
    //         if !matches.is_empty() {
    //             let i = card.id();
    //             let r = i..(i + matches.len());
    //             let score = card_scores[r.clone()].iter().sum();
    //             println!("i={i} range={r:?}, matches={:?}, score={score}", matches);
    //             score
    //         } else {
    //             0
    //         }
    //     })
    //     .sum();
    let ans = 0;
    ans.to_string()
}
