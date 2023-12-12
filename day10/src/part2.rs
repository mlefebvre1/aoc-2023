use std::str::FromStr;

use util::grid::Grid;

use crate::common::Diagram;

pub fn run(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let diagram = Diagram::from_str(&data).unwrap();
    let shape = diagram.grid_shape();

    let visit = diagram.run();
    let mut tile_visited = Grid::new(vec![vec![false; shape.0]; shape.1]);
    visit.into_iter().for_each(|(x, y)| {
        tile_visited.set((x, y), true);
    });
    for row in tile_visited.rows() {
        for col in row {
            if *col {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    "".to_string()
}
