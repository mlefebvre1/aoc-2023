use std::{collections::HashMap, str::FromStr};

use crate::common::{Lens, Operation};

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();

    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

    data.split(',').for_each(|step| {
        let op = Operation::from_str(step).unwrap();

        match op {
            Operation::Add { lens, box_id } => {
                if let Some(b) = boxes.get_mut(&box_id) {
                    if let Some(i) = b.iter().position(|innerlens| lens.label == innerlens.label) {
                        b[i] = lens;
                    } else {
                        b.push(lens);
                    }
                } else {
                    boxes.insert(box_id, vec![lens]);
                }
            }
            Operation::Remove { box_id, label } => {
                if let Some(b) = boxes.get_mut(&box_id) {
                    if let Some(i) = b.iter().position(|lens| lens.label == label) {
                        b.remove(i);
                    }
                }
            }
        }
    });

    let ans: usize = boxes
        .iter()
        .map(|(box_id, lens)| {
            let box_value = box_id + 1;
            lens.iter()
                .enumerate()
                .map(|(i, lens)| box_value * (i + 1) * lens.focal_len)
                .sum::<usize>()
        })
        .sum();

    ans.to_string()
}
