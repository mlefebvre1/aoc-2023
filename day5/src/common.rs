use anyhow::Error;
use std::{
    num::ParseIntError,
    ops::{Range, RangeBounds},
    str::FromStr,
};

#[derive(Debug)]
struct Map {
    dest: usize,
    src: usize,
    len: usize,
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut toks = s.split_whitespace();
        let dest = toks.next().unwrap().parse()?;
        let src = toks.next().unwrap().parse()?;
        let len = toks.next().unwrap().parse()?;
        Ok(Self { dest, src, len })
    }
}

impl Map {
    pub fn src_range(&self) -> Range<usize> {
        self.src..self.src + self.len
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

macro_rules! extract_map {
    ($lines:ident) => {
        $lines
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(Map::from_str)
            .collect()
    };
}

impl FromStr for Almanac {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        //sseds
        let seeds_raw = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let seeds: Result<Vec<usize>, ParseIntError> = seeds_raw
            .first()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|ns| ns.parse())
            .collect();

        // seed-to-soil map
        let seed_to_soil: Result<Vec<Map>, Error> = extract_map!(lines);
        // soil-to-fertilizer map
        let soil_to_fertilizer: Result<Vec<Map>, Error> = extract_map!(lines);
        // fertilizer-to-water map
        let fertilizer_to_water: Result<Vec<Map>, Error> = extract_map!(lines);
        // water-to-light map
        let water_to_light: Result<Vec<Map>, Error> = extract_map!(lines);

        //light-to-temperature map
        let light_to_temperature: Result<Vec<Map>, Error> = extract_map!(lines);

        // temperature-to-humidity map
        let temperature_to_humidity: Result<Vec<Map>, Error> = extract_map!(lines);

        // humidity-to-location map
        let humidity_to_location: Result<Vec<Map>, Error> = extract_map!(lines);

        Ok(Self {
            seeds: seeds?,
            seed_to_soil: seed_to_soil?,
            soil_to_fertilizer: soil_to_fertilizer?,
            fertilizer_to_water: fertilizer_to_water?,
            water_to_light: water_to_light?,
            light_to_temperature: light_to_temperature?,
            temperature_to_humidity: temperature_to_humidity?,
            humidity_to_location: humidity_to_location?,
        })
    }
}

impl Almanac {
    pub fn transform_seeds(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|&seed| {
                // seed-to-soil
                let tseed = Self::transform(seed, &self.seed_to_soil);
                // soil-to-fertilizer
                let tseed = Self::transform(tseed, &self.soil_to_fertilizer);
                // fertilizer_to_water
                let tseed = Self::transform(tseed, &self.fertilizer_to_water);
                // water_to_light
                let tseed = Self::transform(tseed, &self.water_to_light);
                // light_to_temperature
                let tseed = Self::transform(tseed, &self.light_to_temperature);
                // temperature_to_humidity
                let tseed = Self::transform(tseed, &self.temperature_to_humidity);
                // humidity_to_location
                Self::transform(tseed, &self.humidity_to_location)
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
