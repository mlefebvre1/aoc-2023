use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    str::FromStr,
};

use anyhow::{anyhow, Error};
use num::Integer;
use util::grid::Grid;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Garden,
    Rock,
    Start,
}
impl TryFrom<char> for Tile {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Garden),
            '#' => Ok(Self::Rock),
            'S' => Ok(Self::Start),
            _ => Err(anyhow!("can't convert character to Tile type")),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Garden => '.',
            Self::Rock => '#',
            Self::Start => 'S',
        };
        write!(f, "{c}")
    }
}

pub struct Puzzle {
    inner: Grid<Tile>,
}
impl Puzzle {
    pub fn fill(&self, max_steps: usize) -> usize {
        let mut dist_grid = Grid::new(vec![
            vec![0usize; self.inner.nb_columns()];
            self.inner.nb_rows()
        ]);
        let mut visited = HashSet::new();

        let mut heapq = BinaryHeap::new();
        heapq.push(Reverse((0, self.get_start_pos())));
        while let Some(Reverse((dist, (x, y)))) = heapq.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            dist_grid.set((x, y), dist);

            let (x, y) = (x as isize, y as isize);
            for (xi, yi) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if let (Some(xn), Some(yn)) = (x.checked_add(xi), y.checked_add(yi)) {
                    if let Some(Tile::Garden) = self.inner.get((xn as usize, yn as usize)) {
                        heapq.push(Reverse((dist + 1, (xn as usize, yn as usize))));
                    }
                }
            }
        }

        dist_grid
            .iter()
            .filter(|&c| *c > 0 && *c <= max_steps && c.is_even())
            .count()
            + 1
    }

    fn get_start_pos(&self) -> (usize, usize) {
        self.inner.find(&Tile::Start).unwrap()
    }
}
impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<Vec<Tile>>, Error> = s
            .lines()
            .map(|line| line.chars().map(Tile::try_from).collect())
            .collect();
        Ok(Self {
            inner: Grid::new(v?),
        })
    }
}
impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
