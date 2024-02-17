use std::convert::TryInto;

#[cfg(test)]
mod tests;

mod constants;
mod state;

use constants::R_CON;
use state::State;
use crate::aes_cipher::constants::S_BOX;

// Number of columns (32-bit words) comprising the State
pub const N_K: u8 = 4;
// Number of 32-bit words comprising the Cipher Key
pub const N_B: usize = 4;
// Number of rounds, which is a function of Nk and Nb (which is fixed)
pub const N_R: usize = 10;

pub type Word = u32;

pub struct AESCipher {
    expanded_key: [Word; N_B * (N_R + 1)],
    inv_expanded_key: [Word; N_B * (N_R + 1)],
}

impl AESCipher {
    pub fn new(cipher_key: [u8; 4 * N_B]) -> Self {
        let mut expanded_key = [0; (N_B * (N_R + 1))];
        let mut inv_expanded_key = [0; (N_B * (N_R + 1))];

        Self::expand_key(cipher_key, &mut expanded_key);
        Self::inv_expand_key(cipher_key, &mut inv_expanded_key);

        Self { expanded_key, inv_expanded_key }
    }

    pub fn new_u128(cipher_key: u128) -> Self {
        let cipher_key_bytes = cipher_key.to_be_bytes();
        let mut cipher_key = [0; 4 * N_B];
        cipher_key.copy_from_slice(&cipher_key_bytes[0..4 * N_B]);
        Self::new(cipher_key)
    }

    fn expand_key(cipher_key: [u8; 4 * N_K as usize], w: &mut [Word; N_B * (N_R + 1)]) {
        let mut temp: Word;
        let mut i: usize = 0;

        while i < N_K as usize {
            w[i] = u32::from_be_bytes([cipher_key[4 * i], cipher_key[4 * i + 1], cipher_key[4 * i + 2], cipher_key[4 * i + 3]]);
            i += 1;
        }

        i = N_K as usize;

        while i < (N_B * (N_R + 1)) {
            temp = w[i - 1];
            if i % N_K as usize == 0 {
                temp = Self::sub_word(Self::rot_word(temp)) ^ R_CON[i / N_K as usize - 1];
            }
            w[i] = w[i - N_K as usize] ^ temp;
            i += 1;
        }
    }

    fn inv_expand_key(cipher_key: [u8; 4 * N_K as usize], dw: &mut [Word; N_B * (N_R + 1)]) {
        Self::expand_key(cipher_key, dw);

        for round in 1..N_R {
            let new_words = Self::inv_mix_columns_words(&dw[round * N_B..(round + 1) * N_B].try_into().unwrap());
            for i in 0..N_B {
                dw[round * N_B + i] = new_words[i];
            }
        }
    }

    pub fn cipher_block(&self, data_in: [u8; 4 * N_B]) -> [u8; 4 * N_B] {
        let mut data_out = [0; 4 * N_B];

        let mut state = State::new_from_data_in(data_in);

        state.add_round_key(&self.expanded_key[0..N_B].try_into().unwrap());

        for round in 1..N_R {
            state.sub_bytes();
            state.shift_rows();
            state.mix_columns();
            state.add_round_key(&self.expanded_key[(round * N_B)..((round + 1) * N_B)].try_into().unwrap());
        }
        state.sub_bytes();
        state.shift_rows();
        state.add_round_key(&self.expanded_key[(N_R * N_B)..((N_R + 1) * N_B)].try_into().unwrap());

        state.set_data_out(&mut data_out);

        data_out
    }

    #[allow(dead_code)]
    fn slow_inv_cipher_block(&self, data_in: [u8; 4 * N_B]) -> [u8; 4 * N_B] {
        let mut data_out = [0; 4 * N_B];
        let mut state = State::new_from_data_in(data_in);

        state.add_round_key(&self.inv_expanded_key[(N_R * N_B)..((N_R + 1) * N_B)].try_into().unwrap());

        for round in (1..N_R).rev() {
            state.inv_shift_rows();
            state.inv_sub_bytes();
            state.add_round_key(&self.inv_expanded_key[(round * N_B)..((round + 1) * N_B)].try_into().unwrap());
            state.inv_mix_columns();
        }
        state.inv_shift_rows();
        state.inv_sub_bytes();
        state.add_round_key(&self.inv_expanded_key[0..N_B].try_into().unwrap());

        state.set_data_out(&mut data_out);

        data_out
    }

    pub fn inv_cipher_block(&self, data_in: [u8; 4 * N_B]) -> [u8; 4 * N_B] {
        let mut data_out = [0; 4 * N_B];

        let mut state = State::new_from_data_in(data_in);

        state.add_round_key(&self.inv_expanded_key[(N_R * N_B)..((N_R + 1) * N_B)].try_into().unwrap());

        for round in (1..N_R).rev() {
            state.inv_sub_bytes();
            state.inv_shift_rows();
            state.inv_mix_columns();
            state.add_round_key(&self.inv_expanded_key[(round * N_B)..((round + 1) * N_B)].try_into().unwrap());
        }
        state.inv_sub_bytes();
        state.inv_shift_rows();
        state.add_round_key(&self.inv_expanded_key[0..N_B].try_into().unwrap());

        state.set_data_out(&mut data_out);

        data_out
    }

    /*
    Used in the Key Expansion routine that takes a four-byte
    word and performs a cyclic permutation. It takes a word [a0, a1, a2, a3]
    as input, performs a cyclic permutation, and returns the word [a1, a2, a3, a0]
     */
    fn rot_word(word: Word) -> Word {
        word << 8 | word >> 24
    }

    /*
    Used in the Key Expansion routine that takes a four-byte
    input word and applies an S-box to each of the four bytes to
    produce an output word
     */
    fn sub_word(word: Word) -> Word {
        let mut result = 0;

        for i in 0..4 {
            let byte = Self::get_byte_from_word(word, i);
            let new_byte = Self::apply_s_box(byte);
            result |= (new_byte as u32) << (8 * i);
        }
        result
    }

    fn apply_s_box(value: u8) -> u8 {
        let pos_x = (value >> 4) as usize;
        let pos_y = (value & 0x0f) as usize;
        S_BOX[pos_x * 16 + pos_y]
    }

    fn get_byte_from_word(word: Word, pos: usize) -> u8 {
        if pos > 3 {
            panic!("pos must be less than 4");
        }

        (word >> (8 * pos)) as u8
    }

    fn inv_mix_columns_words(words: &[Word; N_B]) -> [Word; N_B] {
        let mut state = State::new_from_words(words);
        state.inv_mix_columns();
        state
            .data
            .get_cols()
            .map(|col| u32::from_be_bytes([col[0], col[1], col[2], col[3]]))
            .collect::<Vec<_>>()[0..N_B]
            .try_into()
            .expect("Invalid length")
    }
}
