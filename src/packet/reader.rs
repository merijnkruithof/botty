pub struct PacketReader<'a> {
    buffer: &'a Vec<u8>,
    position: usize,
}

impl<'a> PacketReader<'a> {
    pub fn new(buffer: &'a Vec<u8>) -> Self {
        PacketReader {
            buffer,

            // let's ignore the packet size for now. it's really a bad practice, because it could be used
            // for serialization. but eh, fuck it.
            position: 4,
        }
    }

    pub fn read_uint16(&mut self) -> Option<u16> {
        if self.position + 2 < self.buffer.len() {
            let result =
                u16::from_be_bytes([self.buffer[self.position], self.buffer[self.position + 1]]);

            self.position += 2;

            Some(result)
        } else {
            None
        }
    }
}
