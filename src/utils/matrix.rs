/// Efficient matrix implementation for 4x4 matrix of u8 values
/// First 4 bytes are first column, next 4 bytes are second column, etc.
#[derive(Debug)]
pub struct Matrix {
    data: u128,
}

impl Matrix {
    pub fn new() -> Self {
        Self { data: 0 }
    }

    #[cfg(test)]
    pub fn new_from_data(data: [[u8; 4]; 4]) -> Self {
        let mut result = 0;
        for i in 0..4 {
            for j in 0..4 {
                result |= (data[i][j] as u128) << (8 * (i + 4 * j));
            }
        }
        Self { data: result }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        ((self.data >> (8 * (row + 4 * col))) & 0xff) as u8
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        let mask = 0xff << (8 * (row + 4 * col));
        self.data &= !mask;
        self.data |= (value as u128) << (8 * (row + 4 * col));
    }

    pub fn get_rows_amount(&self) -> usize {
        4
    }

    pub fn get_cols_amount(&self) -> usize {
        4
    }

    #[cfg(test)]
    pub fn get_row(&self, row: usize) -> [u8; 4] {
        let mut result = [0; 4];
        for i in 0..4 {
            result[i] = self.get(row, i);
        }
        result
    }

    pub fn get_cols(&self) -> impl Iterator<Item = [u8; 4]> + '_ {
        (0..4).map(|i| {
            let mut result = [0; 4];
            for j in 0..4 {
                result[j] = self.get(j, i);
            }
            result
        })
    }

    pub fn get_col(&self, col: usize) -> [u8; 4] {
        let mut result = [0; 4];
        for i in 0..4 {
            result[i] = self.get(i, col);
        }
        result
    }

    pub fn set_col(&mut self, col: usize, data: [u8; 4]) {
        for i in 0..4 {
            self.set(i, col, data[i]);
        }
    }

    pub fn shift_row_left(&mut self, row: usize, amount: usize) {
        if amount > 2 {
            self.shift_row_right(row, 4 - amount);
            return;
        }
        for _ in 0..amount {
            let temp = self.get(row, 0);
            for i in 0..3 {
                self.set(row, i, self.get(row, i + 1));
            }
            self.set(row, 3, temp);
        }
    }

    pub fn shift_row_right(&mut self, row: usize, amount: usize) {
        if amount > 2 {
            self.shift_row_left(row, 4 - amount);
            return;
        }
        for _ in 0..amount {
            let temp = self.get(row, 3);
            for i in (1..4).rev() {
                self.set(row, i, self.get(row, i - 1));
            }
            self.set(row, 0, temp);
        }
    }
}
