use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Error};
use util::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    EmptySpace,
    RightwardMirror,
    LeftwardMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl TryFrom<char> for Tile {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::EmptySpace),
            '|' => Ok(Self::VerticalSplitter),
            '-' => Ok(Self::HorizontalSplitter),
            '/' => Ok(Self::RightwardMirror),
            '\\' => Ok(Self::LeftwardMirror),
            _ => Err(anyhow!("fail to convert tile")),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::EmptySpace => '.',
            Self::VerticalSplitter => '|',
            Self::HorizontalSplitter => '-',
            Self::RightwardMirror => '/',
            Self::LeftwardMirror => '\\',
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub struct Beam {
    direction: Direction,
    pos: (usize, usize),
}
impl Beam {
    /// Returns a new beam if the current tile splits the beam in 2
    fn mutate(&mut self, grid: &Grid<Tile>) -> Option<Self> {
        let tile = grid.get((self.pos.0, self.pos.1)).unwrap();
        match tile {
            Tile::EmptySpace => None,
            Tile::HorizontalSplitter => match self.direction {
                Direction::Right | Direction::Left => None,
                Direction::Up | Direction::Down => {
                    // Create a new beam that goes to left direction
                    self.direction = Direction::Right;
                    Some(Self {
                        direction: Direction::Left,
                        pos: self.pos,
                    })
                }
            },
            Tile::VerticalSplitter => match self.direction {
                Direction::Right | Direction::Left => {
                    self.direction = Direction::Down;
                    Some(Self {
                        direction: Direction::Up,
                        pos: self.pos,
                    })
                }
                Direction::Up | Direction::Down => None,
            },
            Tile::LeftwardMirror => {
                match self.direction {
                    Direction::Up => self.direction = Direction::Left,
                    Direction::Down => self.direction = Direction::Right,
                    Direction::Left => self.direction = Direction::Up,
                    Direction::Right => self.direction = Direction::Down,
                }
                None
            }
            Tile::RightwardMirror => {
                match self.direction {
                    Direction::Up => self.direction = Direction::Right,
                    Direction::Down => self.direction = Direction::Left,
                    Direction::Left => self.direction = Direction::Down,
                    Direction::Right => self.direction = Direction::Up,
                }
                None
            }
        }
    }
}

//6921
pub struct Puzzle1 {
    grid: Grid<Tile>,
    energized: Grid<bool>,
}
impl Puzzle1 {
    pub fn run(&mut self) {
        let mut beams = vec![Beam {
            pos: (0, 0),
            direction: Direction::Right,
        }];
        self.energized.set((0, 0), true);
        for _ in 0..650 {
            // Change the direction of the beams according to the current standing tile, and spawn new beams if
            // splitters were encountered
            let mut new_beams: Vec<Beam> = beams
                .iter_mut()
                .filter_map(|beam| beam.mutate(&self.grid))
                .collect();

            // Add any new beam to the beam list
            beams.append(&mut new_beams);

            // Move all the beams
            beams = beams.iter().filter_map(|beam| self.mv(beam)).collect();

            // After moving all the beams, energize the grid
            for (x, y) in beams.iter().map(|beam| beam.pos) {
                self.energized.set((x, y), true);
            }
        }
        self.display_energized();
    }
    /// returns None if the beam is exiting the grid
    fn mv(&self, beam: &Beam) -> Option<Beam> {
        let (x, y) = beam.pos;
        let mut new_beam = *beam;
        match beam.direction {
            Direction::Up => {
                if y == 0 {
                    return None;
                }
                new_beam.pos.1 -= 1;
                Some(new_beam)
            }
            Direction::Down => {
                if y == self.grid.nb_rows() - 1 {
                    return None;
                }
                new_beam.pos.1 += 1;
                Some(new_beam)
            }
            Direction::Left => {
                if x == 0 {
                    return None;
                }
                new_beam.pos.0 -= 1;
                Some(new_beam)
            }
            Direction::Right => {
                if x == self.grid.nb_columns() - 1 {
                    return None;
                }
                new_beam.pos.0 += 1;
                Some(new_beam)
            }
        }
    }

    pub fn count_energized_tiles(&self) -> usize {
        self.energized.find_all(&true).len()
    }

    pub fn display_energized(&self) {
        for row in self.energized.rows() {
            for col in row.iter() {
                if *col {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
        // println!("{}", self.energized)
    }
}
impl FromStr for Puzzle1 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<Vec<Tile>>, Error> = s
            .lines()
            .map(|line| line.chars().map(Tile::try_from).collect())
            .collect();
        let v = v?;
        let e = vec![vec![false; v[0].len()]; v.len()];
        Ok(Self {
            grid: Grid::new(v),
            energized: Grid::new(e),
        })
    }
}
impl Display for Puzzle1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}
