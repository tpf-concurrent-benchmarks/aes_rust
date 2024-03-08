/*
Intermediate Cipher result that can be pictured as a rectangular array
of bytes, having four rows and Nb columns.
 */
use super::constants::{INV_S_BOX, S_BOX};
use super::N_B;
use crate::aes_block_cipher::Word;
use crate::utils::Matrix;

pub struct State {
    pub data: Matrix,
}

impl State {
    pub fn new() -> Self {
        Self {
            data: Matrix::new(),
        }
    }

    #[cfg(test)]
    pub fn new_from_matrix(data: Matrix) -> Self {
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

    pub fn mix_columns(&mut self) {
        for i in 0..N_B {
            let mut col = self.data.get_col(i);
            Self::mix_column(&mut col);
            self.data.set_col(i, col);
        }
    }

    pub fn inv_mix_columns(&mut self) {
        for i in 0..N_B {
            let mut col = self.data.get_col(i);
            Self::inv_mix_column(&mut col);
            self.data.set_col(i, col);
        }
    }

    // Source: https://crypto.stackexchange.com/a/71206
    fn mix_column(col: &mut [u8; 4]) {
        let a = col[0];
        let b = col[1];
        let c = col[2];
        let d = col[3];
        col[0] = Self::galois_dobule((a ^ b) as i8) ^ b ^ c ^ d;
        col[1] = Self::galois_dobule((b ^ c) as i8) ^ c ^ d ^ a;
        col[2] = Self::galois_dobule((c ^ d) as i8) ^ d ^ a ^ b;
        col[3] = Self::galois_dobule((d ^ a) as i8) ^ a ^ b ^ c;
    }   

    fn inv_mix_column(col: &mut [u8; 4]) {
        let a = col[0];
        let b = col[1];
        let c = col[2];
        let d = col[3];
        let x = Self::galois_dobule((a ^ b ^ c ^ d) as i8);
        let y = Self::galois_dobule((x ^ a ^ c) as i8);
        let z = Self::galois_dobule((x ^ b ^ d) as i8);
        col[0] = Self::galois_dobule((y ^ a ^ b) as i8) ^ b ^ c ^ d;
        col[1] = Self::galois_dobule((z ^ b ^ c) as i8) ^ c ^ d ^ a;
        col[2] = Self::galois_dobule((y ^ c ^ d) as i8) ^ d ^ a ^ b;
        col[3] = Self::galois_dobule((z ^ d ^ a) as i8) ^ a ^ b ^ c;
    }


    #[inline]
    fn galois_dobule(a: i8) -> u8 {
        let mut result = (a << 1) as u8;
        if a < 0 {
            result ^= 0x1b;
        }
        result
    }
}
