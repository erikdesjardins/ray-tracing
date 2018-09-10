use std::io::{self, Write};

pub struct Writer<W: Write> {
    writer: W,
    written: u64,
    expected: u64,
}

impl<W: Write> Writer<W> {
    pub fn new(mut writer: W, x: u64, y: u64) -> Result<Self, io::Error> {
        writeln!(writer, "P3")?; // colors are in ASCII
        writeln!(writer, "{} {}", x, y)?; // columns and rows
        writeln!(writer, "255")?; // max color
        Ok(Self {
            writer,
            written: 0,
            expected: x * y,
        })
    }

    pub fn write_pixel(&mut self, r: u8, g: u8, b: u8) -> Result<(), io::Error> {
        writeln!(self.writer, "{} {} {}", r, g, b)?;
        self.written += 1;
        assert!(
            self.written <= self.expected,
            "too many pixels were written"
        );
        Ok(())
    }
}

impl<W: Write> Drop for Writer<W> {
    fn drop(&mut self) {
        assert_eq!(
            self.written, self.expected,
            "correct number of pixels were written"
        );
    }
}
