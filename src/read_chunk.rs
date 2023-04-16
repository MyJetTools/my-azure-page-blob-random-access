pub struct ReadChunk {
    pub offset: usize,
    pub length: usize,
    data: Vec<u8>,
}

impl ReadChunk {
    pub fn new(offset: usize, length: usize, data: Vec<u8>) -> Self {
        Self {
            offset,
            length,
            data,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.offset..self.offset + self.length]
    }
}
