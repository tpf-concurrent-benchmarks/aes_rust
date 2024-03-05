use crate::aes_block_cipher::{AESBlockCipher, N_B};
use crate::utils::{ChunkReader, ChunkWriter};
use rayon::prelude::*;

const BUFFER_SIZE: usize = 100;

pub struct AESCipher {
    block_cipher: AESBlockCipher,
}

impl AESCipher {
    pub fn new(cipher_key: u128) -> Self {
        let block_cipher = AESBlockCipher::new_u128(cipher_key);
        Self { block_cipher }
    }

    fn cipher_blocks(&self, chunks: &[[u8; 4 * N_B]]) -> Vec<[u8; 4 * N_B]> {
        chunks
            .par_iter()
            .map(|block| self.block_cipher.cipher_block(block))
            .collect::<Vec<_>>()
    }

    fn decipher_blocks(&self, chunks: &[[u8; 4 * N_B]]) -> Vec<[u8; 4 * N_B]> {
        chunks
            .par_iter()
            .map(|block| self.block_cipher.inv_cipher_block(block))
            .collect::<Vec<_>>()
    }

    pub fn cipher<R, W>(&self, input: R, output: W) -> Result<(), std::io::Error>
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        let mut chunk_reader = ChunkReader::new(input, 4 * N_B, true);
        let mut chunk_writer = ChunkWriter::new(output);
        let mut buffer = [[0u8; 16]; BUFFER_SIZE];

        loop {
            let chunks_filled = chunk_reader.read_chunks(BUFFER_SIZE, &mut buffer)?;
            if chunks_filled == 0 {
                break;
            }
            let ciphered_chunks = self.cipher_blocks(&buffer[..chunks_filled]);
            chunk_writer.write_chunks(false, &ciphered_chunks)?;
        }

        Ok(())
    }

    pub fn decipher<R, W>(&self, input: R, output: W) -> Result<(), std::io::Error>
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        let mut chunk_reader = ChunkReader::new(input, 4 * N_B, false);
        let mut chunk_writer = ChunkWriter::new(output);
        let mut buffer = [[0u8; 16]; BUFFER_SIZE];

        loop {
            let chunks_filled = chunk_reader.read_chunks(BUFFER_SIZE, &mut buffer)?;
            if chunks_filled == 0 {
                break;
            }
            let deciphered_chunks = self.decipher_blocks(&buffer[..chunks_filled]);
            chunk_writer.write_chunks(true, &deciphered_chunks)?;
        }

        Ok(())
    }

    pub fn cipher_file(&self, input_file: &str, output_file: &str) -> Result<(), std::io::Error> {
        let input = std::fs::File::open(input_file)?;
        let output = std::fs::File::create(output_file)?;
        self.cipher(input, output)
    }

    pub fn decipher_file(&self, input_file: &str, output_file: &str) -> Result<(), std::io::Error> {
        let input = std::fs::File::open(input_file)?;
        let output = std::fs::File::create(output_file)?;
        self.decipher(input, output)
    }
}
