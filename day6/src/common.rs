#[derive(Debug)]
pub struct Race {
    time: usize,
    record: usize,
}

impl Race {
    pub fn new(time: usize, record: usize) -> Self {
        Self { time, record }
    }
    pub fn nb_ways(&self) -> usize {
        self.simulation()
            .iter()
            .filter(|&dist| *dist > self.record)
            .count()
    }

    fn simulation(&self) -> Vec<usize> {
        (0..self.time)
            .map(|hold| hold * (self.time - hold))
            .collect()
    }
}
