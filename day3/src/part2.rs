use util::grid::Grid;

use crate::common::{adjacent_numbers, extract_numbers_matrix, make_symbol_grid};

use super::common::Symbol;

pub fn run() -> String {
    let data = std::fs::read_to_string("day3/data/day3.txt").unwrap();
    // display_separators(&data);
    let numbers = extract_numbers_matrix(&data);
    let grid = make_symbol_grid(&data, &numbers);
    let ans: u32 = gear_ratios(&grid).iter().sum();
    ans.to_string()
}

fn gear_ratios(grid: &Grid<Symbol>) -> Vec<u32> {
    let mut gear_ratios = vec![];
    for (y, row) in grid.rows().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if let Symbol::Other(c) = s {
                if *c == '*' {
                    let gears = adjacent_numbers((x, y), grid);
                    if gears.len() == 2 {
                        gear_ratios.push(gears.iter().copied().product());
                    }
                }
            }
        }
    }
    gear_ratios
}
