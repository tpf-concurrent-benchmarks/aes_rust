pub mod aes_cipher;
mod metrics_logger;
mod matrix;
mod measured_aes_cipher;
mod statsd_metrics_logger;
mod noop_metrics_logger;

use rayon::prelude::*;
use crate::aes_cipher::{AESCipher, N_B};
use crate::measured_aes_cipher::MeasuredAESCipher;
use crate::noop_metrics_logger::NoOpMetricsLogger;
use crate::statsd_metrics_logger::StatsDMetricsLogger;

use metrics_logger::MetricsLogger;

fn main() {
    std::thread::sleep(std::time::Duration::from_secs(10));

    let blocks_to_encrypt = 1000000;
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

    if std::env::var("LOCAL").is_ok() {
        let logger = NoOpMetricsLogger::new();
        let measured_cipher = MeasuredAESCipher::new(cipher, &logger);

        let result = apply_operations_and_compare(&measured_cipher, blocks);
        assert!(result);

        println!("Test passed (local)");

        let elapsed_time = start_time.elapsed().as_secs_f64();
        println!("Elapsed time: {}s", elapsed_time);
        logger.gauge("completion_time", elapsed_time);
    } else {
        let logger = StatsDMetricsLogger::new("graphite:8125", "aes_cipher");
        let measured_cipher = MeasuredAESCipher::new(cipher, &logger);

        let result = apply_operations_and_compare(&measured_cipher, blocks);
        assert!(result);

        println!("Test passed (prod)");

        let elapsed_time = start_time.elapsed().as_secs_f64();
        println!("Elapsed time: {}s", elapsed_time);
        logger.gauge("completion_time", elapsed_time);
    }
}

fn apply_operations_and_compare<T>(cipher: &MeasuredAESCipher<T>, blocks: Vec<[u8; 4 * N_B]>) -> bool
    where
        T: MetricsLogger + Sync,
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