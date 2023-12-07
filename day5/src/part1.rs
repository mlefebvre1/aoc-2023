use std::{num::ParseIntError, str::FromStr};

use anyhow::{anyhow, Error};

use crate::common::{Map, Maps};

#[derive(Debug)]
struct Seeds(Vec<usize>);

impl FromStr for Seeds {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("seeds: ")
            .ok_or(anyhow!("fail to strip 'seeds: ' while parsing seeds"))?;
        let seeds: Result<Vec<usize>, ParseIntError> =
            s.split_whitespace().map(|seed| seed.parse()).collect();
        Ok(Self(seeds?))
    }
}
impl Seeds {
    pub fn inner(self) -> Vec<usize> {
        self.0
    }
}

#[derive(Debug)]
struct Almanac {
    pub seeds: Vec<usize>,
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
            .iter()
            .map(|&seed| {
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
                Self::transform(tseed, &self.maps.humidity_to_location)
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
    let locations = almanac.transform_seeds();
    let ans = locations.iter().min().unwrap();
    ans.to_string()
}
