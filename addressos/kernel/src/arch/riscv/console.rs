use core::fmt::{self, Write};

pub(crate) fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

#[inline]
pub(crate) fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}