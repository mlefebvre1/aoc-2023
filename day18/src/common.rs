use std::str::FromStr;

use anyhow::{anyhow, Error};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up(isize),
    Down(isize),
    Right(isize),
    Left(isize),
}
impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s.split_whitespace();
        let d = ss.next().ok_or(anyhow!("failed to get direction"))?;
        let meters = ss
            .next()
            .ok_or(anyhow!("failed to get the number of meter"))?
            .parse()?;
        match d {
            "U" => Ok(Self::Up(meters)),
            "D" => Ok(Self::Down(meters)),
            "R" => Ok(Self::Right(meters)),
            "L" => Ok(Self::Left(meters)),
            _ => Err(anyhow!("failed to decode direction")),
        }
    }
}
impl TryFrom<Color> for Direction {
    type Error = Error;
    fn try_from(value: Color) -> Result<Self, Self::Error> {
        let s = value.0.replace('#', "");
        let nb_digits = s.chars().count();
        let n = isize::from_str_radix(&s.as_str()[..nb_digits - 1], 16)?;
        let d = s
            .chars()
            .last()
            .ok_or(anyhow!("failed to get last digit"))?;
        match d {
            '0' => Ok(Self::Right(n)),
            '1' => Ok(Self::Down(n)),
            '2' => Ok(Self::Left(n)),
            '3' => Ok(Self::Up(n)),
            _ => Err(anyhow!("no direction")),
        }
    }
}

#[derive(Debug, Clone)]
struct Color(String);
impl FromStr for Color {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.replace(')', "")))
    }
}
struct Edge {
    dir: Direction,
    color: Color,
}

pub struct Puzzle(Vec<Edge>);
impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges: Vec<Edge> = s
            .lines()
            .map(|line| {
                let mut ss = line.split('(');
                let dir = Direction::from_str(ss.next().unwrap()).unwrap();
                let color = Color::from_str(ss.next().unwrap()).unwrap();
                Edge { dir, color }
            })
            .collect();
        Ok(Self(edges))
    }
}

impl Puzzle {
    pub fn run_part1(self) -> isize {
        let (area, boundary_len) =
            Self::calculate_area_and_boundary_len(self.0.into_iter().map(|edge| edge.dir));
        let interiors = Self::calculate_interior(area, boundary_len);
        boundary_len + interiors
    }
    pub fn run_part2(self) -> isize {
        let (area, boundary_len) = Self::calculate_area_and_boundary_len(
            self.0
                .into_iter()
                .map(|edge| Direction::try_from(edge.color.clone()).unwrap()),
        );
        let interiors = Self::calculate_interior(area, boundary_len);
        boundary_len + interiors
    }

    fn calculate_area_and_boundary_len(
        directions: impl Iterator<Item = Direction>,
    ) -> (isize, isize) {
        // Green theorem for the area
        let mut ypos = 0;
        let mut boundary_len = 0;
        let mut area = 0;
        for dir in directions {
            match dir {
                Direction::Up(n) => {
                    ypos += n;
                    boundary_len += n;
                }
                Direction::Down(n) => {
                    ypos -= n;
                    boundary_len += n;
                }
                Direction::Left(n) => {
                    area -= ypos * n;
                    boundary_len += n;
                }
                Direction::Right(n) => {
                    area += ypos * n;
                    boundary_len += n;
                }
            }
        }
        (area, boundary_len)
    }

    fn calculate_interior(area: isize, boundary_len: isize) -> isize {
        // The infamous Pick theorem
        area - (boundary_len / 2) + 1
    }
}
