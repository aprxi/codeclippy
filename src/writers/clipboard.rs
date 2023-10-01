use std::io::{self, Write};

use arboard::Clipboard;

use super::ClippyWriter;

pub struct ClipboardWriter {
    clipboard: Clipboard,
}

impl ClipboardWriter {
    pub fn new() -> Self {
        let mut clipboard = Clipboard::new().unwrap();
        // Clear the clipboard content
        clipboard.set_text(String::new()).unwrap();
        Self { clipboard }
    }
}

impl ClippyWriter for ClipboardWriter {
    fn write_line(&mut self, s: &str) -> io::Result<()> {
        let mut current_text = self.clipboard.get_text().unwrap_or_default();
        current_text.push_str(s);
        current_text.push('\n');
        self.clipboard
            .set_text(current_text)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

impl Write for ClipboardWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut current_text = self.clipboard.get_text().unwrap_or_default();
        let new_text = String::from_utf8_lossy(buf);
        current_text.push_str(&new_text);
        self.clipboard
            .set_text(current_text)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
