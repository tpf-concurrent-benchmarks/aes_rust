use std::io::Read;

pub struct ChunkReader<T>
    where T: Read
{
    input: T,
    chunk_size: usize,
    with_padding: bool,
}

impl<T> ChunkReader<T>
    where T: Read
{
    pub fn new(input: T, chunk_size: usize, with_padding: bool) -> Self {
        ChunkReader {
            input,
            chunk_size,
            with_padding,
        }
    }

    /// Read at most `chunk_size` bytes from the input, and place them in the buffer.
    /// Return the number of chunks filled.
    pub fn read_chunks(&mut self, chunks_amount: usize, buffer: &mut [[u8; 16]]) -> std::io::Result<usize> {
        let mut chunks_filled = 0;
        while chunks_filled < chunks_amount {
            let chunk = &mut buffer[chunks_filled];
            match self.fill_chunk(chunk) {
                Ok(0) => return Ok(chunks_filled),
                Ok(n) => {
                    chunks_filled += 1;
                    if n < self.chunk_size {
                        return Ok(chunks_filled);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(chunks_filled)
    }

    /// Fill the buffer with the next chunk of data from the input.
    /// Returns the number of bytes read, or an error if the read operation fails.
    fn fill_chunk(&mut self, buffer: &mut [u8; 16]) -> std::io::Result<usize> {
        let mut bytes_read = 0;
        while bytes_read < self.chunk_size {
            match self.input.read(&mut buffer[bytes_read..]) {
                Ok(0) => {
                    if self.with_padding {
                        self.apply_null_padding(bytes_read, buffer);
                    }
                    return Ok(bytes_read);
                }
                Ok(n) => bytes_read += n,
                Err(e) => return Err(e),
            }
        }
        Ok(bytes_read)
    }

    fn apply_null_padding(&self, from: usize, buffer: &mut [u8; 16]) {
        (from..self.chunk_size).for_each(|i| buffer[i] = 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn test_read_one_chunk_exact_size() {
        let input = Cursor::new(vec![54u8; 16]);
        let mut reader = ChunkReader::new(input, 16, true);
        let mut buffer = [[0u8; 16]; 1];
        let chunks_filled = reader.read_chunks(1, &mut buffer).unwrap();
        assert_eq!(chunks_filled, 1);
        assert_eq!(buffer[0], [54u8; 16]);
    }

    #[test]
    fn test_read_one_chunk_partial_size() {
        let input = Cursor::new(vec![54u8; 8]);
        let mut reader = ChunkReader::new(input, 16, true);
        let mut buffer = [[0u8; 16]; 1];
        let chunks_filled = reader.read_chunks(1, &mut buffer).unwrap();
        assert_eq!(chunks_filled, 1);
        assert_eq!(buffer[0], [54u8, 54u8, 54u8, 54u8, 54u8, 54u8, 54u8, 54u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
    }

    #[test]
    fn test_read_multiple_chunks_exact_size() {
        let mut vec = vec![0u8; 32];
        (0..16).for_each(|i| vec[i] = 54u8);
        (16..32).for_each(|i| vec[i] = 76u8);

        let input = Cursor::new(vec);
        let mut reader = ChunkReader::new(input, 16, true);
        let mut buffer = [[0u8; 16]; 2];
        let chunks_filled = reader.read_chunks(2, &mut buffer).unwrap();
        assert_eq!(chunks_filled, 2);
        assert_eq!(buffer[0], [54u8; 16]);
        assert_eq!(buffer[1], [76u8; 16]);
    }

    #[test]
    fn test_read_multiple_chunks_partial_size() {
        let mut vec = vec![0u8; 24];
        (0..8).for_each(|i| vec[i] = 54u8);
        (8..16).for_each(|i| vec[i] = 76u8);
        (16..24).for_each(|i| vec[i] = 98u8);

        let input = Cursor::new(vec);
        let mut reader = ChunkReader::new(input, 16, true);
        let mut buffer = [[0u8; 16]; 2];
        let chunks_filled = reader.read_chunks(2, &mut buffer).unwrap();
        assert_eq!(chunks_filled, 2);
        assert_eq!(buffer[0], [54u8, 54u8, 54u8, 54u8, 54u8, 54u8, 54u8, 54u8, 76u8, 76u8, 76u8, 76u8, 76u8, 76u8, 76u8, 76u8]);
        assert_eq!(buffer[1], [98u8, 98u8, 98u8, 98u8, 98u8, 98u8, 98u8, 98u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
    }

    #[test]
    fn test_read_more_than_available() {
        let input = Cursor::new(vec![54u8; 16]);
        let mut reader = ChunkReader::new(input, 16, true);
        let mut buffer = [[0u8; 16]; 2];
        let chunks_filled = reader.read_chunks(2, &mut buffer).unwrap();
        assert_eq!(chunks_filled, 1);
        assert_eq!(buffer[0], [54u8; 16]);
        assert_eq!(buffer[1], [0u8; 16]);
    }
}