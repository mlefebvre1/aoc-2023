use std::{fmt::Display, num::ParseIntError, str::FromStr};

use anyhow::{anyhow, Error};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpringCondition {
    Damaged,
    Operational,
    Unknown,
}

impl TryFrom<char> for SpringCondition {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(anyhow!("failed to convert to spring condition")),
        }
    }
}
impl Display for SpringCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damaged => write!(f, "#"),
            Self::Operational => write!(f, "."),
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl From<Bin> for SpringCondition {
    fn from(value: Bin) -> Self {
        match value {
            Bin::One => Self::Damaged,
            Bin::Zero => Self::Operational,
        }
    }
}

#[derive(Debug)]
pub struct Row {
    springs: Vec<SpringCondition>,
    contiguous_springs: Vec<usize>,
}

impl FromStr for Row {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let springs: Result<Vec<SpringCondition>, Error> =
            s.next().unwrap().chars().map(|c| c.try_into()).collect();
        let contiguous_springs: Result<Vec<usize>, ParseIntError> =
            s.next().unwrap().split(',').map(|ss| ss.parse()).collect();
        Ok(Self {
            springs: springs?,
            contiguous_springs: contiguous_springs?,
        })
    }
}

impl Row {
    pub fn nb_arrangements(&self) -> usize {
        let unknown_indexes: Vec<usize> = self
            .springs
            .iter()
            .enumerate()
            .filter(|(_i, &spring)| spring == SpringCondition::Unknown)
            .map(|(i, _)| i)
            .collect();

        let nb_unkowns = unknown_indexes.len();
        let binaries = (0..2usize.pow(nb_unkowns as u32)).map(|i| Binary::from_int(i, nb_unkowns));
        binaries
            .filter(|binary| {
                let mut row = self.springs.clone();
                unknown_indexes
                    .iter()
                    .zip(binary.as_ref())
                    .for_each(|(&i, &bin)| {
                        row[i] = bin.into();
                    });
                self.new_row_valid(&row)
            })
            .count()
    }

    pub fn new_row_valid(&self, row: &[SpringCondition]) -> bool {
        let mut row_it = row.iter();
        for nb_contiguous in self.contiguous_springs.iter() {
            // find next broken spring
            let row_it = row_it
                .by_ref()
                .skip_while(|&spring| *spring == SpringCondition::Operational);
            let nb_damaged = row_it
                .take_while(|&spring| *spring == SpringCondition::Damaged)
                .count();
            if nb_damaged != *nb_contiguous {
                return false;
            }
        }
        // Make sure the rest are all operational
        row_it.all(|&spring| spring == SpringCondition::Operational)
    }
    // pub fn find_arrangements(&self) -> usize {}
}

#[derive(Debug, Clone)]
struct Binary(Vec<Bin>);
impl Binary {
    pub fn from_int(mut n: usize, nb_bits: usize) -> Self {
        let mut binary = vec![Bin::Zero; nb_bits];
        for b in binary.iter_mut() {
            let r = n % 2;
            n /= 2;
            *b = r.into();
            if n == 0 {
                break;
            }
        }
        Self(binary)
    }
}
impl AsRef<Vec<Bin>> for Binary {
    fn as_ref(&self) -> &Vec<Bin> {
        &self.0
    }
}
impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();
        self.0.iter().for_each(|b| s.push_str(&b.to_string()));
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy)]
enum Bin {
    One,
    Zero,
}
impl From<usize> for Bin {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::One,
            _ => Self::Zero,
        }
    }
}
impl Display for Bin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "#"),
            Self::Zero => write!(f, "."),
        }
    }
}
