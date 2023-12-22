use std::{cmp::Reverse, collections::HashSet, str::FromStr};

use anyhow::{anyhow, Error};
use std::collections::BinaryHeap;

use util::grid::Grid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

pub struct Puzzle(Grid<u32>);
impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m: Result<Vec<Vec<u32>>, Error> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .ok_or(anyhow!("fail to convert char to digit"))
                    })
                    .collect()
            })
            .collect();

        Ok(Self(Grid::new(m?)))
    }
}
impl Puzzle {
    pub fn run(&self, min_straight_move: usize, max_straight_move: usize) -> usize {
        let shape = self.0.shape();
        let src = (0, 0);
        let dst = (shape.0 - 1, shape.1 - 1);
        let mut heapq = BinaryHeap::new();
        let mut visited = HashSet::new();
        heapq.push(Reverse((0, src, Direction::Unknown, 0)));

        while let Some(Reverse((dist, node, dir, steps))) = heapq.pop() {
            if node == dst && steps >= min_straight_move {
                return dist;
            }

            if visited.contains(&(node, dir, steps)) {
                continue;
            }
            visited.insert((node, dir, steps));

            let mut candidates = vec![];
            if let Direction::Unknown = dir {
                // Occurs only on first node
                candidates.push(((1, 0), Direction::Right, 1));
                candidates.push(((0, 1), Direction::Down, 1));
            } else {
                // can only turn right/left if at least 'min_straight_move' steps were taken in the same direction
                if steps >= min_straight_move {
                    // attempt to turn right
                    if let Some((right_node, new_dir)) = Self::turn_right(node, dir) {
                        candidates.push((right_node, new_dir, 1))
                    }
                    // attempt to turn left
                    if let Some((left_node, new_dir)) = Self::turn_left(node, dir) {
                        candidates.push((left_node, new_dir, 1));
                    }
                }
                // attempt to go straigt
                if steps < max_straight_move {
                    if let Some(straight_node) = Self::go_straight(node, dir) {
                        candidates.push((straight_node, dir, steps + 1));
                    }
                }
            }

            for (new_node, new_dir, new_steps) in candidates {
                if let Some(&new_node_dist) = self.0.get(new_node) {
                    let new_dist = dist + new_node_dist as usize;
                    heapq.push(Reverse((new_dist, new_node, new_dir, new_steps)))
                }
            }
        }

        0
    }
    fn turn_right(current: (usize, usize), dir: Direction) -> Option<((usize, usize), Direction)> {
        match dir {
            Direction::Up => current
                .0
                .checked_add(1)
                .map(|x| ((x, current.1), Direction::Right)),
            Direction::Down => current
                .0
                .checked_sub(1)
                .map(|x| ((x, current.1), Direction::Left)),
            Direction::Left => current
                .1
                .checked_sub(1)
                .map(|y| ((current.0, y), Direction::Up)),
            Direction::Right => current
                .1
                .checked_add(1)
                .map(|y| ((current.0, y), Direction::Down)),
            _ => None,
        }
    }
    fn turn_left(current: (usize, usize), dir: Direction) -> Option<((usize, usize), Direction)> {
        match dir {
            Direction::Up => current
                .0
                .checked_sub(1)
                .map(|x| ((x, current.1), Direction::Left)),
            Direction::Down => current
                .0
                .checked_add(1)
                .map(|x| ((x, current.1), Direction::Right)),
            Direction::Left => current
                .1
                .checked_add(1)
                .map(|y| ((current.0, y), Direction::Down)),
            Direction::Right => current
                .1
                .checked_sub(1)
                .map(|y| ((current.0, y), Direction::Up)),
            _ => None,
        }
    }

    fn go_straight(current: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::Up => current.1.checked_sub(1).map(|y| (current.0, y)),
            Direction::Down => current.1.checked_add(1).map(|y| (current.0, y)),
            Direction::Left => current.0.checked_sub(1).map(|x| (x, current.1)),
            Direction::Right => current.0.checked_add(1).map(|x| (x, current.1)),
            _ => None,
        }
    }
}
