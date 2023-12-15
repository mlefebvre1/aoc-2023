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
    pub fn reflections_score_smudge(&self) -> usize {
        self.0
            .iter()
            .map(|pattern| pattern.find_smudge().score())
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
    pub fn find_smudge(&self) -> Reflection {
        for i in 1..self.0.nb_rows() {
            let (up_range, down_range) = Self::calculate_ranges(i, self.0.nb_rows());
            let mut up_slice = self.0.rows_slice(up_range.start, up_range.end);
            up_slice.reverse();
            let down_slice = self.0.rows_slice(down_range.start, down_range.end);
            for row in 0..up_slice.len() {
                if up_slice[row] != down_slice[row]
                    && up_slice[row]
                        .iter()
                        .zip(down_slice[row].iter())
                        .filter(|(up, down)| up != down)
                        .count()
                        == 1
                {
                    let mut up_slice_mod = up_slice.clone();
                    for col in 0..up_slice[row].len() {
                        if up_slice[row][col] == down_slice[row][col] {
                            up_slice_mod[row][col].smudge();
                            break;
                        }
                    }
                    if up_slice_mod[row] == down_slice[row] {
                        println!("{i}");
                    }
                }
            }
            // for (up_row, down_row) in up_slice.iter().zip(down_slice.iter()) {
            // if up_row != down_row
            //     && up_row
            //         .iter()
            //         .zip(down_row.iter())
            //         .filter(|(up, down)| up != down)
            //         .count()
            //         == 1
            // {
            //     let up_slice2 = up_slice.clone();
            //     let down_slice2 = down_slice.clone();
            //     if let Some(i) = up_row
            //         .iter()
            //         .zip(down_row.iter())
            //         .position(|(up_item, down_item)| up_item != down_item)
            //     {
            //         up_row[i].smudge();
            //     }
            //     println!("{up_row:?}\n{down_row:?}");
            //     println!("{i}");
            // }
        }
        println!();

        // (1..self.0.nb_rows())
        //     .filter(|&i| {
        //         let (up, down) = Self::calculate_ranges(i, self.0.nb_rows());
        //         let mut up_slice = self.0.rows_slice(up.start, up.end);
        //         up_slice.reverse();
        //         let down_slice = self.0.rows_slice(down.start, down.end);
        //         up_slice
        //             .iter()
        //             .zip(down_slice.iter())
        //             .filter(|(up_row, down_row)| {
        //                 (up_row != down_row)
        //                     && up_row
        //                         .iter()
        //                         .zip(down_row.iter())
        //                         .filter(|(up, down)| up != down)
        //                         .count()
        //                         == 1
        //             })
        //             .inspect(|(up_row, down_row)| println!("{up_row:?}\n{down_row:?}"))
        //             .count()
        //             == 1
        //     })
        //     .for_each(|i| println!("{i}"));

        Reflection::NotFound
    }

    fn calculate_ranges(i: usize, m: usize) -> (Range<usize>, Range<usize>) {
        let span = i.min(m - i);
        let up = if i > m / 2 { (i - span)..i } else { 0..i };
        let down = if i > m / 2 { i..m } else { i..(2 * span) };
        (up, down)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Element {
    Ash,
    Rock,
}
impl Element {
    fn smudge(&mut self) {
        match self {
            Self::Ash => *self = Self::Rock,
            Self::Rock => *self = Self::Ash,
        }
    }
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
