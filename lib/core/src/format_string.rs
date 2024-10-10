use core::{cmp::min, fmt};

pub struct FormatString<'a> {
    buffer: &'a mut [u8],
    used: usize,
}

impl<'a> FormatString<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        FormatString { buffer, used: 0 }
    }

    pub fn as_str(self) -> Option<&'a str> {
        if self.used <= self.buffer.len() {
            use core::str::from_utf8;
            Some(from_utf8(&self.buffer[..self.used]).unwrap())
        } else {
            None
        }
    }
}

impl<'a> fmt::Write for FormatString<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.used > self.buffer.len() {
            return Err(fmt::Error);
        }
        let remaining_buf = &mut self.buffer[self.used..];
        let raw_s = s.as_bytes();
        let write_num = min(raw_s.len(), remaining_buf.len());
        remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
        self.used += raw_s.len();
        if write_num < raw_s.len() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

pub fn show<'a>(buffer: &'a mut [u8; 1024], args: fmt::Arguments) -> Result<&'a str, fmt::Error> {
    let mut w = FormatString::new(buffer);
    fmt::write(&mut w, args)?;
    w.as_str().ok_or(fmt::Error)
}

#[macro_export]
macro_rules! format {
    ($buffer:expr, $($arg:tt)*) => {
        {
            show(&mut $buffer, format_args!($($arg)*))
        }
    };
}
