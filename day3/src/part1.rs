use crate::common::{adjacent_numbers, extract_numbers_matrix, make_symbol_matrix};

use super::common::{Matrix, Symbol};

pub fn run() -> String {
    let data = std::fs::read_to_string("day3/data/day3.txt").unwrap();
    // display_separators(&data);
    let numbers = extract_numbers_matrix(&data);
    let matrix = make_symbol_matrix(&data, &numbers);
    let ans: u32 = row_adjacents(&matrix).iter().flatten().sum();
    ans.to_string()
}

fn row_adjacents(matrix: &Matrix<Symbol>) -> Matrix<u32> {
    let mut row_adjacents = vec![];
    for (y, row) in matrix.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if let Symbol::Other(_) = s {
                row_adjacents.push(adjacent_numbers((x, y), matrix));
            }
        }
    }
    row_adjacents
}
