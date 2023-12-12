use std::str::FromStr;

use anyhow::{anyhow, Error};
use util::grid::Grid;

#[derive(Debug, PartialEq)]
enum TileType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Starting,
}
impl TryFrom<char> for TileType {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Starting),
            _ => Err(anyhow!("can't convert '{value}' to tile type")),
        }
    }
}

#[derive(PartialEq, Debug)]
enum VerticalDirection {
    North,
    South,
    Undef,
}
#[derive(Debug, PartialEq)]
enum HorizontalDirection {
    East,
    West,
    Undef,
}

#[derive(Debug)]
pub struct Diagram(Grid<TileType>);
impl FromStr for Diagram {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<Vec<TileType>>, Error> = s
            .lines()
            .map(|line| line.chars().map(TileType::try_from).collect())
            .collect();
        Ok(Self(Grid::new(v?)))
    }
}
impl Diagram {
    #[allow(dead_code)]
    pub fn display(&self) {
        self.0.rows().for_each(|row| {
            println!("{row:?}");
        })
    }
    pub fn grid_shape(&self) -> (usize, usize) {
        self.0.shape()
    }
    pub fn initial_flow(&self) -> ((usize, usize), HorizontalDirection, VerticalDirection) {
        let mut tile_loc = self.0.find(&TileType::Starting).unwrap();
        let mut vertical = VerticalDirection::Undef;
        let mut horizontal = HorizontalDirection::Undef;

        for (x, y, hor, vert) in [
            (
                Some(tile_loc.0 + 1),
                Some(tile_loc.1),
                HorizontalDirection::East,
                VerticalDirection::Undef,
            ),
            (
                Some(tile_loc.0),
                Some(tile_loc.1 + 1),
                HorizontalDirection::Undef,
                VerticalDirection::South,
            ),
            (
                tile_loc.0.checked_sub(1),
                Some(tile_loc.1),
                HorizontalDirection::West,
                VerticalDirection::Undef,
            ),
            (
                Some(tile_loc.0),
                tile_loc.1.checked_sub(1),
                HorizontalDirection::Undef,
                VerticalDirection::North,
            ),
        ] {
            if let (Some(x), Some(y)) = (x, y) {
                let tile = self.0.get((x, y)).unwrap();
                if hor == HorizontalDirection::East {
                    match tile {
                        TileType::NorthWest | TileType::SouthWest | TileType::Horizontal => {
                            return ((x, y), hor, vert);
                        }
                        _ => continue,
                    }
                } else if hor == HorizontalDirection::West {
                    match tile {
                        TileType::NorthEast | TileType::SouthEast | TileType::Horizontal => {
                            return ((x, y), hor, vert);
                        }
                        _ => continue,
                    }
                } else if vert == VerticalDirection::North {
                    match tile {
                        TileType::SouthEast | TileType::SouthWest | TileType::Vertical => {
                            return ((x, y), hor, vert);
                        }
                        _ => continue,
                    }
                } else if vert == VerticalDirection::South {
                    match tile {
                        TileType::NorthEast | TileType::NorthWest | TileType::Vertical => {
                            return ((x, y), hor, vert);
                        }
                        _ => continue,
                    }
                }
            }
        }
        panic!()
    }
    pub fn run(&self) -> Vec<(usize, usize)> {
        let (mut tile_loc, mut horizontal, mut vertical) = self.initial_flow();

        let mut visit = vec![];

        loop {
            visit.push(tile_loc);
            let tile = self.0.get(tile_loc).unwrap();
            match tile {
                TileType::Horizontal => {
                    if horizontal == HorizontalDirection::East {
                        tile_loc = (tile_loc.0 + 1, tile_loc.1)
                    } else if horizontal == HorizontalDirection::West {
                        tile_loc = (tile_loc.0 - 1, tile_loc.1)
                    }
                }
                TileType::NorthEast => {
                    if horizontal == HorizontalDirection::West {
                        tile_loc = (tile_loc.0, tile_loc.1 - 1);
                        horizontal = HorizontalDirection::Undef;
                        vertical = VerticalDirection::North;
                    } else if vertical == VerticalDirection::South {
                        tile_loc = (tile_loc.0 + 1, tile_loc.1);
                        horizontal = HorizontalDirection::East;
                        vertical = VerticalDirection::Undef;
                    }
                }
                TileType::NorthWest => {
                    if horizontal == HorizontalDirection::East {
                        tile_loc = (tile_loc.0, tile_loc.1 - 1);
                        horizontal = HorizontalDirection::Undef;
                        vertical = VerticalDirection::North;
                    } else if vertical == VerticalDirection::South {
                        tile_loc = (tile_loc.0 - 1, tile_loc.1);
                        horizontal = HorizontalDirection::West;
                        vertical = VerticalDirection::Undef;
                    }
                }
                TileType::SouthEast => {
                    if horizontal == HorizontalDirection::West {
                        tile_loc = (tile_loc.0, tile_loc.1 + 1);
                        horizontal = HorizontalDirection::Undef;
                        vertical = VerticalDirection::South;
                    } else if vertical == VerticalDirection::North {
                        tile_loc = (tile_loc.0 + 1, tile_loc.1);
                        horizontal = HorizontalDirection::East;
                        vertical = VerticalDirection::Undef;
                    }
                }
                TileType::SouthWest => {
                    if horizontal == HorizontalDirection::East {
                        tile_loc = (tile_loc.0, tile_loc.1 + 1);
                        horizontal = HorizontalDirection::Undef;
                        vertical = VerticalDirection::South;
                    } else if vertical == VerticalDirection::North {
                        tile_loc = (tile_loc.0 - 1, tile_loc.1);
                        horizontal = HorizontalDirection::West;
                        vertical = VerticalDirection::Undef;
                    }
                }
                TileType::Vertical => {
                    if vertical == VerticalDirection::North {
                        tile_loc = (tile_loc.0, tile_loc.1 - 1);
                    } else if vertical == VerticalDirection::South {
                        tile_loc = (tile_loc.0, tile_loc.1 + 1);
                    }
                }
                TileType::Starting => {
                    return visit;
                }
                _ => (),
            }
        }
    }
}
