use anyhow::Error;
use std::{ops::Range, str::FromStr};

#[derive(Debug)]
pub struct Map {
    pub dest: usize,
    pub src: usize,
    pub len: usize,
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
    pub fn from_range(start: usize, end: usize, dst_start: usize) -> Self {
        Self {
            src: start,
            dest: dst_start,
            len: end - start,
        }
    }
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

#[derive(Debug)]
pub struct Maps {
    pub seed_to_soil: Vec<Map>,
    pub soil_to_fertilizer: Vec<Map>,
    pub fertilizer_to_water: Vec<Map>,
    pub water_to_light: Vec<Map>,
    pub light_to_temperature: Vec<Map>,
    pub temperature_to_humidity: Vec<Map>,
    pub humidity_to_location: Vec<Map>,
}

impl FromStr for Maps {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
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
