use std::io::{self, Write};

use vec::Vec3;

pub struct Writer<W: Write> {
    writer: W,
    written: u64,
    expected: u64,
}

impl<W: Write> Writer<W> {
    pub fn new(mut writer: W, x: u64, y: u64) -> Result<Self, io::Error> {
        // "P3" means colors are in ASCII
        writeln!(writer, "P3")?;
        // columns (x), then rows (y)
        writeln!(writer, "{} {}", x, y)?;
        // max color
        writeln!(writer, "255")?;
        Ok(Self {
            writer,
            written: 0,
            expected: x * y,
        })
    }

    pub fn write_pixel(&mut self, color: &Vec3<u8>) -> Result<(), io::Error> {
        writeln!(self.writer, "{} {} {}", color.x, color.y, color.z)?;
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
