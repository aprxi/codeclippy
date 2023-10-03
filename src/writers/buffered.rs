use std::io::{self, Write};

use super::ClippyWriter;

pub struct BufferedWriter {
    buffer: Vec<u8>,
}

impl BufferedWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl Write for BufferedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // No-op since we're not writing to an underlying writer
        Ok(())
    }
}

impl ClippyWriter for BufferedWriter {
    fn write_line(&mut self, s: &str) -> io::Result<()> {
        self.write(s.as_bytes())?;
        self.write(b"\n")?;
        Ok(())
    }
    fn get_buffer(&self) -> Option<&Vec<u8>> {
        if self.buffer.is_empty() {
            return None;
        }
        Some(&self.buffer)
    }
}
