use super::*;

#[test]
fn test_rot_word() {
    let word: Word = 0x09cf4f3c;
    let expected_word: Word = 0xcf4f3c09;
    assert_eq!(AESKey::rot_word(word), expected_word);
}

#[test]
fn test_sub_word() {
    let word: Word = 0xcf4f3c09;
    let expected_word: Word = 0x8a84eb01;
    assert_eq!(AESKey::sub_word(word), expected_word);
}

#[test]
fn test_key_expansion_aes_128() {
    let cipher_key: [u8; 4 * N_K as usize] = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ];

    let expected_words: [Word; N_B * (N_R + 1)] = [
        0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0xa0fafe17, 0x88542cb1, 0x23a33939,
        0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
        0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
        0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
        0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
        0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
        0xe13f0cc8, 0xb6630ca6,
    ];

    let key = AESKey::new_direct(cipher_key);

    for i in 0..(N_B * (N_R + 1)) {
        assert_eq!(key.data[i], expected_words[i]);
    }
}
