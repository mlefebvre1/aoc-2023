use std::{collections::HashMap, fmt::Display, str::FromStr};

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
    fn keep_beam(&self, visit: &Visit) -> bool {
        match self.direction {
            Direction::Up => !visit.u,
            Direction::Down => !visit.d,
            Direction::Left => !visit.l,
            Direction::Right => !visit.r,
        }
    }

    /// Returns a new beam if the current tile splits the beam in 2, return the original beam if the obstacle
    /// was not reached with that direction yet
    fn mutate(mut beam: Beam, grid: &Grid<Tile>) -> Vec<Self> {
        let tile = grid.get((beam.pos.0, beam.pos.1)).unwrap();

        match tile {
            Tile::EmptySpace => vec![beam],
            Tile::HorizontalSplitter => match beam.direction {
                Direction::Right | Direction::Left => {
                    vec![beam]
                }
                Direction::Up | Direction::Down => {
                    // Create a new beam that goes to left direction
                    beam.direction = Direction::Right;
                    vec![
                        beam,
                        Self {
                            direction: Direction::Left,
                            pos: beam.pos,
                        },
                    ]
                }
            },
            Tile::VerticalSplitter => match beam.direction {
                Direction::Right | Direction::Left => {
                    beam.direction = Direction::Down;
                    vec![
                        beam,
                        Self {
                            direction: Direction::Up,
                            pos: beam.pos,
                        },
                    ]
                }
                Direction::Up | Direction::Down => vec![beam],
            },
            Tile::LeftwardMirror => {
                match beam.direction {
                    Direction::Up => beam.direction = Direction::Left,
                    Direction::Down => beam.direction = Direction::Right,
                    Direction::Left => beam.direction = Direction::Up,
                    Direction::Right => beam.direction = Direction::Down,
                }
                vec![beam]
            }
            Tile::RightwardMirror => {
                match beam.direction {
                    Direction::Up => beam.direction = Direction::Right,
                    Direction::Down => beam.direction = Direction::Left,
                    Direction::Left => beam.direction = Direction::Down,
                    Direction::Right => beam.direction = Direction::Up,
                }
                vec![beam]
            }
        }
    }
}

// determines if a beam already reached the obstacle with the direction
#[derive(Debug, Clone, Copy, Default)]
struct Visit {
    u: bool,
    d: bool,
    l: bool,
    r: bool,
}

impl Visit {
    fn update(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.u = true,
            Direction::Down => self.d = true,
            Direction::Left => self.l = true,
            Direction::Right => self.r = true,
        }
    }
}

//6921
pub struct Puzzle {
    grid: Grid<Tile>,
    energized: Grid<bool>,
    visits: HashMap<(usize, usize), Visit>,
}
impl Puzzle {
    pub fn initial_beams_part1(&self) -> Vec<Vec<Beam>> {
        vec![vec![Beam {
            pos: (0, 0),
            direction: Direction::Right,
        }]]
    }
    pub fn initial_beams_part2(&self) -> Vec<Vec<Beam>> {
        let mut top_beams: Vec<Vec<Beam>> = (1..self.grid.nb_columns() - 2)
            .map(|x| {
                vec![Beam {
                    pos: (x, 0),
                    direction: Direction::Down,
                }]
            })
            .collect();
        let mut down_beams: Vec<Vec<Beam>> = (1..self.grid.nb_columns() - 2)
            .map(|x| {
                vec![Beam {
                    pos: (x, self.grid.nb_rows() - 1),
                    direction: Direction::Up,
                }]
            })
            .collect();
        let mut left_beams: Vec<Vec<Beam>> = (1..self.grid.nb_rows() - 2)
            .map(|y| {
                vec![Beam {
                    pos: (0, y),
                    direction: Direction::Right,
                }]
            })
            .collect();
        let mut right_beams: Vec<Vec<Beam>> = (1..self.grid.nb_rows() - 2)
            .map(|y| {
                vec![Beam {
                    pos: (self.grid.nb_columns() - 1, y),
                    direction: Direction::Left,
                }]
            })
            .collect();
        let mut corners = vec![
            // top-left
            vec![Beam {
                pos: (0, 0),
                direction: Direction::Right,
            }],
            vec![Beam {
                pos: (0, 0),
                direction: Direction::Down,
            }],
            // Top-right
            vec![Beam {
                pos: (self.grid.nb_columns() - 1, 0),
                direction: Direction::Left,
            }],
            vec![Beam {
                pos: (self.grid.nb_columns() - 1, 0),
                direction: Direction::Down,
            }],
            // Bottom-right
            vec![Beam {
                pos: (self.grid.nb_columns() - 1, self.grid.nb_rows() - 1),
                direction: Direction::Left,
            }],
            vec![Beam {
                pos: (self.grid.nb_columns() - 1, self.grid.nb_rows() - 1),
                direction: Direction::Up,
            }],
            //Bottom-left
            vec![Beam {
                pos: (0, self.grid.nb_rows() - 1),
                direction: Direction::Right,
            }],
            vec![Beam {
                pos: (0, self.grid.nb_rows() - 1),
                direction: Direction::Up,
            }],
        ];
        corners.append(&mut top_beams);
        corners.append(&mut down_beams);
        corners.append(&mut right_beams);
        corners.append(&mut left_beams);
        corners
    }

    pub fn run(&mut self, part1: bool) -> usize {
        let initial_beams = if part1 {
            self.initial_beams_part1()
        } else {
            self.initial_beams_part2()
        };
        initial_beams
            .into_iter()
            .map(|mut beams| {
                self.energized.assign(false); //reset to not energized
                self.clear_visits();

                loop {
                    // for each beam position energize the grid
                    beams.iter().map(|beam| beam.pos).for_each(|(x, y)| {
                        self.energized.set((x, y), true);
                    });

                    // Change the direction of the beams according to the current standing tile, and spawn new beams if
                    // splitters were encountered
                    beams = beams
                        .iter()
                        .flat_map(|&beam| Beam::mutate(beam, &self.grid))
                        .collect();

                    // remove beams that already visited the path
                    beams = beams
                        .into_iter()
                        .filter(|beam| {
                            let tile = self.grid.get(beam.pos).unwrap();
                            if *tile != Tile::EmptySpace {
                                beam.keep_beam(&self.visits[&beam.pos])
                            } else {
                                true
                            }
                        })
                        .collect::<Vec<_>>();

                    // update obstacle states
                    beams.iter().for_each(|beam| {
                        let tile = self.grid.get(beam.pos).unwrap();
                        if *tile != Tile::EmptySpace {
                            self.visits
                                .get_mut(&beam.pos)
                                .unwrap()
                                .update(beam.direction);
                        }
                    });

                    // Move all the beams
                    beams = beams.iter().filter_map(|beam| self.mv(beam)).collect();

                    // if there's no beam, then we're done
                    if beams.is_empty() {
                        return self.count_energized_tiles();
                    }
                }
            })
            .max()
            .unwrap()
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

    pub fn clear_visits(&mut self) {
        self.visits.values_mut().for_each(|v| {
            v.u = false;
            v.d = false;
            v.l = false;
            v.r = false;
        })
    }

    pub fn count_energized_tiles(&self) -> usize {
        self.energized.find_all(&true).len()
    }
}
impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<Vec<Tile>>, Error> = s
            .lines()
            .map(|line| line.chars().map(Tile::try_from).collect())
            .collect();
        let v = v?;
        let e = vec![vec![false; v[0].len()]; v.len()];
        let visits = v
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &col)| {
                        (col != Tile::EmptySpace).then_some(((x, y), Visit::default()))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok(Self {
            grid: Grid::new(v),
            energized: Grid::new(e),
            visits,
        })
    }
}
impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}
