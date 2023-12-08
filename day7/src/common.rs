use std::str::FromStr;

use anyhow::{anyhow, Error};

#[derive(Debug, PartialEq, PartialOrd)]
struct Card(usize);
impl TryFrom<char> for Card {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            c @ '2'..='9' => Ok(Self(
                c.to_digit(10).ok_or(anyhow!("fail to convert to digit"))? as usize,
            )),
            'T' => Ok(Self(10)),
            'J' => Ok(Self(11)),
            'Q' => Ok(Self(12)),
            'K' => Ok(Self(13)),
            'A' => Ok(Self(14)),
            _ => Err(anyhow!("character '{value}' can't be converted to a Card")),
        }
    }
}
impl Card {
    pub fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
struct Hand([Card; 5]);
impl FromStr for Hand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand: Result<Vec<Card>, Error> = s.chars().take(5).map(|c| c.try_into()).collect();
        Ok(Self(hand?.try_into().unwrap()))
    }
}
impl Hand {
    fn hand_type(&self) -> HandType {
        self.into()
    }
}

impl PartialEq for Hand {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type() != other.hand_type() {
            return self.hand_type().cmp(&other.hand_type());
        }

        for (c1, c2) in self.0.iter().zip(other.0.iter()) {
            if c1 > c2 {
                return std::cmp::Ordering::Greater;
            }
            if c1 < c2 {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    }
}
impl Eq for Hand {
    fn assert_receiver_is_total_eq(&self) {}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}
impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut reps = [0usize; 13];
        value.0.iter().for_each(|card| {
            reps[card.value() - 2] += 1;
        });

        match reps {
            _ if reps.iter().any(|&rep| rep == 5) => HandType::FiveOfAKind,
            _ if reps.iter().any(|&rep| rep == 4) => HandType::FourOfAKind,
            _ if reps.iter().any(|&rep| rep == 3) && reps.iter().any(|&rep| rep == 2) => {
                HandType::FullHouse
            }
            _ if reps.iter().any(|&rep| rep == 3) => HandType::ThreeOfAKind,
            _ if reps.iter().filter(|&rep| *rep == 2).count() == 2 => HandType::TwoPair,
            _ if reps.iter().any(|&rep| rep == 2) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    hand: Hand,
    bid: usize,
}
impl FromStr for Game {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let hand = Hand::from_str(s.next().unwrap())?;
        let bid = s.next().unwrap().parse()?;
        Ok(Self { hand, bid })
    }
}
impl Game {
    pub fn bid(&self) -> usize {
        self.bid
    }
}
impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}
impl Eq for Game {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}
