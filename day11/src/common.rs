use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Error};
use util::grid::Grid;

#[derive(Debug, Clone, PartialEq)]
pub enum SpaceType {
    EmptySpace,
    Galaxy,
}
impl Display for SpaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::EmptySpace => '.',
            Self::Galaxy => '#',
        };
        write!(f, "{s}")
    }
}

impl TryFrom<char> for SpaceType {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::EmptySpace),
            '#' => Ok(Self::Galaxy),
            _ => Err(anyhow!("failed to convert character to SpaceType")),
        }
    }
}

#[derive(Debug)]
pub struct Image(Grid<SpaceType>);
impl FromStr for Image {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .lines()
            .map(|line| {
                let v: Vec<SpaceType> = line
                    .chars()
                    .map(|c| SpaceType::try_from(c).unwrap())
                    .collect();
                v
            })
            .collect();
        let mut image = Self(Grid::new(v));
        image.expand();
        Ok(image)
    }
}
impl Image {
    pub fn expand(&mut self) {
        let rows_index_with_no_galaxies: Vec<usize> = self
            .0
            .rows()
            .enumerate()
            .filter(|(_i, row)| row.iter().all(|s| *s == SpaceType::EmptySpace))
            .map(|(i, _)| i)
            .collect();
        for (i, row_index) in rows_index_with_no_galaxies.iter().enumerate() {
            self.0.insert_row(
                row_index + i,
                vec![SpaceType::EmptySpace; self.0.nb_columns()],
            );
        }

        let columns_index_with_no_galaxies: Vec<usize> = self
            .0
            .columns()
            .into_iter()
            .enumerate()
            .filter(|(_i, col)| col.iter().all(|&s| *s == SpaceType::EmptySpace))
            .map(|(i, _)| i)
            .collect();

        for (i, column_index) in columns_index_with_no_galaxies.iter().enumerate() {
            self.0.insert_column(
                column_index + i,
                vec![SpaceType::EmptySpace; self.0.nb_rows()],
            )
        }
    }

    pub fn run(&self) -> usize {
        let galaxies = self.0.find_all(&SpaceType::Galaxy);
        let pairs =
            (0..galaxies.len() - 1).flat_map(|i| (i + 1..galaxies.len()).map(move |j| (i, j)));
        pairs
            .map(|(g1, g2)| {
                let (g1x, g1y) = galaxies[g1];
                let (g2x, g2y) = galaxies[g2];
                let dx = (g1x as isize - g2x as isize).abs();
                let dy = (g1y as isize - g2y as isize).abs();
                (dx + dy) as usize
            })
            .sum()
    }
}
impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
