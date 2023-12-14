use std::{fmt::Display, ops::Range, str::FromStr};

use anyhow::{anyhow, Error, Ok};
use util::grid::Grid;

pub struct Patterns(Vec<Pattern>);
impl FromStr for Patterns {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut patterns = vec![];
        loop {
            let vi = lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\n");
            if vi.is_empty() {
                break;
            }
            let pattern = Pattern::from_str(&vi)?;
            patterns.push(pattern);
        }
        Ok(Self(patterns))
    }
}
impl AsRef<Vec<Pattern>> for Patterns {
    fn as_ref(&self) -> &Vec<Pattern> {
        &self.0
    }
}
impl Patterns {
    pub fn reflections_score(&self) -> usize {
        self.0
            .iter()
            .map(|pattern| pattern.find_reflection().score())
            .sum()
    }
}

pub struct Pattern(Grid<Element>);
impl FromStr for Pattern {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<Vec<Element>>, Error> = s
            .lines()
            .map(|line| line.chars().map(Element::try_from).collect())
            .collect();
        Ok(Self(Grid::new(v?)))
    }
}
impl AsRef<Grid<Element>> for Pattern {
    fn as_ref(&self) -> &Grid<Element> {
        &self.0
    }
}
impl Pattern {
    pub fn find_reflection(&self) -> Reflection {
        if let Some(n) = (1..self.0.nb_rows()).find(|&i| {
            let (up, down) = Self::calculate_ranges(i, self.0.nb_rows());
            let mut up_slice = self.0.rows_slice(up.start, up.end);
            up_slice.reverse();
            let down_slice = self.0.rows_slice(down.start, down.end);
            up_slice == down_slice
        }) {
            return Reflection::Horizontal(n);
        }

        if let Some(n) = (1..self.0.nb_columns()).find(|&i| {
            let (left, right) = Self::calculate_ranges(i, self.0.nb_columns());
            let mut left_slice = self.0.columns_slice(left.start, left.end);
            left_slice.reverse();
            let right_slice = self.0.columns_slice(right.start, right.end);
            left_slice == right_slice
        }) {
            return Reflection::Vertical(n);
        }

        Reflection::NotFound
    }
    fn calculate_ranges(i: usize, m: usize) -> (Range<usize>, Range<usize>) {
        let span = i.min(m - i);
        let up = if i > m / 2 { (i - span)..i } else { 0..i };
        let down = if i > m / 2 { i..m } else { i..(2 * span) };
        (up, down)
    }
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Ash,
    Rock,
}
impl TryFrom<char> for Element {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            _ => Err(anyhow!("not an element!")),
        }
    }
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ash => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

pub enum Reflection {
    Horizontal(usize),
    Vertical(usize),
    NotFound,
}
impl Reflection {
    pub fn score(&self) -> usize {
        match self {
            Self::Horizontal(n) => *n * 100,
            Self::Vertical(n) => *n,
            Self::NotFound => 0,
        }
    }
}
