#[cfg(test)]
mod tests;

/*
Represents both the expanded key (direct) and the inverse expanded key
 */
use crate::aes_block_cipher::constants::{R_CON, S_BOX};
use crate::aes_block_cipher::state::State;
use crate::aes_block_cipher::{Word, N_B, N_K, N_R};

pub struct AESKey {
    pub data: [Word; N_B * (N_R + 1)],
}

impl AESKey {
    pub fn new_direct(cipher_key: [u8; 4 * N_K as usize]) -> Self {
        let mut data = [0; N_B * (N_R + 1)];
        Self::expand_key(cipher_key, &mut data);
        Self { data }
    }

    pub fn new_inverse(cipher_key: [u8; 4 * N_K as usize]) -> Self {
        let mut data = [0; N_B * (N_R + 1)];
        Self::inv_expand_key(cipher_key, &mut data);
        Self { data }
    }

    fn expand_key(cipher_key: [u8; 4 * N_K as usize], data: &mut [Word; N_B * (N_R + 1)]) {
        let mut temp: Word;
        let mut i: usize = 0;

        while i < N_K as usize {
            data[i] = u32::from_be_bytes([
                cipher_key[4 * i],
                cipher_key[4 * i + 1],
                cipher_key[4 * i + 2],
                cipher_key[4 * i + 3],
            ]);
            i += 1;
        }

        i = N_K as usize;

        while i < (N_B * (N_R + 1)) {
            temp = data[i - 1];
            if i % N_K as usize == 0 {
                temp = Self::sub_word(Self::rot_word(temp)) ^ R_CON[i / N_K as usize - 1];
            }
            data[i] = data[i - N_K as usize] ^ temp;
            i += 1;
        }
    }

    fn inv_expand_key(cipher_key: [u8; 4 * N_K as usize], dw: &mut [Word; N_B * (N_R + 1)]) {
        Self::expand_key(cipher_key, dw);

        for round in 1..N_R {
            let new_words = Self::inv_mix_columns_words(
                &dw[round * N_B..(round + 1) * N_B].try_into().unwrap(),
            );
            for i in 0..N_B {
                dw[round * N_B + i] = new_words[i];
            }
        }
    }

    /*
    Takes a four-byte input word and applies an S-box to each
    of the four bytes to produce an output word
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

    /*
    Takes a four-byte word and performs a cyclic permutation.
    It takes a word [a0, a1, a2, a3] as input, performs a cyclic
    permutation, and returns the word [a1, a2, a3, a0]
     */
    fn rot_word(word: Word) -> Word {
        word << 8 | word >> 24
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

    fn get_byte_from_word(word: Word, pos: usize) -> u8 {
        if pos > 3 {
            panic!("pos must be less than 4");
        }

        (word >> (8 * pos)) as u8
    }

    fn apply_s_box(value: u8) -> u8 {
        let pos_x = (value >> 4) as usize;
        let pos_y = (value & 0x0f) as usize;
        S_BOX[pos_x * 16 + pos_y]
    }
}
