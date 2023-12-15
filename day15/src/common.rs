use std::str::FromStr;

use anyhow::Error;

pub struct Step(usize);
impl Step {
    pub fn inner(&self) -> usize {
        self.0
    }
}

impl FromStr for Step {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_value = 0;
        for c in s.chars() {
            current_value += c as usize;
            current_value *= 17;
            current_value %= 256;
        }
        Ok(Self(current_value))
    }
}

type Label = String;

#[derive(Debug)]
pub enum Operation {
    Remove { box_id: usize, label: Label },
    Add { box_id: usize, lens: Lens },
}
impl FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            let label = s.split('-').next().unwrap();
            let box_id = Step::from_str(label)?.inner();
            Ok(Self::Remove {
                box_id,
                label: label.to_string(),
            })
        } else {
            let mut sp = s.split('=');
            let label = sp.next().unwrap();
            let box_id = Step::from_str(label)?.inner();
            let focal_len = sp.next().unwrap().parse()?;
            let lens = Lens {
                label: label.to_string(),
                focal_len,
            };
            Ok(Self::Add { lens, box_id })
        }
    }
}

#[derive(Debug)]
pub struct Lens {
    pub label: Label,
    pub focal_len: usize,
}
