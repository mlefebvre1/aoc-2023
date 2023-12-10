use std::{num::ParseIntError, str::FromStr};

use anyhow::Error;

pub struct Report(Vec<History>);
impl FromStr for Report {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<History>, Error> = s.lines().map(History::from_str).collect();
        Ok(Self(v?))
    }
}
impl Report {
    pub fn run_extrapolate_first(&self) -> isize {
        self.0
            .iter()
            .map(|h| {
                let (e, _) = h.run();
                e
            })
            .sum()
    }
    pub fn run_extrapolate_last(&self) -> isize {
        self.0
            .iter()
            .map(|h| {
                let (_, e) = h.run();
                e
            })
            .sum()
    }
}

struct History(Vec<isize>);
impl FromStr for History {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<isize>, ParseIntError> =
            s.split_whitespace().map(|l| l.parse()).collect();
        Ok(Self(v?))
    }
}

impl History {
    fn run(&self) -> (isize, isize) {
        let mut firsts = vec![];
        let mut lasts = vec![];
        let mut vin = self.0.clone();
        loop {
            let mut vout = vec![0isize; vin.len() - 1];

            for i in 0..vout.len() {
                vout[i] = vin[i + 1] - vin[i];
            }
            firsts.push(vin.first().copied().unwrap());
            lasts.push(vin.last().copied().unwrap());
            if vout.iter().all(|&n| n == 0) {
                let mut first_extrapolated = vec![0isize; firsts.len() + 1];
                (0..firsts.len()).rev().for_each(|i| {
                    first_extrapolated[i] = firsts[i] - first_extrapolated[i + 1];
                });
                return (
                    first_extrapolated.first().copied().unwrap(),
                    lasts.into_iter().sum(),
                );
            }

            vin = vout;
        }
    }
}
