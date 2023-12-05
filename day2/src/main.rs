use anyhow::{anyhow, Error};
use std::str::FromStr;

#[derive(Default, Debug)]
struct Pick {
    red: usize,
    green: usize,
    blue: usize,
}

impl Pick {
    fn add(&mut self, nb: usize, color: &str) {
        match color {
            "red" => self.red += nb,
            "green" => self.green += nb,
            "blue" => self.blue += nb,
            _ => (),
        }
    }
    pub fn is_possible(&self) -> bool {
        // only 12 red cubes, 13 green cubes, and 14 blue cubes are possible
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for Pick {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pick = Pick::default();

        // X blue, Y red, Z green
        for item in s.split(',') {
            let item = item
                .strip_prefix(' ')
                .ok_or(anyhow!("failed to strip whitespace"))?;
            let mut item = item.split_whitespace();
            let nb: usize = item.next().ok_or(anyhow!("failed to parse nb"))?.parse()?;
            let color = item.next().ok_or(anyhow!("failed to parse color"))?;
            pick.add(nb, color);
        }

        Ok(pick)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    picks: Vec<Pick>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.picks.iter().all(|pick| pick.is_possible())
    }

    fn min_possible_set(&self) -> Pick {
        let red = self.picks.iter().max_by_key(|k| k.red).unwrap().red;
        let green = self.picks.iter().max_by_key(|k| k.green).unwrap().green;
        let blue = self.picks.iter().max_by_key(|k| k.blue).unwrap().blue;
        Pick { red, green, blue }
    }
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Format is as follow
        // Game N: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let s: Vec<&str> = s.split(':').collect();

        // First process game id ==> Game N
        let game_str = s.first().ok_or(anyhow!("failed to get game data"))?;
        let game_id: usize = game_str
            .strip_prefix("Game ")
            .map(|c| c.parse().unwrap())
            .ok_or(anyhow!("failed to parse game id"))?;

        // Process the cubes ==> 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let cubes_str = s.get(1).ok_or(anyhow!("failed to get cubes data"))?;
        let picks = cubes_str
            .split(';')
            .map(|c| Pick::from_str(c).unwrap())
            .collect();

        Ok(Self { id: game_id, picks })
    }
}

fn main() {
    let cli = util::Cli::get();
    println!("part1={}", part1(&cli.file));
    println!("part2={}", part2(&cli.file));
}

fn part1(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let ans: usize = data
        .lines()
        .filter_map(|line| {
            let game = Game::from_str(line).unwrap();

            if game.is_possible() {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();

    ans.to_string()
}

fn part2(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let ans: usize = data
        .lines()
        .map(|line| {
            let game = Game::from_str(line).unwrap();
            let set = game.min_possible_set();
            set.power()
        })
        .sum();

    ans.to_string()
}
