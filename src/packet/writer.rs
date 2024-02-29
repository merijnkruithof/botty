use bytes::{BufMut, BytesMut};

pub struct Writer<'a> {
    buffer: &'a mut BytesMut,
}

impl<'a> Writer<'a> {
    pub fn new(buffer: &'a mut BytesMut) -> Self {
        // reserve space for the packet length
        buffer.put_u32(0);

        Writer { buffer }
    }

    pub fn write_uint16(&mut self, data: u16) {
        self.buffer.put_u16(data);

        self.adjust_buffer_len();
    }

    pub fn write_uint32(&mut self, data: u32) {
        self.buffer.put_u32(data);

        self.adjust_buffer_len();
    }

    pub fn write_string(&mut self, data: &str) {
        self.buffer.put_u16(data.len() as u16);
        self.buffer.put_slice(data.as_bytes());

        self.adjust_buffer_len();
    }

    fn adjust_buffer_len(&mut self) {
        let data_len = (self.buffer.len() - 4) as u32;

        self.buffer[..4].copy_from_slice(&data_len.to_be_bytes());
    }
}
