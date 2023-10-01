mod clipboard;
mod stdout;

use std::io::{self, Write};

pub use clipboard::ClipboardWriter;
pub use stdout::StdoutWriter;

pub trait ClippyWriter: Write {
    fn write_line(&mut self, s: &str) -> io::Result<()>;
}
