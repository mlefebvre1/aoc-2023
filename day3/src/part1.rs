use util::grid::Grid;

use crate::common::{adjacent_numbers, extract_numbers_matrix, make_symbol_grid};

use super::common::Symbol;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    // display_separators(&data);
    let numbers = extract_numbers_matrix(&data);
    let matrix = make_symbol_grid(&data, &numbers);
    let ans: u32 = row_adjacents(&matrix).iter().flatten().sum();
    ans.to_string()
}

fn row_adjacents(grid: &Grid<Symbol>) -> Vec<Vec<u32>> {
    let mut row_adjacents = vec![];
    for (y, row) in grid.rows().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if let Symbol::Other(_) = s {
                row_adjacents.push(adjacent_numbers((x, y), grid));
            }
        }
    }
    row_adjacents
}
