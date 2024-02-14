// Number of columns (32-bit words) comprising the State
const N_K: u8 = 4;
// Number of 32-bit words comprising the Cipher Key
const N_B: usize = 4;
// Number of rounds, which is a function of Nk and Nb (which is fixed)
const N_R: usize = 10;

#[derive(Debug)]
struct Matrix<const R: usize, const C: usize> {
    data: [[u8; C]; R],
}

const S_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16];

const INV_S_BOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d];

const R_CON: [u32; 10] = [0x01000000, 0x02000000, 0x04000000,
    0x08000000, 0x10000000, 0x20000000,
    0x40000000, 0x80000000, 0x1b000000,
    0x36000000];

impl<const R: usize, const C: usize> Matrix<R, C> {
    fn new() -> Self {
        Self { data: [[0; C]; R] }
    }

    #[cfg(test)]
    fn new_from_data(data: [[u8; C]; R]) -> Self {
        Self { data }
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        self.data[row][col]
    }

    fn set(&mut self, row: usize, col: usize, value: u8) {
        self.data[row][col] = value;
    }

    fn get_rows_amount(&self) -> usize {
        R
    }

    fn get_cols_amount(&self) -> usize {
        C
    }

    #[cfg(test)]
    fn get_row(&self, row: usize) -> [u8; C] {
        self.data[row]
    }

    fn get_cols(&self) -> impl Iterator<Item=[u8; R]> + '_ {
        (0..C).map(move |i| self.get_col(i))
    }


    fn get_col(&self, col: usize) -> [u8; R] {
        let mut result = [0; R];
        (0..R).for_each(|i| {
            result[i] = self.data[i][col];
        });
        result
    }

    fn set_col(&mut self, col: usize, data: [u8; R]) {
        (0..R).for_each(|i| self.data[i][col] = data[i]);
    }


    fn shift_row_left(&mut self, row: usize, amount: usize) {
        for _ in 0..amount {
            let temp = self.data[row][0];
            for i in 0..C - 1 {
                self.data[row][i] = self.data[row][i + 1];
            }
            self.data[row][C - 1] = temp;
        }
    }

    fn shift_row_right(&mut self, row: usize, amount: usize) {
        for _ in 0..amount {
            let temp = self.data[row][C - 1];
            for i in (1..C).rev() {
                self.data[row][i] = self.data[row][i - 1];
            }
            self.data[row][0] = temp;
        }
    }
}

type Word = u32;

/*
Intermediate Cipher result that can be pictured as a rectangular array
of bytes, having four rows and Nb columns.
 */
type State = Matrix<4, N_B>;

/*
Transformation in the Cipher and Inverse Cipher in which a Round
Key is added to the State using an XOR operation.
 */
fn add_round_key(state: &mut State, round_key: &[Word; N_B]) {
    (0..N_B).for_each(|i| {
        let col = state.get_col(i);
        let word = round_key[i];
        let word_bytes = word.to_be_bytes();
        let new_col = [col[0] ^ word_bytes[0], col[1] ^ word_bytes[1], col[2] ^ word_bytes[2], col[3] ^ word_bytes[3]];
        state.set_col(i, new_col);
    });
}

fn get_state_from_data_in(data_in: [u8; 4 * N_B]) -> State {
    let mut state = State::new();
    for i in 0..N_B {
        let col = [
            data_in[4 * i],
            data_in[4 * i + 1],
            data_in[4 * i + 2],
            data_in[4 * i + 3]];
        state.set_col(i, col);
    }
    state
}

fn set_data_out_from_state(data_out: &mut [u8; 4 * N_B], state: State) {
    for i in 0..N_B {
        let col = state.get_col(i);
        data_out[4 * i] = col[0];
        data_out[4 * i + 1] = col[1];
        data_out[4 * i + 2] = col[2];
        data_out[4 * i + 3] = col[3];
    }
}

fn apply_substitution(state: &mut State, sub_box: &[u8; 256]) {
    for row in 0..state.get_rows_amount() {
        for col in 0..state.get_cols_amount() {
            let value = state.get(row, col);
            state.set(row, col, sub_box[value as usize]);
        }
    }
}

fn sub_bytes(state: &mut State) {
    apply_substitution(state, &S_BOX);
}

fn inv_sub_bytes(state: &mut State) {
    apply_substitution(state, &INV_S_BOX);
}

fn shift_rows(state: &mut State) {
    for i in 1..state.get_rows_amount() {
        state.shift_row_left(i, i)
    }
}

fn inv_shift_rows(state: &mut State) {
    for i in 1..state.get_rows_amount() {
        state.shift_row_right(i, i)
    }
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


fn mix_columns(state: &mut State) {
    for i in 0..N_B {
        let col = state.get_col(i);
        let new_col = [
            galois_mul(col[0], 2) ^ galois_mul(col[1], 3) ^ col[2] ^ col[3],
            col[0] ^ galois_mul(col[1], 2) ^ galois_mul(col[2], 3) ^ col[3],
            col[0] ^ col[1] ^ galois_mul(col[2], 2) ^ galois_mul(col[3], 3),
            galois_mul(col[0], 3) ^ col[1] ^ col[2] ^ galois_mul(col[3], 2),
        ];
        state.set_col(i, new_col);
    }
}

fn inv_mix_columns(state: &mut State) {
    for i in 0..N_B {
        let col = state.get_col(i);
        let new_col = [
            galois_mul(col[0], 14) ^ galois_mul(col[1], 11) ^ galois_mul(col[2], 13) ^ galois_mul(col[3], 9),
            galois_mul(col[0], 9) ^ galois_mul(col[1], 14) ^ galois_mul(col[2], 11) ^ galois_mul(col[3], 13),
            galois_mul(col[0], 13) ^ galois_mul(col[1], 9) ^ galois_mul(col[2], 14) ^ galois_mul(col[3], 11),
            galois_mul(col[0], 11) ^ galois_mul(col[1], 13) ^ galois_mul(col[2], 9) ^ galois_mul(col[3], 14),
        ];
        state.set_col(i, new_col);
    }
}

fn build_state_from_words(words: &[Word; N_B]) -> State {
    let mut state = State::new();
    (0..N_B).for_each(|i| {
        let word = words[i];
        let word_bytes = word.to_be_bytes();
        let col = [word_bytes[0], word_bytes[1], word_bytes[2], word_bytes[3]];
        state.set_col(i, col);
    });
    state
}

fn inv_mix_columns_words(words: &[Word; N_B]) -> [Word; N_B] {
    let mut state = build_state_from_words(words);
    inv_mix_columns(&mut state);
    state
        .get_cols()
        .map(|col| u32::from_be_bytes([col[0], col[1], col[2], col[3]]))
        .collect::<Vec<_>>()[0..N_B]
        .try_into()
        .expect("Invalid length")
}

fn inv_expand_key(cipher_key: [u8; 4 * N_K as usize], dw: &mut [Word; N_B * (N_R + 1)]) {
    expand_key(cipher_key, dw);

    for round in 1..N_R {
        let new_words = inv_mix_columns_words(&dw[round * N_B..(round + 1) * N_B].try_into().unwrap());
        for i in 0..N_B {
            dw[round * N_B + i] = new_words[i];
        }
    }
}

fn cipher(data_in: [u8; 4 * N_B], data_out: &mut [u8; 4 * N_B], w: [Word; N_B * (N_R + 1)]) {
    let mut state = get_state_from_data_in(data_in);

    add_round_key(&mut state, &w[0..N_B].try_into().unwrap());

    for round in 1..N_R {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &w[(round * N_B)..((round + 1) * N_B)].try_into().unwrap());
    }
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &w[(N_R * N_B)..((N_R + 1) * N_B)].try_into().unwrap());

    set_data_out_from_state(data_out, state);
}

fn slow_inv_cipher(data_in: [u8; 4 * N_B], data_out: &mut [u8; 4 * N_B], dw: [Word; N_B * (N_R + 1)]) {
    let mut state = get_state_from_data_in(data_in);

    add_round_key(&mut state, &dw[(N_R * N_B)..((N_R + 1) * N_B)].try_into().unwrap());

    for round in (1..N_R).rev() {
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(&mut state, &dw[(round * N_B)..((round + 1) * N_B)].try_into().unwrap());
        inv_mix_columns(&mut state);
    }
    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    add_round_key(&mut state, &dw[0..N_B].try_into().unwrap());

    set_data_out_from_state(data_out, state);
}

fn inv_cipher(data_in: [u8; 4 * N_B], data_out: &mut [u8; 4 * N_B], dw: [Word; N_B * (N_R + 1)]) {
    let mut state = get_state_from_data_in(data_in);

    add_round_key(&mut state, &dw[(N_R * N_B)..((N_R + 1) * N_B)].try_into().unwrap());

    for round in (1..N_R).rev() {
        inv_sub_bytes(&mut state);
        inv_shift_rows(&mut state);
        inv_mix_columns(&mut state);
        add_round_key(&mut state, &dw[(round * N_B)..((round + 1) * N_B)].try_into().unwrap());
    }
    inv_sub_bytes(&mut state);
    inv_shift_rows(&mut state);
    add_round_key(&mut state, &dw[0..N_B].try_into().unwrap());

    set_data_out_from_state(data_out, state);
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

/*
Function used in the Key Expansion routine that takes a four-byte
input word and applies an S-box to each of the four bytes to
produce an output word
 */
fn sub_word(word: Word) -> Word {
    let mut result = 0;

    for i in 0..4 {
        let byte = get_byte_from_word(word, i);
        let new_byte = apply_s_box(byte);
        result |= (new_byte as u32) << (8 * i);
    }
    result
}

/*
Function used in the Key Expansion routine that takes a four-byte
word and performs a cyclic permutation. It takes a word [a0, a1, a2, a3]
as input, performs a cyclic permutation, and returns the word [a1, a2, a3, a0]
 */
fn rot_word(word: Word) -> Word {
    word << 8 | word >> 24
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
            temp = sub_word(rot_word(temp)) ^ R_CON[i / N_K as usize - 1];
        }
        w[i] = w[i - N_K as usize] ^ temp;
        i += 1;
    }
}

struct AESCipher {
    expanded_key: [Word; N_B * (N_R + 1)],
    inv_expanded_key: [Word; N_B * (N_R + 1)],
}

impl AESCipher {
    fn new(cipher_key: [u8; 4 * N_B]) -> Self {
        let mut expanded_key = [0; (N_B * (N_R + 1))];
        let mut inv_expanded_key = [0; (N_B * (N_R + 1))];

        expand_key(cipher_key, &mut expanded_key);
        inv_expand_key(cipher_key, &mut inv_expanded_key);

        Self { expanded_key, inv_expanded_key }
    }

    fn new_u128(cipher_key: u128) -> Self {
        let cipher_key_bytes = cipher_key.to_be_bytes();
        let mut cipher_key = [0; 4 * N_B];
        cipher_key.copy_from_slice(&cipher_key_bytes[0..4 * N_B]);
        Self::new(cipher_key)
    }

    fn cipher_block(&self, data_in: [u8; 4 * N_B]) -> [u8; 4 * N_B] {
        let mut data_out = [0; 4 * N_B];
        cipher(data_in, &mut data_out, self.expanded_key);
        data_out
    }

    #[allow(dead_code)]
    fn slow_inv_cipher_block(&self, data_in: [u8; 4 * N_B]) -> [u8; 4 * N_B] {
        let mut data_out = [0; 4 * N_B];
        slow_inv_cipher(data_in, &mut data_out, self.expanded_key);
        data_out
    }

    fn inv_cipher_block(&self, data_in: [u8; 4 * N_B]) -> [u8; 4 * N_B] {
        let mut data_out = [0; 4 * N_B];
        inv_cipher(data_in, &mut data_out, self.inv_expanded_key);
        data_out
    }
}

fn main() {
    let plain_text: [u8; 4 * N_B] = [
        0x32, 0x43, 0xf6, 0xa8,
        0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2,
        0xe0, 0x37, 0x07, 0x34];

    let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

    let cipher = AESCipher::new_u128(cipher_key);

    let cipher_block = cipher.cipher_block(plain_text);


    let plain_block = cipher.inv_cipher_block(cipher_block);

    for i in 0..(N_B * 4) {
        assert_eq!(plain_text[i], plain_block[i]);
    }
    println!("Test passed");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot_word() {
        let word: Word = 0x09cf4f3c;
        let expected_word: Word = 0xcf4f3c09;
        assert_eq!(rot_word(word), expected_word);
    }

    #[test]
    fn test_sub_word() {
        let word: Word = 0xcf4f3c09;
        let expected_word: Word = 0x8a84eb01;
        assert_eq!(sub_word(word), expected_word);
    }

    #[test]
    fn test_shift_rows() {
        let mut state = State::new_from_data([
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5]
        ]);

        let expected_state = State::new_from_data([
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xb4, 0x41, 0x27, 0xbf],
            [0x11, 0x98, 0x5d, 0x52],
            [0xe5, 0x30, 0xae, 0xf1]
        ]);

        shift_rows(&mut state);

        for i in 0..4 {
            assert_eq!(state.get_row(i), expected_state.get_row(i));
        }
    }

    #[test]
    fn test_inv_shift_rows() {
        let mut state = State::new_from_data([
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xb4, 0x41, 0x27, 0xbf],
            [0x11, 0x98, 0x5d, 0x52],
            [0xe5, 0x30, 0xae, 0xf1]
        ]);

        let expected_state = State::new_from_data([
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5]
        ]);

        inv_shift_rows(&mut state);

        for i in 0..4 {
            assert_eq!(state.get_row(i), expected_state.get_row(i));
        }
    }

    #[test]
    fn test_sub_bytes() {
        let mut state = State::new_from_data([
            [0x19, 0xa0, 0x9a, 0xe9],
            [0x3d, 0xf4, 0xc6, 0xf8],
            [0xe3, 0xe2, 0x8d, 0x48],
            [0xbe, 0x2b, 0x2a, 0x08]
        ]);

        let expected_state = State::new_from_data([
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30]
        ]);

        sub_bytes(&mut state);

        for i in 0..4 {
            assert_eq!(state.get_row(i), expected_state.get_row(i));
        }
    }

    #[test]
    fn test_inv_sub_bytes() {
        let mut state = State::new_from_data([
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30]
        ]);

        let expected_state = State::new_from_data([
            [0x19, 0xa0, 0x9a, 0xe9],
            [0x3d, 0xf4, 0xc6, 0xf8],
            [0xe3, 0xe2, 0x8d, 0x48],
            [0xbe, 0x2b, 0x2a, 0x08]
        ]);

        inv_sub_bytes(&mut state);

        for i in 0..4 {
            assert_eq!(state.get_row(i), expected_state.get_row(i));
        }
    }


    #[test]
    fn test_get_state_from_data_in() {
        let data_in: [u8; 4 * N_B] = [
            0x32, 0x88, 0x31, 0xe0,
            0x43, 0x5a, 0x31, 0x37,
            0xf6, 0x30, 0x98, 0x07,
            0xa8, 0x8d, 0xa2, 0x34];

        let expected_state = State::new_from_data([
            [0x32, 0x43, 0xf6, 0xa8],
            [0x88, 0x5a, 0x30, 0x8d],
            [0x31, 0x31, 0x98, 0xa2],
            [0xe0, 0x37, 0x07, 0x34]
        ]);

        let state = get_state_from_data_in(data_in);


        for i in 0..4 {
            assert_eq!(state.get_row(i), expected_state.get_row(i));
        }
    }

    #[test]
    fn test_set_data_out_from_state() {
        let mut data_out: [u8; 4 * N_B] = [0; 4 * N_B];

        let state = State::new_from_data([
            [0x39, 0x02, 0xdc, 0x19],
            [0x25, 0xdc, 0x11, 0x6a],
            [0x84, 0x09, 0x85, 0x0b],
            [0x1d, 0xfb, 0x97, 0x32]
        ]);

        let expected_data_out: [u8; 4 * N_B] = [
            0x39, 0x25, 0x84, 0x1d,
            0x02, 0xdc, 0x09, 0xfb,
            0xdc, 0x11, 0x85, 0x97,
            0x19, 0x6a, 0x0b, 0x32];

        set_data_out_from_state(&mut data_out, state);

        for i in 0..(N_B * 4) {
            assert_eq!(data_out[i], expected_data_out[i]);
        }
    }


    #[test]
    fn test_key_expansion_aes_128() {
        let cipher_key: [u8; 4 * N_K as usize] = [
            0x2b, 0x7e, 0x15, 0x16,
            0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88,
            0x09, 0xcf, 0x4f, 0x3c];

        let expected_words: [Word; N_B * (N_R + 1)] = [
            0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c,
            0xa0fafe17, 0x88542cb1, 0x23a33939, 0x2a6c7605,
            0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f,
            0x3d80477d, 0x4716fe3e, 0x1e237e44, 0x6d7a883b,
            0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00,
            0xd4d1c6f8, 0x7c839d87, 0xcaf2b8bc, 0x11f915bc,
            0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
            0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f,
            0xead27321, 0xb58dbad2, 0x312bf560, 0x7f8d292f,
            0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e,
            0xd014f9a8, 0xc9ee2589, 0xe13f0cc8, 0xb6630ca6];

        let mut w: [Word; N_B * (N_R + 1)] = [0; (N_B * (N_R + 1))];

        expand_key(cipher_key, &mut w);

        for i in 0..(N_B * (N_R + 1)) {
            assert_eq!(w[i], expected_words[i]);
        }
    }

    #[test]
    fn test_mix_columns() {
        let mut state = State::new_from_data([
            [0xdb, 0xf2, 0x01, 0xc6],
            [0x13, 0x0a, 0x01, 0xc6],
            [0x53, 0x22, 0x01, 0xc6],
            [0x45, 0x5c, 0x01, 0xc6]
        ]);

        let expected_state = State::new_from_data([
            [0x8e, 0x9f, 0x01, 0xc6],
            [0x4d, 0xdc, 0x01, 0xc6],
            [0xa1, 0x58, 0x01, 0xc6],
            [0xbc, 0x9d, 0x01, 0xc6]
        ]);

        mix_columns(&mut state);

        for i in 0..4 {
            assert_eq!(state.get_row(i), expected_state.get_row(i));
        }
    }

    /*
    #[test]
    fn test_cipher() {
        let plain_bytes: [u8; 4 * N_B] = [
            0x32, 0x43, 0xf6, 0xa8,
            0x88, 0x5a, 0x30, 0x8d,
            0x31, 0x31, 0x98, 0xa2,
            0xe0, 0x37, 0x07, 0x34];


        let cipher_key: [u8; 4 * N_K as usize] = [
            0x2b, 0x7e, 0x15, 0x16,
            0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88,
            0x09, 0xcf, 0x4f, 0x3c];

        let expected_cipher_bytes: [u8; 4 * N_B] = [
            0x39, 0x25, 0x84, 0x1d,
            0x02, 0xdc, 0x09, 0xfb,
            0xdc, 0x11, 0x85, 0x97,
            0x19, 0x6a, 0x0b, 0x32];

        let mut w: [Word; (N_B * (N_R + 1))] = [0; (N_B * (N_R + 1))];
        expand_key(cipher_key, &mut w);

        let mut cipher_bytes: [u8; 4 * N_B] = [0; 4 * N_B];
        cipher(plain_bytes, &mut cipher_bytes, w);

        for i in 0..(N_B * 4) {
            assert_eq!(cipher_bytes[i], expected_cipher_bytes[i]);
        }
    }
     */

    #[test]
    fn test_cipher() {
        let plain_bytes: [u8; 4 * N_B] = [
            0x32, 0x43, 0xf6, 0xa8,
            0x88, 0x5a, 0x30, 0x8d,
            0x31, 0x31, 0x98, 0xa2,
            0xe0, 0x37, 0x07, 0x34];


        let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

        let expected_cipher_bytes: [u8; 4 * N_B] = [
            0x39, 0x25, 0x84, 0x1d,
            0x02, 0xdc, 0x09, 0xfb,
            0xdc, 0x11, 0x85, 0x97,
            0x19, 0x6a, 0x0b, 0x32];

        let cipher = AESCipher::new_u128(cipher_key);

        let block = cipher.cipher_block(plain_bytes);

        for i in 0..(N_B * 4) {
            assert_eq!(block[i], expected_cipher_bytes[i]);
        }
    }

    #[test]
    fn test_inv_mix_columns() {
        let mut state = State::new_from_data([
            [0x8e, 0x9f, 0x01, 0xc6],
            [0x4d, 0xdc, 0x01, 0xc6],
            [0xa1, 0x58, 0x01, 0xc6],
            [0xbc, 0x9d, 0x01, 0xc6]
        ]);

        let expected_state = State::new_from_data([
            [0xdb, 0xf2, 0x01, 0xc6],
            [0x13, 0x0a, 0x01, 0xc6],
            [0x53, 0x22, 0x01, 0xc6],
            [0x45, 0x5c, 0x01, 0xc6]
        ]);

        inv_mix_columns(&mut state);

        for i in 0..4 {
            assert_eq!(state.get_row(i), expected_state.get_row(i));
        }
    }


    #[test]
    fn test_inv_cipher() {
        let expected_plain_text: [u8; 4 * N_B] = [
            0x32, 0x43, 0xf6, 0xa8,
            0x88, 0x5a, 0x30, 0x8d,
            0x31, 0x31, 0x98, 0xa2,
            0xe0, 0x37, 0x07, 0x34];

        let cipher_key: [u8; 4 * N_K as usize] = [
            0x2b, 0x7e, 0x15, 0x16,
            0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88,
            0x09, 0xcf, 0x4f, 0x3c];

        let cipher_bytes: [u8; 4 * N_B] = [
            0x39, 0x25, 0x84, 0x1d,
            0x02, 0xdc, 0x09, 0xfb,
            0xdc, 0x11, 0x85, 0x97,
            0x19, 0x6a, 0x0b, 0x32];

        let mut dw: [Word; N_B * (N_R + 1)] = [0; (N_B * (N_R + 1))];
        inv_expand_key(cipher_key, &mut dw);

        let mut plain_text: [u8; 4 * N_B] = [0; 4 * N_B];
        inv_cipher(cipher_bytes, &mut plain_text, dw);

        for i in 0..(N_B * 4) {
            assert_eq!(plain_text[i], expected_plain_text[i]);
        }
    }
}

