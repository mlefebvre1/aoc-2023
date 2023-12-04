use crate::common::{adjacent_numbers, extract_numbers_matrix, make_symbol_matrix};

use super::common::{Matrix, Symbol};

pub fn run() -> String {
    let data = std::fs::read_to_string("day3/data/day3.txt").unwrap();
    // display_separators(&data);
    let numbers = extract_numbers_matrix(&data);
    let matrix = make_symbol_matrix(&data, &numbers);
    let ans: u32 = gear_ratios(&matrix).iter().sum();
    ans.to_string()
}

fn gear_ratios(matrix: &Matrix<Symbol>) -> Vec<u32> {
    let mut gear_ratios = vec![];
    for (y, row) in matrix.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if let Symbol::Other(c) = s {
                if *c == '*' {
                    let gears = adjacent_numbers((x, y), matrix);
                    if gears.len() == 2 {
                        gear_ratios.push(gears.iter().copied().product());
                    }
                }
            }
        }
    }
    gear_ratios
}
