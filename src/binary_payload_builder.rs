use rust_extensions::AsSliceOrVec;

pub struct BinaryPayloadBuilder {
    data: Vec<u8>,
}

impl BinaryPayloadBuilder {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn write_i8(&mut self, value: i8) {
        self.data.push(value as u8);
    }

    pub fn write_u16(&mut self, value: u16) {
        self.data.extend_from_slice(value.to_le_bytes().as_slice());
    }

    pub fn write_i16(&mut self, value: i16) {
        self.data.extend_from_slice(value.to_le_bytes().as_slice());
    }

    pub fn write_u32(&mut self, value: u32) {
        self.data.extend_from_slice(value.to_le_bytes().as_slice());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.data.extend_from_slice(value.to_le_bytes().as_slice());
    }

    pub fn write_u64(&mut self, value: u64) {
        self.data.extend_from_slice(value.to_le_bytes().as_slice());
    }

    pub fn write_i64(&mut self, value: i64) {
        self.data.extend_from_slice(value.to_le_bytes().as_slice());
    }
}

impl<'s> Into<AsSliceOrVec<'s, u8>> for BinaryPayloadBuilder {
    fn into(self) -> AsSliceOrVec<'s, u8> {
        AsSliceOrVec::AsVec(self.data)
    }
}
