use std::str::FromStr;

use crate::common::Diagram;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let coords = Diagram::from_str(&data).unwrap().run();
    let area = calculate_area(&coords);
    let boundary = coords.len();
    let ans = area - (boundary / 2) + 1; // pick theorem to calculate interior points from the are and the boundary points
    ans.to_string()
}

fn calculate_area(coords: &[(usize, usize)]) -> usize {
    let mut area = 0;
    for i in 0..coords.len() {
        let next_i = (i + 1) % coords.len();
        area +=
            (coords[i].0 * coords[next_i].1) as isize - (coords[i].1 * coords[next_i].0) as isize;
    }
    (area / 2).unsigned_abs()
}
