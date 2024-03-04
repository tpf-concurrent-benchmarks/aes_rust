pub mod aes_cipher;
mod metrics_logger;
mod matrix;
mod statsd_metrics_logger;

use rayon::prelude::*;
use crate::aes_cipher::{AESCipher, N_B};
use crate::metrics_logger::MetricsLogger;
use crate::statsd_metrics_logger::StatsDMetricsLogger;

fn main() {
    std::thread::sleep(std::time::Duration::from_secs(10));

    let blocks_to_encrypt = 10000;
    let blocks = (0..blocks_to_encrypt).map(|_| {
        let mut block = [0u8; 4 * N_B];
        for i in 0..(4 * N_B) {
            block[i] = rand::random();
        }
        block
    }).collect::<Vec<_>>();

    let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

    let cipher = AESCipher::new_u128(cipher_key);

    let start_time = std::time::Instant::now();

    let result = apply_operations_and_compare(&cipher, blocks);

    let elapsed_time = start_time.elapsed().as_secs_f64();

    assert!(result);

    println!("Test passed");
    println!("Elapsed time: {}s", elapsed_time);

    match std::env::var("LOCAL").unwrap_or("false".to_string()).as_str() {
        "true" => {
            let logger = StatsDMetricsLogger::new("graphite:8125", "aes_cipher");
            logger.gauge("completion_time", elapsed_time);
        }
        _ => {}
    }
}

fn apply_operations_and_compare(cipher: &AESCipher, blocks: Vec<[u8; 4 * N_B]>) -> bool
{
    let ciphered_blocks = blocks.par_iter().map(|block| cipher.cipher_block(block)).collect::<Vec<_>>();

    let deciphered_blocks = ciphered_blocks.par_iter().map(|block| cipher.inv_cipher_block(block)).collect::<Vec<_>>();

    for (original_block, deciphered_block) in blocks.iter().zip(deciphered_blocks.iter()) {
        for i in 0..(N_B * 4) {
            if original_block[i] != deciphered_block[i] {
                return false;
            }
        }
    }
    true
}