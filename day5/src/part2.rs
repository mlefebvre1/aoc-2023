use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::str::FromStr;

use crate::common::{Map, Maps};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct SeedRange {
    start: usize,
    len: usize,
}
impl SeedRange {
    pub fn from_range(start: usize, end: usize) -> Self {
        Self {
            start,
            len: end - start,
        }
    }
    pub fn remap(start: usize, end: usize, map: &Map) -> Self {
        /*
                S0..S1
        Seed:   |....|
                M0..M1
        Map:    |....|

        Seed:         |..........|
            (1)|....|              |...|    -> Nothing to do
            (2)     |.......|          -> remap(S0..M1) + M1..S1        (M0..S0)
            (3)             |.......|  -> S0..M0 + remap(M0..S1)
            (4)         |.......|      -> S0..M0 + remap(M0..M1) + M1..S1
            (5)     |................| -> remap(S0..S1)
        */
        let skip_len = start - map.src;
        let d0 = map.dest + skip_len;
        Self {
            start: d0,
            len: map.len - skip_len,
        }
        // seed1..seed2
        // src1..src2
        // dst1..dst2
    }
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
    pub fn transform_seeds(&self) -> Vec<Vec<SeedRange>> {
        let seed_ranges: Vec<Vec<SeedRange>> = self
            .seeds
            .iter()
            .map(|seed| {
                // seed-to-soil
                let tseed = Self::transform(vec![seed.clone()], &self.maps.seed_to_soil);
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
            .collect();
        seed_ranges
    }
    fn transform(mut seed_ranges: Vec<SeedRange>, maps: &[Map]) -> Vec<SeedRange> {
        for m in maps.iter() {
            for seed_range in seed_ranges.iter() {
                if m.src_range().contains(&seed_range.start) {
                    let diff = seed_range.start - m.src;
                    let len = diff - m.len;
                    let rest = seed_range.len - diff;
                    if rest > 0 {
                        // let upper_range = SeedRange {
                        //     start:m.src+len
                        // }
                    }
                    let new_range = SeedRange {
                        start: diff + m.dest,
                        len,
                    };
                }
            }
        }
        seed_ranges
        // maps.iter()
        //     .find_map(|m| {
        //         m.src_range().contains(&seed).then(|| {
        //             let i = seed - m.src;
        //             m.dest + i
        //         })
        //     })
        //     .unwrap_or(seed)
    }
    fn split_range(seed_range: SeedRange, map: Map) -> Vec<SeedRange> {
        /*
                S0..S1
        Seed:   |....|

                M0..M1
        Map:    |....|

        Seed:         |..........|
            (1)|....|              |...|    -> Nothing to do
            (2)     |.......|          -> remap(S0..M1) + M1..S1
            (3)             |.......|  -> S0..M0 + remap(M0..S1)
            (4)         |.......|      -> S0..M0 + remap(M0..M1) + M1..S1
            (5)     |................| -> remap(S0..S1)
        */
        let (s0, s1) = (seed_range.start, seed_range.start + seed_range.len);
        let (m0, m1) = (map.src, map.src + map.len);

        // case (1)
        if s0 > m1 || m0 > s1 {
            return vec![seed_range];
        }

        // case (2)
        if s0 > m0 && s1 > m1 {
            let skip_len = s0 - m0;
            let d0 = map.dest + skip_len;
            let d1 = d0 + map.len - skip_len;
            return vec![SeedRange::from_range(d0, d1), SeedRange::from_range(m1, s1)];
        }

        // case (3)
        if s0 < m0 && s1 < m1 {
            let skip_len = s1 - m0;
            let d0 = map.dest;
            let d1 = d0 + skip_len;
            return vec![SeedRange::from_range(s0, m0), SeedRange::from_range(d0, d1)];
        }

        // case (4)
        if s0 < m0 && s1 > m1 {
            let d0 = map.dest;
            let d1 = d0 + map.len;
            return vec![
                SeedRange::from_range(s0, m0),
                SeedRange::from_range(d0, d1),
                SeedRange::from_range(m1, s1),
            ];
        }

        // case (5)
        //              |....|
        // (5)     |................| -> remap(S0..S1)
        if s0 > m0 && s1 < m1 {
            let skip = s0 - m0;
            let d0 = map.dest + skip;
            let d1 = d0 + (s1 - s0);
            return vec![SeedRange::from_range(d0, d1)];
        }

        vec![]
    }
}

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let almanac = Almanac::from_str(&data).unwrap();
    let locations = almanac.transform_seeds();
    let ans = "";
    // let ans = locations.iter().min().unwrap();
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_range_case1() {
        /*
                S0..S1
        Seed:   |....|
                M0..M1
        Map:    |....|

        Seed:         |..........|
            (1)|....|              |...|    -> Nothing to do
            (2)     |.......|          -> remap(S0..M1) + M1..S1
            (3)             |.......|  -> S0..M0 + remap(M0..S1)
            (4)         |.......|      -> S0..M0 + remap(M0..M1) + M1..S1
            (5)     |................| -> remap(S0..S1)
        */
        let s0 = 5;
        let s1 = 10;
        let seed = SeedRange::from_range(s0, s1);

        let m0 = 0;
        let m1 = 3;
        let map = Map::from_range(m0, m1, 0);
        let v = Almanac::split_range(seed.clone(), map);
        assert_eq!(v, vec![SeedRange::from_range(5, 10)]);

        let m0 = 11;
        let m1 = 15;
        let map = Map::from_range(m0, m1, 0);
        let v = Almanac::split_range(seed, map);
        assert_eq!(v, vec![SeedRange::from_range(5, 10)]);
    }

    #[test]
    fn test_split_range_case2() {
        /*

        Seed:         |..........|
            (2)     |.......|          -> remap(S0..M1) + M1..S1
        */
        let s0 = 5;
        let s1 = 10;
        let seed = SeedRange::from_range(s0, s1);

        let m0 = 0;
        let m1 = 8;
        let map = Map::from_range(m0, m1, 20);
        let v = Almanac::split_range(seed.clone(), map);
        assert_eq!(
            v,
            vec![SeedRange::from_range(25, 28), SeedRange::from_range(8, 10)]
        )
    }

    #[test]
    fn test_split_range_case3() {
        /*

        Seed:         |..........|
            (3)             |.......|  -> S0..M0 + remap(M0..S1)
        */
        let s0 = 5;
        let s1 = 10;
        let seed = SeedRange::from_range(s0, s1);

        let m0 = 6;
        let m1 = 14;
        let map = Map::from_range(m0, m1, 20);
        let v = Almanac::split_range(seed.clone(), map);
        assert_eq!(
            v,
            vec![SeedRange::from_range(5, 6), SeedRange::from_range(20, 24),]
        )
    }

    #[test]
    fn test_split_range_case4() {
        /*

        Seed:         |..........|
            (4)         |.......|      -> S0..M0 + remap(M0..M1) + M1..S1
        */
        let s0 = 5;
        let s1 = 15;
        let seed = SeedRange::from_range(s0, s1);

        let m0 = 7;
        let m1 = 12;
        let map = Map::from_range(m0, m1, 20);
        let v = Almanac::split_range(seed.clone(), map);
        assert_eq!(
            v,
            vec![
                SeedRange::from_range(5, 7),
                SeedRange::from_range(20, 25),
                SeedRange::from_range(12, 15),
            ]
        )
    }

    #[test]
    fn test_split_range_case5() {
        /*

        Seed:         |..........|
            (5)     |................| -> remap(S0..S1)
        */
        let s0 = 5;
        let s1 = 15;
        let seed = SeedRange::from_range(s0, s1);

        let m0 = 0;
        let m1 = 20;
        let map = Map::from_range(m0, m1, 20);
        let v = Almanac::split_range(seed.clone(), map);
        assert_eq!(v, vec![SeedRange::from_range(25, 35)])
    }
}
