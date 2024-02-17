use crate::aes_cipher::{AESCipher, N_B};
use crate::metrics_logger::MetricsLogger;

pub struct MeasuredAESCipher<T>
    where
        T: MetricsLogger,
{
    cipher: AESCipher,
    logger: T,
}

impl<T> MeasuredAESCipher<T>
    where
        T: MetricsLogger,
{
    pub fn new(cipher: AESCipher, logger: T) -> Self {
        MeasuredAESCipher { cipher, logger }
    }

    pub fn cipher_block(&self, data_in: [u8; 4 * N_B]) -> [u8; 4 * N_B] {
        self.logger.run_and_measure("cipher_block_duration", || {
            self.cipher.cipher_block(data_in)
        })
    }

    pub fn inv_cipher_block(&self, data_in: [u8; 4 * N_B]) -> [u8; 4 * N_B] {
        self.logger.run_and_measure("inv_cipher_block_duration", || {
            self.cipher.inv_cipher_block(data_in)
        })
    }
}