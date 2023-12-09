use std::str::FromStr;

use anyhow::{anyhow, Error};

#[derive(Debug, PartialEq, PartialOrd)]
struct Card<const J: usize>(usize);
impl<const J: usize> TryFrom<char> for Card<J> {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            c @ '2'..='9' => Ok(Self(
                c.to_digit(10).ok_or(anyhow!("fail to convert to digit"))? as usize,
            )),
            'T' => Ok(Self(10)),
            'J' => Ok(Self(J)),
            'Q' => Ok(Self(12)),
            'K' => Ok(Self(13)),
            'A' => Ok(Self(14)),
            _ => Err(anyhow!("character '{value}' can't be converted to a Card")),
        }
    }
}
impl<const J: usize> Card<J> {
    pub fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
struct Hand<const J: usize>([Card<J>; 5]);
impl<const J: usize> FromStr for Hand<J> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Q97J7
        let hand: Result<Vec<Card<J>>, Error> = s.chars().take(5).map(|c| c.try_into()).collect();
        Ok(Self(hand?.try_into().unwrap()))
    }
}
impl<const J: usize> Hand<J> {
    fn hand_type(&self) -> HandType {
        self.into()
    }
}

impl<const J: usize> PartialEq for Hand<J> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<const J: usize> PartialOrd for Hand<J> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<const J: usize> Ord for Hand<J> {
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
impl<const J: usize> Eq for Hand<J> {
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
impl HandType {
    fn is_two_pair(reps: &[usize; 14]) -> bool {
        /*
            2, 2
            2, 1+1J         <- not possible, 3 of a kind
            2, 2J           <- not possible, 4 of a kind
            1J+1J, 1J+1     <- not possible, 4 of a kind
            1J+1J, 1J+1J    <- not possible, 4 of a kind
        */
        reps.iter().filter(|&rep| *rep == 2).count() == 2
    }
    fn is_full_house(reps: &[usize; 14]) -> bool {
        /*

            2, 3
            2, 2+1J
            2, 1+2J     -> not possible, 4 of a kind
            2, 3J       -> not possible, 5 of a kind

            1+1J, 3     -> not possible, 4 of a kind
            1+1J, 2+1J  -> not possible, 4 of a kind
            1+1J, 1+2J  -> not possible, 4 of a kind
            1+1J, 3J    -> not possible, 5 of a kind

            2J, 3       -> not possible, 5 of a kind
            2J, 2+1J    -> not possible, 5 of a kind
            2J, 1+2J    -> not possible, 5 of a kind
            2J, 3J      -> not possible, 5 of a kind
        */
        let nb_wildcards = reps[0];
        let nb_pairs = reps.iter().skip(1).filter(|&rep| *rep == 2).count();
        let nb_three_of_a_kind = reps.iter().skip(1).filter(|&rep| *rep == 3).count();
        (nb_pairs == 1 && nb_three_of_a_kind == 1) || (nb_pairs == 2 && nb_wildcards == 1)
    }
}
impl<const J: usize> From<&Hand<J>> for HandType {
    fn from(value: &Hand<J>) -> Self {
        let mut reps = [0usize; 14];
        value.0.iter().for_each(|card| {
            reps[card.value() - 1] += 1;
        });
        let nb_wildcards = reps[0];
        match reps {
            _ if reps.iter().skip(1).any(|&rep| rep + nb_wildcards == 5) => HandType::FiveOfAKind,
            _ if reps.iter().skip(1).any(|&rep| rep + nb_wildcards == 4) => HandType::FourOfAKind,
            _ if Self::is_full_house(&reps) => HandType::FullHouse,
            _ if reps.iter().skip(1).any(|&rep| rep + nb_wildcards == 3) => HandType::ThreeOfAKind,
            _ if Self::is_two_pair(&reps) => HandType::TwoPair,
            _ if reps.iter().skip(1).any(|&rep| rep + nb_wildcards == 2) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug)]
pub struct Game<const J: usize> {
    hand: Hand<J>,
    bid: usize,
}
impl<const J: usize> FromStr for Game<J> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Hand  Bid
        // Q97J7 740
        let mut s = s.split_whitespace();
        let hand = Hand::from_str(s.next().unwrap())?;
        let bid = s.next().unwrap().parse()?;
        Ok(Self { hand, bid })
    }
}
impl<const J: usize> Game<J> {
    pub fn bid(&self) -> usize {
        self.bid
    }
}
impl<const J: usize> PartialEq for Game<J> {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}
impl<const J: usize> Eq for Game<J> {
    fn assert_receiver_is_total_eq(&self) {}
}
impl<const J: usize> PartialOrd for Game<J> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<const J: usize> Ord for Game<J> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

#[derive(Debug)]
pub struct Puzzle<const J: usize>(Vec<Game<J>>);
impl<const J: usize> FromStr for Puzzle<J> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let puzzle: Result<Vec<Game<J>>, Error> = s.lines().map(Game::from_str).collect();
        Ok(Self(puzzle?))
    }
}
impl<const J: usize> Puzzle<J> {
    pub fn run(&mut self) -> usize {
        self.0.sort();
        self.0
            .iter()
            .enumerate()
            .map(|(rank, game)| (rank + 1) * game.bid())
            .sum()
    }
}
