#[derive(Debug)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[u8; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new() -> Self {
        Self { data: [[0; C]; R] }
    }

    pub fn new_from_data(data: [[u8; C]; R]) -> Self {
        Self { data }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.data[row][col] = value;
    }

    pub fn get_rows_amount(&self) -> usize {
        R
    }

    pub fn get_cols_amount(&self) -> usize {
        C
    }

    #[cfg(test)]
    pub fn get_row(&self, row: usize) -> [u8; C] {
        self.data[row]
    }

    pub fn get_cols(&self) -> impl Iterator<Item = [u8; R]> + '_ {
        (0..C).map(move |i| self.get_col(i))
    }

    pub fn get_col(&self, col: usize) -> [u8; R] {
        let mut result = [0; R];
        (0..R).for_each(|i| {
            result[i] = self.data[i][col];
        });
        result
    }

    pub fn set_col(&mut self, col: usize, data: [u8; R]) {
        (0..R).for_each(|i| self.data[i][col] = data[i]);
    }

    pub fn shift_row_left(&mut self, row: usize, amount: usize) {
        for _ in 0..amount {
            let temp = self.data[row][0];
            for i in 0..C - 1 {
                self.data[row][i] = self.data[row][i + 1];
            }
            self.data[row][C - 1] = temp;
        }
    }

    pub fn shift_row_right(&mut self, row: usize, amount: usize) {
        for _ in 0..amount {
            let temp = self.data[row][C - 1];
            for i in (1..C).rev() {
                self.data[row][i] = self.data[row][i - 1];
            }
            self.data[row][0] = temp;
        }
    }
}