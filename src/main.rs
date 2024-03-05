pub mod aes_block_cipher;
mod metrics_logger;
mod utils;


use std::io::Read;
use rayon::prelude::*;
use crate::aes_block_cipher::{AESBlockCipher, N_B};
use crate::metrics_logger::{MetricsLogger, StatsDMetricsLogger};
use crate::utils::{ChunkReader, ChunkWriter};

const BUFFER_SIZE: usize = 100;

fn main() {
    let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

    let cipher = AESBlockCipher::new_u128(cipher_key);

    let start_time = std::time::Instant::now();

    match encrypt_file(&cipher, "test_files/input.txt", "test_files/output.txt") {
        Ok(_) => {}
        Err(e) => {
            println!("Error while encrypting file: {}", e);
            std::process::exit(1);
        }
    }

    match decrypt_file(&cipher, "test_files/output.txt", "test_files/decrypted.txt") {
        Ok(_) => {}
        Err(e) => {
            println!("Error while decrypting file: {}", e);
            std::process::exit(1);
        }
    }

    let elapsed_time = start_time.elapsed().as_secs_f64();

    match compare_files("test_files/input.txt", "test_files/decrypted.txt") {
        Ok(true) => {
            println!("Test passed")
        }
        Ok(false) => {
            println!("Test failed");
        }
        Err(e) => {
            println!("Error while comparing files: {}", e);
            std::process::exit(1);
        }
    }

    println!("Elapsed time: {}s", elapsed_time);

    if std::env::var("LOCAL").unwrap_or("false".to_string()).as_str() == "true" {
        let logger = StatsDMetricsLogger::new("graphite:8125", "aes_cipher");
        logger.gauge("completion_time", elapsed_time);
    }
}

fn encrypt_chunks(cipher: &AESBlockCipher, chunks: &[[u8; 4 * N_B]]) -> Vec<[u8; 4 * N_B]> {
    chunks.par_iter().map(|block| cipher.cipher_block(block)).collect::<Vec<_>>()
}

fn decrypt_chunks(cipher: &AESBlockCipher, chunks: &[[u8; 4 * N_B]]) -> Vec<[u8; 4 * N_B]> {
    chunks.par_iter().map(|block| cipher.inv_cipher_block(block)).collect::<Vec<_>>()
}

fn encrypt_file(cipher: &AESBlockCipher, input_file: &str, output_file: &str) -> std::io::Result<()> {
    let input = std::fs::File::open(input_file)?;
    let mut reader = ChunkReader::new(input, 16, true);

    let output = std::fs::File::create(output_file)?;
    let mut writer = ChunkWriter::new(output);

    let mut buffer = [[0u8; 16]; BUFFER_SIZE];

    loop {
        let chunks_filled = reader.read_chunks(BUFFER_SIZE, &mut buffer).unwrap();
        if chunks_filled == 0 {
            break;
        }
        let ciphered_chunks = encrypt_chunks(cipher, &buffer[..chunks_filled]);
        writer.write_chunks(false, &ciphered_chunks).unwrap();
    }

    Ok(())
}

fn decrypt_file(cipher: &AESBlockCipher, input_file: &str, output_file: &str) -> std::io::Result<()> {
    let input = std::fs::File::open(input_file)?;
    let mut reader = ChunkReader::new(input, 16, false);

    let output = std::fs::File::create(output_file)?;
    let mut writer = ChunkWriter::new(output);

    let mut buffer = [[0u8; 16]; BUFFER_SIZE];

    loop {
        let chunks_filled = reader.read_chunks(BUFFER_SIZE, &mut buffer).unwrap();
        if chunks_filled == 0 {
            break;
        }
        let ciphered_chunks = decrypt_chunks(cipher, &buffer[..chunks_filled]);
        writer.write_chunks(true, &ciphered_chunks).unwrap();
    }

    Ok(())
}

fn compare_files(file1: &str, file2: &str) -> std::io::Result<bool> {
    let file1 = std::fs::File::open(file1)?;
    let file2 = std::fs::File::open(file2)?;

    let mut reader1 = std::io::BufReader::new(file1);
    let mut reader2 = std::io::BufReader::new(file2);

    let mut buffer1 = [0u8; BUFFER_SIZE];
    let mut buffer2 = [0u8; BUFFER_SIZE];

    loop {
        let bytes_read1 = reader1.read(&mut buffer1)?;
        let bytes_read2 = reader2.read(&mut buffer2)?;

        if bytes_read1 != bytes_read2 {
            return Ok(false);
        }

        if bytes_read1 == 0 {
            break;
        }

        if buffer1[..bytes_read1] != buffer2[..bytes_read2] {
            return Ok(false);
        }
    }
    Ok(true)
}

