#[derive(Debug)]
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

pub type Matrix<T> = Vec<Vec<T>>;
pub type Number = String;

pub fn make_symbol_matrix(data: &str, numbers: &[Vec<String>]) -> Matrix<Symbol> {
    let mut matrix: Matrix<Symbol> = data
        .lines()
        .map(|line| line.chars().map(Symbol::from).collect())
        .collect();

    for y in 0..matrix.len() {
        let ns = &numbers[y];
        let mut ni = 0;
        let mut rep = 0;
        for x in 0..matrix[y].len() {
            if let Symbol::Number(_) = matrix[y][x] {
                let n: u32 = ns[ni].parse().unwrap();
                matrix[y][x] = Symbol::Number(n);
                rep += 1;
                if rep >= ns[ni].len() {
                    rep = 0;
                    ni += 1;
                }
            }
        }
    }
    matrix
}

pub fn extract_numbers_matrix(data: &str) -> Matrix<Number> {
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

pub fn adjacent_numbers((x, y): (usize, usize), matrix: &Matrix<Symbol>) -> Vec<u32> {
    let mut adjacents = vec![];
    // up-left
    if let Some(Symbol::Number(n)) = get_number(matrix, (x - 1, y - 1)) {
        adjacents.push(*n);
    }
    //up
    if let Some(Symbol::Number(n)) = get_number(matrix, (x, y - 1)) {
        adjacents.push(*n);
    }
    //up-right
    if let Some(Symbol::Number(n)) = get_number(matrix, (x + 1, y - 1)) {
        adjacents.push(*n);
    }
    //right
    if let Some(Symbol::Number(n)) = get_number(matrix, (x + 1, y)) {
        adjacents.push(*n);
    }
    //down-right
    if let Some(Symbol::Number(n)) = get_number(matrix, (x + 1, y + 1)) {
        adjacents.push(*n);
    }
    //down
    if let Some(Symbol::Number(n)) = get_number(matrix, (x, y + 1)) {
        adjacents.push(*n);
    }
    //down-left
    if let Some(Symbol::Number(n)) = get_number(matrix, (x - 1, y + 1)) {
        adjacents.push(*n);
    }
    //left
    if let Some(Symbol::Number(n)) = get_number(matrix, (x - 1, y)) {
        adjacents.push(*n);
    }

    adjacents.dedup();
    adjacents
}

fn get_number(matrix: &Matrix<Symbol>, (x, y): (usize, usize)) -> Option<&Symbol> {
    matrix.get(y).and_then(|row| row.get(x))
}
