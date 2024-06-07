pub mod aes_block_cipher;
mod metrics_logger;
mod utils;

mod config;

mod aes_cipher;

use crate::aes_cipher::AESCipher;
use crate::metrics_logger::{MetricsLogger, StatsDMetricsLogger};

const BUFFER_SIZE: usize = 8192;
const COMPLETION_TIME_METRIC_NAME: &str = "completion_time";

fn main() -> Result<(), String> {
    dotenv::dotenv().ok();

    let config = config::Config::new_from_env();
    println!("Starting program with the following configuration:\n{:?}", config);

    let cipher_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;
    let mut cipher = AESCipher::new(cipher_key, config.n_threads)?;

   let start_time = std::time::Instant::now();

    (0..config.repeat).for_each(|_| {
        match run_iteration(&mut cipher, &config) {
            Ok(_) => {}
            Err(e) => {
                println!("Error while encrypting/decrypting file: {}", e);
                std::process::exit(1);
            }
        };
    });

    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("Elapsed time: {}s", elapsed_time);


    if config.publish_metrics
    {
        let logger: StatsDMetricsLogger = Default::default();
        logger.gauge(COMPLETION_TIME_METRIC_NAME, elapsed_time);
    }
    Ok(())
}

fn run_iteration(cipher: &mut AESCipher, config: &config::Config) -> std::io::Result<()> {
    match (&config.input_file, &config.encrypted_file, &config.decrypted_file) {
        (Some(input_file), Some(encrypted_file), Some(decrypted_file)) => {
            cipher.cipher_file(input_file.as_str(), encrypted_file.as_str())?;
            cipher.decipher_file(encrypted_file.as_str(), decrypted_file.as_str())?;
        }
        (Some(input_file), Some(encrypted_file), None) => {
            cipher.cipher_file(input_file.as_str(), encrypted_file.as_str())?;
        }
        (None, Some(encrypted_file), Some(decrypted_file)) => {
            cipher.decipher_file(encrypted_file.as_str(), decrypted_file.as_str())?;
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid input",
            ));
        }
    }
    Ok(())
}