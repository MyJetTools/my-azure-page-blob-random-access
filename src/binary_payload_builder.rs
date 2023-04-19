use rust_extensions::AsSliceOrVec;

pub struct BinaryPayloadBuilder<'s> {
    data: &'s mut [u8],
    offset: usize,
}

impl<'s> BinaryPayloadBuilder<'s> {
    pub fn new(data: &'s mut [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn write_u8(&mut self, value: u8) {
        let value_to_write = self.data.get_mut(self.offset).unwrap();
        *value_to_write = value;
        self.offset += 1;
    }

    pub fn write_i8(&mut self, value: i8) {
        let value_to_write = self.data.get_mut(self.offset).unwrap();
        *value_to_write = value as u8;
        self.offset += 1;
    }

    pub fn write_u16(&mut self, value: u16) {
        const SIZE: usize = 2;
        let value_to_write = &mut self.data[self.offset..self.offset + SIZE];
        value_to_write.copy_from_slice(value.to_le_bytes().as_slice());
        self.offset += SIZE;
    }

    pub fn write_i16(&mut self, value: i16) {
        const SIZE: usize = 2;
        let value_to_write = &mut self.data[self.offset..self.offset + SIZE];
        value_to_write.copy_from_slice(value.to_le_bytes().as_slice());
        self.offset += SIZE;
    }

    pub fn write_u32(&mut self, value: u32) {
        const SIZE: usize = 4;
        let value_to_write = &mut self.data[self.offset..self.offset + SIZE];
        value_to_write.copy_from_slice(value.to_le_bytes().as_slice());
        self.offset += SIZE;
    }

    pub fn write_i32(&mut self, value: i32) {
        const SIZE: usize = 4;
        let value_to_write = &mut self.data[self.offset..self.offset + SIZE];
        value_to_write.copy_from_slice(value.to_le_bytes().as_slice());
        self.offset += SIZE;
    }

    pub fn write_u64(&mut self, value: u64) {
        const SIZE: usize = 8;
        let value_to_write = &mut self.data[self.offset..self.offset + SIZE];
        value_to_write.copy_from_slice(value.to_le_bytes().as_slice());
        self.offset += SIZE;
    }

    pub fn write_i64(&mut self, value: i64) {
        const SIZE: usize = 8;
        let value_to_write = &mut self.data[self.offset..self.offset + SIZE];
        value_to_write.copy_from_slice(value.to_le_bytes().as_slice());
        self.offset += SIZE;
    }
}

impl<'s> Into<AsSliceOrVec<'s, u8>> for BinaryPayloadBuilder<'s> {
    fn into(self) -> AsSliceOrVec<'s, u8> {
        AsSliceOrVec::AsSlice(self.data)
    }
}
