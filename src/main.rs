pub mod aes_cipher;
mod metrics_logger;
mod matrix;
mod statsd_metrics_logger;

mod chunk_reader;
mod chunk_writer;

use rayon::prelude::*;
use crate::aes_cipher::{AESCipher, N_B};
use crate::metrics_logger::MetricsLogger;
use crate::statsd_metrics_logger::StatsDMetricsLogger;

const BUFFER_SIZE: usize = 100;

fn main() {
    let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

    let cipher = AESCipher::new_u128(cipher_key);

    let start_time = std::time::Instant::now();

    match encrypt_file(&cipher, "input.txt", "output.txt") {
        Ok(_) => {}
        Err(e) => {
            println!("Error while encrypting file: {}", e);
            std::process::exit(1);
        }
    }

    let elapsed_time = start_time.elapsed().as_secs_f64();


    println!("Test passed");
    println!("Elapsed time: {}s", elapsed_time);

    if std::env::var("LOCAL").unwrap_or("false".to_string()).as_str() == "true" {
        let logger = StatsDMetricsLogger::new("graphite:8125", "aes_cipher");
        logger.gauge("completion_time", elapsed_time);
    }
}

fn encrypt_chunks(cipher: &AESCipher, chunks: &[[u8; 4 * N_B]]) -> Vec<[u8; 4 * N_B]> {
    chunks.par_iter().map(|block| cipher.cipher_block(block)).collect::<Vec<_>>()
}

fn encrypt_file(cipher: &AESCipher, input_file: &str, output_file: &str) -> std::io::Result<()> {
    let input = std::fs::File::open(input_file)?;
    let mut reader = chunk_reader::ChunkReader::new(input, 16);

    let output = std::fs::File::create(output_file)?;
    let mut writer = chunk_writer::ChunkWriter::new(output);

    let mut buffer = [[0u8; 16]; BUFFER_SIZE];

    loop {
        let chunks_filled = reader.read_chunks(BUFFER_SIZE, &mut buffer).unwrap();
        if chunks_filled == 0 {
            break;
        }
        let ciphered_chunks = encrypt_chunks(cipher, &buffer);
        writer.write_chunks(false, &ciphered_chunks).unwrap();
    }

    Ok(())
}

