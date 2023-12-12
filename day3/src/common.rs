use util::grid::Grid;

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Number(u32),
    Period,
    Other(char),
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Period,
            c if c.is_numeric() => Self::Number(c.to_digit(10).unwrap()),
            _ => Self::Other(value),
        }
    }
}

pub type Number = String;

pub fn make_symbol_grid(data: &str, numbers: &[Vec<String>]) -> Grid<Symbol> {
    let mut grid = Grid::new(
        data.lines()
            .map(|line| line.chars().map(Symbol::from).collect())
            .collect(),
    );

    for (y, _) in numbers.iter().enumerate() {
        let ns = &numbers[y];
        let mut ni = 0;
        let mut rep = 0;
        for x in 0..grid.row_len(y).unwrap() {
            if let Some(Symbol::Number(_)) = grid.get((x, y)) {
                let n: u32 = ns[ni].parse().unwrap();
                grid.set((x, y), Symbol::Number(n));
                rep += 1;
                if rep >= ns[ni].len() {
                    rep = 0;
                    ni += 1;
                }
            }
        }
    }
    grid
}

pub fn extract_numbers_matrix(data: &str) -> Vec<Vec<Number>> {
    data.lines()
        .map(|line| {
            let s = line.split(['#', '$', '%', '&', '*', '+', '-', '.', '/', '=', '@']);
            let v: Vec<String> = s
                .map(|w| w.chars().filter(|c| c.is_numeric()).collect::<String>())
                .filter(|w| !w.is_empty())
                .collect();
            v
        })
        .collect()
}

#[allow(unused)]
pub fn display_separators(s: &str) {
    let mut seps: Vec<char> = s
        .lines()
        .flat_map(|line| line.chars().filter(|c| !c.is_numeric()).collect::<Vec<_>>())
        .collect();
    seps.sort();
    seps.dedup();
    println!("seps={seps:?}");
}

pub fn adjacent_numbers((x, y): (usize, usize), grid: &Grid<Symbol>) -> Vec<u32> {
    let mut adjacents = vec![];
    // up-left
    if let Some(Symbol::Number(n)) = grid.get((x - 1, y - 1)) {
        adjacents.push(*n);
    }
    //up
    if let Some(Symbol::Number(n)) = grid.get((x, y - 1)) {
        adjacents.push(*n);
    }
    //up-right
    if let Some(Symbol::Number(n)) = grid.get((x + 1, y - 1)) {
        adjacents.push(*n);
    }
    //right
    if let Some(Symbol::Number(n)) = grid.get((x + 1, y)) {
        adjacents.push(*n);
    }
    //down-right
    if let Some(Symbol::Number(n)) = grid.get((x + 1, y + 1)) {
        adjacents.push(*n);
    }
    //down
    if let Some(Symbol::Number(n)) = grid.get((x, y + 1)) {
        adjacents.push(*n);
    }
    //down-left
    if let Some(Symbol::Number(n)) = grid.get((x - 1, y + 1)) {
        adjacents.push(*n);
    }
    //left
    if let Some(Symbol::Number(n)) = grid.get((x - 1, y)) {
        adjacents.push(*n);
    }

    adjacents.dedup();
    adjacents
}
