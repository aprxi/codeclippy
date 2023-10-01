use std::io::{self, Write};

use super::ClippyWriter;

pub struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self {
        Self
    }
}

impl ClippyWriter for StdoutWriter {
    fn write_line(&mut self, s: &str) -> io::Result<()> {
        println!("{}", s);
        Ok(())
    }
}

impl Write for StdoutWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        print!("{}", s); // Print to standard output
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush() // Flush the standard output
    }
}
