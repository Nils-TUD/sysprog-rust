pub mod ports;
pub mod serial;
pub mod vga;

use core::fmt;

/// The fmt::Write implementation for println that writes to the screen and serial line
pub struct Printer {}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let vga = crate::io::vga::VGA_INST.get_mut();
        for b in s.as_bytes() {
            vga.write_char(*b);
            serial::write_char(*b);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        core::fmt::write(&mut crate::io::Printer {}, format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! println {
    ()                       => ($crate::print!("\n"));
    ($fmt:expr)              => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}
