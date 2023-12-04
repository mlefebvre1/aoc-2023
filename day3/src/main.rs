#[derive(Debug)]
enum Symbol {
    Number(u32),
    Period,
    Other,
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Period,
            c if c.is_numeric() => Self::Number(c.to_digit(10).unwrap()),
            _ => Self::Other,
        }
    }
}

type Matrix = Vec<Vec<Symbol>>;

fn make_matrix(data: &str, numbers: &[Vec<String>]) -> Matrix {
    let mut matrix: Matrix = data
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

fn extract_numbers(data: &str) -> Vec<Vec<String>> {
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
fn display_separators(s: &str) {
    let mut seps: Vec<char> = s
        .lines()
        .flat_map(|line| line.chars().filter(|c| !c.is_numeric()).collect::<Vec<_>>())
        .collect();
    seps.sort();
    seps.dedup();
    println!("seps={seps:?}");
}

fn get_touch_lists(matrix: &Matrix) -> Vec<Vec<u32>> {
    let mut touch_lists = vec![];
    for (y, row) in matrix.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if let Symbol::Other = s {
                let mut touch_list = vec![];
                if y != 0 {
                    if x != 0 {
                        // up-left
                        if let Symbol::Number(n) = matrix[y - 1][x - 1] {
                            touch_list.push(n)
                        }
                    }
                    //up
                    if let Symbol::Number(n) = matrix[y - 1][x] {
                        touch_list.push(n)
                    }
                    if x != row.len() - 1 {
                        //up-right
                        if let Symbol::Number(n) = matrix[y - 1][x + 1] {
                            touch_list.push(n)
                        }
                    }
                }
                if y != matrix.len() - 1 {
                    if x != row.len() - 1 {
                        //down-right
                        if let Symbol::Number(n) = matrix[y + 1][x + 1] {
                            touch_list.push(n)
                        }
                    }
                    //down
                    if let Symbol::Number(n) = matrix[y + 1][x] {
                        touch_list.push(n)
                    }
                    if x != 0 {
                        //down-left
                        if let Symbol::Number(n) = matrix[y + 1][x - 1] {
                            touch_list.push(n)
                        }
                    }
                }

                if x != row.len() - 1 {
                    //right
                    if let Symbol::Number(n) = matrix[y][x + 1] {
                        touch_list.push(n)
                    }
                }

                if x != 0 {
                    //left
                    if let Symbol::Number(n) = matrix[y][x - 1] {
                        touch_list.push(n)
                    }
                }
                // touch_list.sort();
                touch_list.dedup();
                touch_lists.push(touch_list);
            }
        }
    }
    touch_lists
}

fn main() {
    println!("part1={}", part1());
    println!("part2={}", part2());
}

fn part1() -> String {
    let data = std::fs::read_to_string("day3/data/day3.txt").unwrap();
    // display_separators(&data);
    let numbers = extract_numbers(&data);
    let matrix = make_matrix(&data, &numbers);
    let ans: u32 = get_touch_lists(&matrix).iter().flatten().sum();
    ans.to_string()
}

fn part2() -> String {
    let ans = "";
    ans.to_string()
}
