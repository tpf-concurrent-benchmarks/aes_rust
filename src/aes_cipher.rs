use crate::aes_block_cipher::{AESBlockCipher, N_B};
use crate::utils::{ChunkReader, ChunkWriter};
use crate::BUFFER_SIZE;
use rayon::prelude::*;

pub struct AESCipher {
    block_cipher: AESBlockCipher,
    thread_pool: rayon::ThreadPool,
    buffer: Vec<[u8; 4 * N_B]>,
}

impl AESCipher {
    pub fn new(cipher_key: u128, n_threads: usize) -> Result<Self, String> {
        let block_cipher = AESBlockCipher::new_u128(cipher_key);
        let thread_pool = Self::create_thread_pool(n_threads)?;
        let ret = Self {
            block_cipher,
            thread_pool,
            buffer: vec![[0; 4 * N_B]; BUFFER_SIZE],
        };
        Ok(ret)
    }

    fn create_thread_pool(n_threads: usize) -> Result<rayon::ThreadPool, String> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(n_threads)
            .build()
            .map_err(|e| format!("Error while creating thread pool: {}", e))
    }

    fn cipher_blocks(&self, chunks: &[[u8; 4 * N_B]]) -> Vec<[u8; 4 * N_B]> {
        self.thread_pool.install(|| {
            chunks
                .par_iter()
                .map(|block| self.block_cipher.cipher_block(block))
                .collect::<Vec<_>>()
        })
    }

    fn decipher_blocks(&self, chunks: &[[u8; 4 * N_B]]) -> Vec<[u8; 4 * N_B]> {
        self.thread_pool.install(|| {
            chunks
                .par_iter()
                .map(|block| self.block_cipher.inv_cipher_block(block))
                .collect::<Vec<_>>()
        })
    }

    pub fn cipher<R, W>(&mut self, input: R, output: W) -> std::io::Result<()>
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        let mut chunk_reader = ChunkReader::new(input, 4 * N_B, true);
        let mut chunk_writer = ChunkWriter::new(output, false);

        loop {
            let chunks_filled = chunk_reader.read_chunks(BUFFER_SIZE, &mut self.buffer)?;

            if chunks_filled == 0 {
                break;
            }

            let ciphered_chunks = self.cipher_blocks(&self.buffer[..chunks_filled]);

            chunk_writer.write_chunks(&ciphered_chunks)?;
        }

        Ok(())
    }

    pub fn decipher<R, W>(&mut self, input: R, output: W) -> std::io::Result<()>
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        let mut chunk_reader = ChunkReader::new(input, 4 * N_B, false);
        let mut chunk_writer = ChunkWriter::new(output, true);

        loop {
            let chunks_filled = chunk_reader.read_chunks(BUFFER_SIZE, &mut self.buffer)?;

            if chunks_filled == 0 {
                break;
            }

            let deciphered_chunks = self.decipher_blocks(&self.buffer[..chunks_filled]);

            chunk_writer.write_chunks(&deciphered_chunks)?;
        }

        Ok(())
    }

    pub fn cipher_file(
        &mut self,
        input_file: &str,
        output_file: &str,
    ) -> Result<(), std::io::Error> {
        let input = std::fs::File::open(input_file)?;
        let output = std::fs::File::create(output_file)?;
        self.cipher(input, output)
    }

    pub fn decipher_file(
        &mut self,
        input_file: &str,
        output_file: &str,
    ) -> Result<(), std::io::Error> {
        let input = std::fs::File::open(input_file)?;
        let output = std::fs::File::create(output_file)?;
        self.decipher(input, output)
    }
}
