use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Error};
use util::grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Element {
    EmptySpace,
    RoundedRock,
    CubeShapedRock,
}
impl TryFrom<char> for Element {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::RoundedRock),
            '#' => Ok(Self::CubeShapedRock),
            '.' => Ok(Self::EmptySpace),
            _ => Err(anyhow!("can't convert char to element")),
        }
    }
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::EmptySpace => ".",
            Self::RoundedRock => "O",
            Self::CubeShapedRock => "#",
        };
        write!(f, "{s}")
    }
}

pub struct Platform {
    grid: Grid<Element>,
    rounded_rocks: Vec<(usize, usize)>,
}
impl FromStr for Platform {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<Vec<Element>>, Error> = s
            .lines()
            .map(|line| line.chars().map(Element::try_from).collect())
            .collect();
        let v = v?;
        let rounded_rocks = v
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &e)| e == Element::RoundedRock)
                    .map(move |(x, _)| (x, y))
            })
            .collect();

        Ok(Self {
            grid: Grid::new(v),
            rounded_rocks,
        })
    }
}
impl Platform {
    pub fn current_rounded_rocks(&self) -> &Vec<(usize, usize)> {
        &self.rounded_rocks
    }
    pub fn clone_rounded_rocks(&self) -> Vec<(usize, usize)> {
        self.rounded_rocks.clone()
    }
    pub fn tilt_north(&mut self) {
        loop {
            let mut stable = true;
            for (x, y) in self.rounded_rocks.iter_mut() {
                if *y == 0 {
                    continue;
                }

                if let Some(e) = self.grid.get((*x, *y - 1)) {
                    if *e == Element::EmptySpace {
                        self.grid.set((*x, *y), Element::EmptySpace);
                        *y -= 1;
                        self.grid.set((*x, *y), Element::RoundedRock);
                        stable = false;
                    }
                }
            }
            if stable {
                return;
            }
        }
    }
    pub fn tilt_west(&mut self) {
        loop {
            let mut stable = true;
            for (x, y) in self.rounded_rocks.iter_mut() {
                if *x == 0 {
                    continue;
                }

                if let Some(e) = self.grid.get((*x - 1, *y)) {
                    if *e == Element::EmptySpace {
                        self.grid.set((*x, *y), Element::EmptySpace);
                        *x -= 1;
                        self.grid.set((*x, *y), Element::RoundedRock);
                        stable = false;
                    }
                }
            }
            if stable {
                return;
            }
        }
    }
    pub fn tilt_east(&mut self) {
        loop {
            let mut stable = true;
            for (x, y) in self.rounded_rocks.iter_mut() {
                if *x == self.grid.nb_columns() - 1 {
                    continue;
                }

                if let Some(e) = self.grid.get((*x + 1, *y)) {
                    if *e == Element::EmptySpace {
                        self.grid.set((*x, *y), Element::EmptySpace);
                        *x += 1;
                        self.grid.set((*x, *y), Element::RoundedRock);
                        stable = false;
                    }
                }
            }
            if stable {
                return;
            }
        }
    }
    pub fn tilt_south(&mut self) {
        loop {
            let mut stable = true;
            for (x, y) in self.rounded_rocks.iter_mut() {
                if *y == self.grid.nb_rows() - 1 {
                    continue;
                }

                if let Some(e) = self.grid.get((*x, *y + 1)) {
                    if *e == Element::EmptySpace {
                        self.grid.set((*x, *y), Element::EmptySpace);
                        *y += 1;
                        self.grid.set((*x, *y), Element::RoundedRock);
                        stable = false;
                    }
                }
            }
            if stable {
                return;
            }
        }
    }

    pub fn run_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east()
    }

    pub fn calculate_score(&self) -> usize {
        self.rounded_rocks
            .iter()
            .map(|(_, y)| (self.grid.nb_rows() - y))
            .sum()
    }
}
impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[test]
fn test_range() {
    for i in (1..11).rev() {
        println!("{i}");
    }
}
