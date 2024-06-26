use super::*;

#[test]
fn test_shift_rows() {
    let mut state = State::new_from_data([
        [0xd4, 0xe0, 0xb8, 0x1e],
        [0xbf, 0xb4, 0x41, 0x27],
        [0x5d, 0x52, 0x11, 0x98],
        [0x30, 0xae, 0xf1, 0xe5],
    ]);

    let expected_state = State::new_from_data([
        [0xd4, 0xe0, 0xb8, 0x1e],
        [0xb4, 0x41, 0x27, 0xbf],
        [0x11, 0x98, 0x5d, 0x52],
        [0xe5, 0x30, 0xae, 0xf1],
    ]);

    state.shift_rows();

    for i in 0..4 {
        assert_eq!(state.data.get_row(i), expected_state.data.get_row(i));
    }
}

#[test]
fn test_inv_shift_rows() {
    let mut state = State::new_from_data([
        [0xd4, 0xe0, 0xb8, 0x1e],
        [0xb4, 0x41, 0x27, 0xbf],
        [0x11, 0x98, 0x5d, 0x52],
        [0xe5, 0x30, 0xae, 0xf1],
    ]);

    let expected_state = State::new_from_data([
        [0xd4, 0xe0, 0xb8, 0x1e],
        [0xbf, 0xb4, 0x41, 0x27],
        [0x5d, 0x52, 0x11, 0x98],
        [0x30, 0xae, 0xf1, 0xe5],
    ]);

    state.inv_shift_rows();

    for i in 0..4 {
        assert_eq!(state.data.get_row(i), expected_state.data.get_row(i));
    }
}

#[test]
fn test_sub_bytes() {
    let mut state = State::new_from_data([
        [0x19, 0xa0, 0x9a, 0xe9],
        [0x3d, 0xf4, 0xc6, 0xf8],
        [0xe3, 0xe2, 0x8d, 0x48],
        [0xbe, 0x2b, 0x2a, 0x08],
    ]);

    let expected_state = State::new_from_data([
        [0xd4, 0xe0, 0xb8, 0x1e],
        [0x27, 0xbf, 0xb4, 0x41],
        [0x11, 0x98, 0x5d, 0x52],
        [0xae, 0xf1, 0xe5, 0x30],
    ]);

    state.sub_bytes();

    for i in 0..4 {
        assert_eq!(state.data.get_row(i), expected_state.data.get_row(i));
    }
}

#[test]
fn test_inv_sub_bytes() {
    let mut state = State::new_from_data([
        [0xd4, 0xe0, 0xb8, 0x1e],
        [0x27, 0xbf, 0xb4, 0x41],
        [0x11, 0x98, 0x5d, 0x52],
        [0xae, 0xf1, 0xe5, 0x30],
    ]);

    let expected_state = State::new_from_data([
        [0x19, 0xa0, 0x9a, 0xe9],
        [0x3d, 0xf4, 0xc6, 0xf8],
        [0xe3, 0xe2, 0x8d, 0x48],
        [0xbe, 0x2b, 0x2a, 0x08],
    ]);

    state.inv_sub_bytes();

    for i in 0..4 {
        assert_eq!(state.data.get_row(i), expected_state.data.get_row(i));
    }
}

#[test]
fn test_get_state_from_data_in() {
    let data_in: [u8; 4 * N_B] = [
        0x32, 0x88, 0x31, 0xe0, 0x43, 0x5a, 0x31, 0x37, 0xf6, 0x30, 0x98, 0x07, 0xa8, 0x8d, 0xa2,
        0x34,
    ];

    let expected_state = State::new_from_data([
        [0x32, 0x43, 0xf6, 0xa8],
        [0x88, 0x5a, 0x30, 0x8d],
        [0x31, 0x31, 0x98, 0xa2],
        [0xe0, 0x37, 0x07, 0x34],
    ]);

    let state = State::new_from_data_in(&data_in);

    for i in 0..4 {
        assert_eq!(state.data.get_row(i), expected_state.data.get_row(i));
    }
}

#[test]
fn test_set_data_out_from_state() {
    let mut data_out: [u8; 4 * N_B] = [0; 4 * N_B];

    let state = State::new_from_data([
        [0x39, 0x02, 0xdc, 0x19],
        [0x25, 0xdc, 0x11, 0x6a],
        [0x84, 0x09, 0x85, 0x0b],
        [0x1d, 0xfb, 0x97, 0x32],
    ]);

    let expected_data_out: [u8; 4 * N_B] = [
        0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b,
        0x32,
    ];

    state.set_data_out(&mut data_out);

    for i in 0..(N_B * 4) {
        assert_eq!(data_out[i], expected_data_out[i]);
    }
}

#[test]
fn test_mix_columns() {
    let mut state = State::new_from_data([
        [0xdb, 0xf2, 0x01, 0xc6],
        [0x13, 0x0a, 0x01, 0xc6],
        [0x53, 0x22, 0x01, 0xc6],
        [0x45, 0x5c, 0x01, 0xc6],
    ]);

    let expected_state = State::new_from_data([
        [0x8e, 0x9f, 0x01, 0xc6],
        [0x4d, 0xdc, 0x01, 0xc6],
        [0xa1, 0x58, 0x01, 0xc6],
        [0xbc, 0x9d, 0x01, 0xc6],
    ]);

    state.mix_columns();

    for i in 0..4 {
        assert_eq!(state.data.get_row(i), expected_state.data.get_row(i));
    }
}

#[test]
fn test_cipher() {
    let plain_bytes: [u8; 4 * N_B] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ];

    let cipher_key: [u8; 4 * N_K as usize] = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ];

    let expected_cipher_bytes: [u8; 4 * N_B] = [
        0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b,
        0x32,
    ];

    let cipher = AESBlockCipher::new(cipher_key);

    let cipher_bytes = cipher.cipher_block(&plain_bytes);

    for i in 0..(N_B * 4) {
        assert_eq!(cipher_bytes[i], expected_cipher_bytes[i]);
    }
}

#[test]
fn test_cipher_using_new_u128() {
    let plain_bytes: [u8; 4 * N_B] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ];

    let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

    let expected_cipher_bytes: [u8; 4 * N_B] = [
        0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b,
        0x32,
    ];

    let cipher = AESBlockCipher::new_u128(cipher_key);

    let block = cipher.cipher_block(&plain_bytes);

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
        [0xbc, 0x9d, 0x01, 0xc6],
    ]);

    let expected_state = State::new_from_data([
        [0xdb, 0xf2, 0x01, 0xc6],
        [0x13, 0x0a, 0x01, 0xc6],
        [0x53, 0x22, 0x01, 0xc6],
        [0x45, 0x5c, 0x01, 0xc6],
    ]);

    state.inv_mix_columns();

    for i in 0..4 {
        assert_eq!(state.data.get_row(i), expected_state.data.get_row(i));
    }
}

#[test]
fn test_inv_cipher() {
    let expected_plain_text: [u8; 4 * N_B] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ];

    let cipher_key: [u8; 4 * N_K as usize] = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ];

    let cipher_bytes: [u8; 4 * N_B] = [
        0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b,
        0x32,
    ];

    let cipher = AESBlockCipher::new(cipher_key);

    let plain_text = cipher.inv_cipher_block(&cipher_bytes);

    for i in 0..(N_B * 4) {
        assert_eq!(plain_text[i], expected_plain_text[i]);
    }
}
