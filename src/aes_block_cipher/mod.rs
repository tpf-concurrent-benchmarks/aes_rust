use std::convert::TryInto;

#[cfg(test)]
mod tests;

mod aes_key;
mod constants;
pub mod state;

use aes_key::AESKey;

use state::State;

// Number of columns (32-bit words) comprising the State
pub const N_K: u8 = 4;
// Number of 32-bit words comprising the Cipher Key
pub const N_B: usize = 4;
// Number of rounds, which is a function of Nk and Nb (which is fixed)
pub const N_R: usize = 10;

pub type Word = u32;

pub struct AESBlockCipher {
    expanded_key: AESKey,
    inv_expanded_key: AESKey,
}

impl AESBlockCipher {
    pub fn new(cipher_key: [u8; 4 * N_B]) -> Self {
        let expanded_key = AESKey::new_direct(cipher_key);
        let inv_expanded_key = AESKey::new_inverse(cipher_key);

        Self {
            expanded_key,
            inv_expanded_key,
        }
    }

    pub fn new_u128(cipher_key: u128) -> Self {
        let cipher_key_bytes = cipher_key.to_be_bytes();
        let mut cipher_key = [0; 4 * N_B];
        cipher_key.copy_from_slice(&cipher_key_bytes[0..4 * N_B]);
        Self::new(cipher_key)
    }

    pub fn cipher_block(&self, data_in: &[u8; 4 * N_B]) -> [u8; 4 * N_B] {
        let mut data_out = [0; 4 * N_B];

        let mut state = State::new_from_data_in(data_in);

        state.add_round_key(&self.expanded_key.data[0..N_B].try_into().unwrap());

        for round in 1..N_R {
            state.sub_bytes();
            state.shift_rows();
            state.mix_columns();
            state.add_round_key(
                &self.expanded_key.data[(round * N_B)..((round + 1) * N_B)]
                    .try_into()
                    .unwrap(),
            );
        }
        state.sub_bytes();
        state.shift_rows();
        state.add_round_key(
            &self.expanded_key.data[(N_R * N_B)..((N_R + 1) * N_B)]
                .try_into()
                .unwrap(),
        );

        state.set_data_out(&mut data_out);

        data_out
    }

    pub fn inv_cipher_block(&self, data_in: &[u8; 4 * N_B]) -> [u8; 4 * N_B] {
        let mut data_out = [0; 4 * N_B];

        let mut state = State::new_from_data_in(data_in);

        state.add_round_key(
            &self.inv_expanded_key.data[(N_R * N_B)..((N_R + 1) * N_B)]
                .try_into()
                .unwrap(),
        );

        for round in (1..N_R).rev() {
            state.inv_sub_bytes();
            state.inv_shift_rows();
            state.inv_mix_columns();
            state.add_round_key(
                &self.inv_expanded_key.data[(round * N_B)..((round + 1) * N_B)]
                    .try_into()
                    .unwrap(),
            );
        }
        state.inv_sub_bytes();
        state.inv_shift_rows();
        state.add_round_key(&self.inv_expanded_key.data[0..N_B].try_into().unwrap());

        state.set_data_out(&mut data_out);

        data_out
    }
}
