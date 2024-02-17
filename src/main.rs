pub mod aes_cipher;
mod matrix;

use crate::aes_cipher::{AESCipher, N_B};

fn main() {
    let plain_text: [u8; 4 * N_B] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ];

    let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

    let cipher = AESCipher::new_u128(cipher_key);

    let cipher_block = cipher.cipher_block(plain_text);

    let plain_block = cipher.inv_cipher_block(cipher_block);

    for i in 0..(N_B * 4) {
        assert_eq!(plain_text[i], plain_block[i]);
    }
    println!("Test passed");
}
