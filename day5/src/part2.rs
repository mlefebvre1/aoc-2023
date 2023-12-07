use itertools::Itertools;
use std::str::FromStr;

use anyhow::{anyhow, Error};
use rayon::prelude::*;

use crate::common::{Map, Maps};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct SeedRange {
    start: usize,
    len: usize,
}

#[derive(Debug)]
struct Seeds(Vec<SeedRange>);

impl FromStr for Seeds {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("seeds: ")
            .ok_or(anyhow!("fail to strip 'seeds: ' while parsing seeds"))?;
        let seeds: Vec<SeedRange> = s
            .split_whitespace()
            .chunks(2)
            .into_iter()
            .map(|mut chunk| {
                let start = chunk.next().unwrap().parse().unwrap();
                let len = chunk.next().unwrap().parse().unwrap();
                SeedRange { start, len }
            })
            .collect();

        Ok(Self(seeds))
    }
}
impl Seeds {
    pub fn inner(self) -> Vec<SeedRange> {
        self.0
    }
}

#[derive(Debug)]
struct Almanac {
    pub seeds: Vec<SeedRange>,
    pub maps: Maps,
}

impl FromStr for Almanac {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let seeds_raw = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<String>();
        let seeds = Seeds::from_str(&seeds_raw)?;
        let rest = lines.collect::<Vec<_>>().join("\n");
        let maps = Maps::from_str(&rest)?;
        Ok(Self {
            seeds: seeds.inner(),
            maps,
        })
    }
}

impl Almanac {
    pub fn transform_seeds(&self) -> Vec<usize> {
        self.seeds
            .par_iter()
            .flat_map(|seed_range| {
                let mut v = vec![];
                for seed in seed_range.start..seed_range.start + seed_range.len {
                    // seed-to-soil
                    let tseed = Self::transform(seed, &self.maps.seed_to_soil);
                    // soil-to-fertilizer
                    let tseed = Self::transform(tseed, &self.maps.soil_to_fertilizer);
                    // fertilizer_to_water
                    let tseed = Self::transform(tseed, &self.maps.fertilizer_to_water);
                    // water_to_light
                    let tseed = Self::transform(tseed, &self.maps.water_to_light);
                    // light_to_temperature
                    let tseed = Self::transform(tseed, &self.maps.light_to_temperature);
                    // temperature_to_humidity
                    let tseed = Self::transform(tseed, &self.maps.temperature_to_humidity);
                    // humidity_to_location
                    v.push(Self::transform(tseed, &self.maps.humidity_to_location));
                }
                v
            })
            .collect()
    }
    fn transform(seed: usize, maps: &[Map]) -> usize {
        maps.iter()
            .find_map(|m| {
                m.src_range().contains(&seed).then(|| {
                    let i = seed - m.src;
                    m.dest + i
                })
            })
            .unwrap_or(seed)
    }
}

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let almanac = Almanac::from_str(&data).unwrap();
    let now = std::time::Instant::now();
    let locations = almanac.transform_seeds();
    println!("took={}s", now.elapsed().as_secs());
    let ans = locations.iter().min().unwrap();
    ans.to_string()
}
