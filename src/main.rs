pub mod aes_block_cipher;
mod metrics_logger;
mod utils;

mod aes_cipher;

use crate::aes_cipher::AESCipher;
use crate::metrics_logger::{MetricsLogger, StatsDMetricsLogger};
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 10000000;

fn main() -> Result<(), String> {
    let n_threads = std::env::var("N_THREADS")
        .unwrap_or("1".to_string())
        .parse()
        .expect("Error while parsing N_THREADS");

    println!("N_THREADS: {}", n_threads);

    let input_file = "test_files/lorem_ipsum.txt";
    let output_file = "test_files/output.txt";
    let decrypted_file = "test_files/decrypted.txt";

    let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

    println!("Cipher key: {:x}", cipher_key);

    std::io::stdout().flush().unwrap();

    let mut cipher = AESCipher::new(cipher_key, n_threads)?;

    let start_time = std::time::Instant::now();

    match cipher.cipher_file(input_file, output_file) {
        Ok(_) => {}
        Err(e) => {
            println!("Error while encrypting file: {}", e);
            std::process::exit(1);
        }
    }

    match cipher.decipher_file(output_file, decrypted_file) {
        Ok(_) => {}
        Err(e) => {
            println!("Error while decrypting file: {}", e);
            std::process::exit(1);
        }
    }

    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("Elapsed time: {}s", elapsed_time);

    match compare_files(input_file, decrypted_file) {
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

    if std::env::var("LOCAL")
        .unwrap_or("false".to_string())
        .as_str()
        == "true"
    {
        let logger = StatsDMetricsLogger::new("graphite:8125", "aes_cipher");
        logger.gauge("completion_time", elapsed_time);
    }
    Ok(())
}

fn compare_files(file1: &str, file2: &str) -> std::io::Result<bool> {
    let file1 = std::fs::File::open(file1)?;
    let file2 = std::fs::File::open(file2)?;

    let mut reader1 = std::io::BufReader::new(file1);
    let mut reader2 = std::io::BufReader::new(file2);

    let mut buffer1 = vec![0u8; BUFFER_SIZE];
    let mut buffer2 = vec![0u8; BUFFER_SIZE];

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
