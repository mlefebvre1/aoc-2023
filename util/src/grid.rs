use std::fmt::Display;

type InnerGrid<T> = Vec<Vec<T>>;

#[derive(Debug)]
pub struct Grid<T: PartialEq>(InnerGrid<T>);

impl<T: PartialEq> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Grid<T> {
        Self(data)
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.0.get(y).and_then(|row| row.get(x))
    }
    pub fn set(&mut self, (x, y): (usize, usize), value: T) {
        if let Some(c) = self.0.get_mut(y).and_then(|row| row.get_mut(x)) {
            *c = value;
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.0.iter()
    }
    pub fn row_len(&self, y: usize) -> Option<usize> {
        self.0.get(y).map(|row| row.len())
    }
    pub fn nb_rows(&self) -> usize {
        self.0.len()
    }
    pub fn insert_row(&mut self, row_index: usize, row: Vec<T>) {
        self.0.insert(row_index, row);
    }
    pub fn replace_row(&mut self, row_index: usize, row: Vec<T>) {
        self.0[row_index] = row;
    }

    pub fn columns(&self) -> Vec<Vec<&T>> {
        (0..self.nb_columns())
            .map(|x| (0..self.nb_rows()).map(|y| &self.0[y][x]).collect())
            .collect()
    }
    pub fn nb_columns(&self) -> usize {
        self.0[0].len()
    }
    pub fn insert_column(&mut self, column_index: usize, column: Vec<T>) {
        for (y, col) in (0..self.nb_rows()).zip(column) {
            self.0[y].insert(column_index, col);
        }
    }
    pub fn replace_column(&mut self, column_index: usize, column: Vec<T>) {
        for (y, col) in (0..self.nb_rows()).zip(column) {
            self.0[y][column_index] = col;
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        let ylen = self.nb_rows();
        let xlen = self.rows().nth(0).unwrap().len();
        (xlen, ylen)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn find(&self, item: &T) -> Option<(usize, usize)> {
        for (y, row) in self.rows().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col == item {
                    return Some((x, y));
                }
            }
        }
        None
    }
    pub fn find_all(&self, item: &T) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for (y, row) in self.rows().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col == item {
                    v.push((x, y));
                }
            }
        }
        v
    }
}
impl<T: PartialEq + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.rows().for_each(|row| {
            row.iter().for_each(|col| {
                s.push_str(&format!("{col}"));
            });
            s.push('\n');
        });
        write!(f, "{s}")
    }
}
