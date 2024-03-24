use bytes::{BufMut, BytesMut};

pub struct Reader {
    buffer: Vec<u8>,
    position: usize,
}

impl Reader {
    pub fn new(buffer: Vec<u8>) -> Self {
        Reader {
            buffer,

            // +4, because I ignore the packet size.
            position: 4,
        }
    }

    pub fn read_uint16(&mut self) -> Option<u16> {
        if self.position + 2 <= self.buffer.len() {
            let result =
                u16::from_be_bytes([self.buffer[self.position], self.buffer[self.position + 1]]);

            self.position += 2;

            Some(result)
        } else {
            None
        }
    }

    pub fn read_uint32(&mut self) -> Option<u32> {
        if self.position + 4 <= self.buffer.len() {
            let result = u32::from_be_bytes([
                self.buffer[self.position],
                self.buffer[self.position + 1],
                self.buffer[self.position + 2],
                self.buffer[self.position + 3]
            ]);

            self.position += 4;

            Some(result)
        } else {
            None
        }
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        if self.position + 1 <= self.buffer.len() {
            let result = self.buffer[self.position + 1];

            self.position += 1;

            Some(result == 1)
        } else {
            None
        }
    }
}

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
