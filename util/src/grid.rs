type InnerGrid<T> = Vec<Vec<T>>;

pub struct Grid<T>(InnerGrid<T>);

impl<T> Grid<T> {
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

    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn row_len(&self, y: usize) -> Option<usize> {
        self.0.get(y).map(|row| row.len())
    }
}
