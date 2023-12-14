use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Error};
use util::grid::Grid;

#[derive(Debug, Clone, PartialEq)]
pub enum SpaceType {
    EmptySpace,
    Galaxy,
    SpaceExpansion,
}
impl Display for SpaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::EmptySpace => '.',
            Self::Galaxy => '#',
            Self::SpaceExpansion => '*',
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
        let columns_index_with_no_galaxies: Vec<usize> = self
            .0
            .columns_vec()
            .into_iter()
            .enumerate()
            .filter(|(_i, col)| col.iter().all(|&s| *s == SpaceType::EmptySpace))
            .map(|(i, _)| i)
            .collect();

        for row_index in rows_index_with_no_galaxies.into_iter() {
            self.0.replace_row(
                row_index,
                vec![SpaceType::SpaceExpansion; self.0.nb_columns()],
            );
        }

        for column_index in columns_index_with_no_galaxies.into_iter() {
            self.0.replace_column(
                column_index,
                vec![SpaceType::SpaceExpansion; self.0.nb_rows()],
            )
        }
    }

    pub fn run<const EXPAND_FACTOR: usize>(&self) -> usize {
        let galaxies = self.0.find_all(&SpaceType::Galaxy);
        let pairs =
            (0..galaxies.len() - 1).flat_map(|i| (i + 1..galaxies.len()).map(move |j| (i, j)));
        pairs
            .map(|(g1, g2)| {
                let (g1x, g1y) = galaxies[g1];
                let (g2x, g2y) = galaxies[g2];
                let dx = (g1x as isize - g2x as isize).abs();
                let dy = (g1y as isize - g2y as isize).abs();
                let nb_space_expansion =
                    self.nb_space_expansion_crossed(galaxies[g1], galaxies[g2]);
                (dx + dy) as usize + nb_space_expansion * (EXPAND_FACTOR - 1)
            })
            .sum()
    }

    fn nb_space_expansion_crossed(
        &self,
        (p1x, p1y): (usize, usize),
        (p2x, p2y): (usize, usize),
    ) -> usize {
        let nb_space_expansion_rows = if p2y > p1y {
            (p1y..p2y)
                .filter(|&y| self.0.get((p1x, y)).unwrap() == &SpaceType::SpaceExpansion)
                .count()
        } else {
            (p2y..p1y)
                .filter(|&y| self.0.get((p1x, y)).unwrap() == &SpaceType::SpaceExpansion)
                .count()
        };

        let nb_space_expansion_columns = if p2x > p1x {
            (p1x..p2x)
                .filter(|&x| self.0.get((x, p1y)).unwrap() == &SpaceType::SpaceExpansion)
                .count()
        } else {
            (p2x..p1x)
                .filter(|&x| self.0.get((x, p1y)).unwrap() == &SpaceType::SpaceExpansion)
                .count()
        };
        nb_space_expansion_rows + nb_space_expansion_columns
    }
}
impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
