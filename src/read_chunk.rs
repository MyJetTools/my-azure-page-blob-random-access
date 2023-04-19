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

    pub fn read_u8(&mut self) -> u8 {
        let result = self.data[self.offset];
        self.offset += 1;

        result
    }

    pub fn read_i8(&mut self) -> i8 {
        let result = self.data[self.offset] as i8;
        self.offset += 1;

        result
    }

    pub fn read_u16(&mut self) -> u16 {
        const SIZE: usize = 2;
        let mut result = [0u8; SIZE];

        result.copy_from_slice(&self.data[self.offset..self.offset + SIZE]);

        self.offset += SIZE;

        u16::from_le_bytes(result)
    }

    pub fn read_i16(&mut self) -> i16 {
        const SIZE: usize = 2;
        let mut result = [0u8; SIZE];

        result.copy_from_slice(&self.data[self.offset..self.offset + SIZE]);

        self.offset += SIZE;

        i16::from_le_bytes(result)
    }

    pub fn read_u32(&mut self) -> u32 {
        const SIZE: usize = 4;
        let mut result = [0u8; SIZE];

        result.copy_from_slice(&self.data[self.offset..self.offset + SIZE]);

        self.offset += SIZE;

        u32::from_le_bytes(result)
    }

    pub fn read_i32(&mut self) -> i32 {
        const SIZE: usize = 4;
        let mut result = [0u8; SIZE];

        result.copy_from_slice(&self.data[self.offset..self.offset + SIZE]);

        self.offset += SIZE;

        i32::from_le_bytes(result)
    }

    pub fn read_u64(&mut self) -> u64 {
        const SIZE: usize = 8;
        let mut result = [0u8; SIZE];

        result.copy_from_slice(&self.data[self.offset..self.offset + SIZE]);

        self.offset += SIZE;

        u64::from_le_bytes(result)
    }

    pub fn read_i64(&mut self) -> i64 {
        const SIZE: usize = 8;
        let mut result = [0u8; SIZE];

        result.copy_from_slice(&self.data[self.offset..self.offset + SIZE]);

        self.offset += SIZE;

        i64::from_le_bytes(result)
    }
}
