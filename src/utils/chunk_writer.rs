use std::io::{BufWriter, Write};

pub struct ChunkWriter<T>
where
    T: Write,
{
    output: BufWriter<T>,
    remove_padding: bool,
}

impl<T> ChunkWriter<T>
where
    T: Write,
{
    pub fn new(output: T, remove_padding: bool) -> Self {
        ChunkWriter {
            output: BufWriter::new(output),
            remove_padding,
        }
    }

    /// Write the chunks to the output, removing any null padding if `remove_padding` is set to
    /// `true`.
    /// Return `Ok(())` if the write operation is successful, or an error if it fails to write
    /// any of the chunks.
    pub fn write_chunks(&mut self, chunks: &[[u8; 16]]) -> std::io::Result<()> {
        for chunk in chunks {
            self.write_chunk(chunk)?;
        }
        Ok(())
    }

    /// Write the chunk to the output, removing any null padding if `remove_padding` is set to
    /// `true`.
    /// Return `Ok(())` if the write operation is successful, or an error if it fails to write
    /// the chunk.
    fn write_chunk(&mut self, chunk: &[u8; 16]) -> std::io::Result<()> {
        if self.remove_padding {
            self.write_chunk_without_padding(chunk)
        } else {
            self.output.write_all(chunk)
        }
    }

    fn write_chunk_without_padding(&mut self, chunk: &[u8; 16]) -> std::io::Result<()> {
        let padding_pos = chunk.iter().position(|&byte| byte == 0).unwrap_or(16);
        self.output.write_all(&chunk[..padding_pos])
    }
}

impl<T> Drop for ChunkWriter<T>
where
    T: Write,
{
    fn drop(&mut self) {
        self.output.flush().expect("Failed to flush the output");
    }
}
