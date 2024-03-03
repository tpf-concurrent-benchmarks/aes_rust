/*
Intermediate Cipher result that can be pictured as a rectangular array
of bytes, having four rows and Nb columns.
 */
use super::constants::{INV_S_BOX, S_BOX};
use super::N_B;
use crate::aes_cipher::Word;
use crate::matrix::Matrix;

pub struct State {
    pub data: Matrix<4, N_B>,
}

impl State {
    pub fn new() -> Self {
        Self {
            data: Matrix::new(),
        }
    }

    #[cfg(test)]
    pub fn new_from_matrix(data: Matrix<4, N_B>) -> Self {
        Self { data }
    }

    #[cfg(test)]
    pub fn new_from_data(data: [[u8; N_B]; 4]) -> Self {
        let matrix = Matrix::new_from_data(data);
        Self::new_from_matrix(matrix)
    }

    pub fn new_from_data_in(data_in: &[u8; 4 * N_B]) -> Self {
        let mut state = State::new();
        for i in 0..N_B {
            let col = [
                data_in[4 * i],
                data_in[4 * i + 1],
                data_in[4 * i + 2],
                data_in[4 * i + 3],
            ];
            state.data.set_col(i, col);
        }
        state
    }

    pub fn new_from_words(words: &[Word; N_B]) -> Self {
        let mut state = Self::new();
        (0..N_B).for_each(|i| {
            let word = words[i];
            let word_bytes = word.to_be_bytes();
            let col = [word_bytes[0], word_bytes[1], word_bytes[2], word_bytes[3]];
            state.data.set_col(i, col);
        });
        state
    }

    pub fn set_data_out(self, data_out: &mut [u8; 4 * N_B]) {
        for i in 0..N_B {
            let col = self.data.get_col(i);
            data_out[4 * i] = col[0];
            data_out[4 * i + 1] = col[1];
            data_out[4 * i + 2] = col[2];
            data_out[4 * i + 3] = col[3];
        }
    }

    pub fn sub_bytes(&mut self) {
        self.apply_substitution(&S_BOX);
    }

    pub fn inv_sub_bytes(&mut self) {
        self.apply_substitution(&INV_S_BOX);
    }

    fn apply_substitution(&mut self, sub_box: &[u8; 256]) {
        for row in 0..self.data.get_rows_amount() {
            for col in 0..self.data.get_cols_amount() {
                let value = self.data.get(row, col);
                self.data.set(row, col, sub_box[value as usize]);
            }
        }
    }

    pub fn shift_rows(&mut self) {
        for i in 1..self.data.get_rows_amount() {
            self.data.shift_row_left(i, i)
        }
    }

    pub fn inv_shift_rows(&mut self) {
        for i in 1..self.data.get_rows_amount() {
            self.data.shift_row_right(i, i)
        }
    }

    pub fn mix_columns(&mut self) {
        for i in 0..N_B {
            let col = self.data.get_col(i);
            let new_col = [
                Self::galois_mul(col[0], 2) ^ Self::galois_mul(col[1], 3) ^ col[2] ^ col[3],
                col[0] ^ Self::galois_mul(col[1], 2) ^ Self::galois_mul(col[2], 3) ^ col[3],
                col[0] ^ col[1] ^ Self::galois_mul(col[2], 2) ^ Self::galois_mul(col[3], 3),
                Self::galois_mul(col[0], 3) ^ col[1] ^ col[2] ^ Self::galois_mul(col[3], 2),
            ];
            self.data.set_col(i, new_col);
        }
    }

    pub fn inv_mix_columns(&mut self) {
        for i in 0..N_B {
            let col = self.data.get_col(i);
            let new_col = [
                Self::galois_mul(col[0], 14)
                    ^ Self::galois_mul(col[1], 11)
                    ^ Self::galois_mul(col[2], 13)
                    ^ Self::galois_mul(col[3], 9),
                Self::galois_mul(col[0], 9)
                    ^ Self::galois_mul(col[1], 14)
                    ^ Self::galois_mul(col[2], 11)
                    ^ Self::galois_mul(col[3], 13),
                Self::galois_mul(col[0], 13)
                    ^ Self::galois_mul(col[1], 9)
                    ^ Self::galois_mul(col[2], 14)
                    ^ Self::galois_mul(col[3], 11),
                Self::galois_mul(col[0], 11)
                    ^ Self::galois_mul(col[1], 13)
                    ^ Self::galois_mul(col[2], 9)
                    ^ Self::galois_mul(col[3], 14),
            ];
            self.data.set_col(i, new_col);
        }
    }

    /*
    Transformation in the Cipher and Inverse Cipher in which a Round
    Key is added to the State using an XOR operation.
     */
    pub fn add_round_key(&mut self, round_key: &[Word; N_B]) {
        (0..N_B).for_each(|i| {
            let col = self.data.get_col(i);
            let word = round_key[i];
            let word_bytes = word.to_be_bytes();
            let new_col = [
                col[0] ^ word_bytes[0],
                col[1] ^ word_bytes[1],
                col[2] ^ word_bytes[2],
                col[3] ^ word_bytes[3],
            ];
            self.data.set_col(i, new_col);
        });
    }

    fn galois_mul(a: u8, b: u8) -> u8 {
        let mut result = 0;
        let mut a = a;
        let mut b = b;
        while b != 0 {
            if b & 1 != 0 {
                result ^= a;
            }
            if a & 0x80 != 0 {
                a = (a << 1) ^ 0x1b;
            } else {
                a <<= 1;
            }
            b >>= 1;
        }
        result
    }
}
