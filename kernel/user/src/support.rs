use crate::syscall1;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        crate::print!(
            "PANIC at {}, line {}, column {}: ",
            loc.file(),
            loc.line(),
            loc.column()
        );
    }
    else {
        crate::print!("PANIC at unknown location: ");
    }

    if let Some(msg) = info.message() {
        crate::print!("{}", *msg);
    }
    crate::println!("");

    loop {}
}

use core::fmt;

pub struct Printer {}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.as_bytes() {
            syscall1(0, *b as usize);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        core::fmt::write(&mut crate::support::Printer {}, format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! println {
    ()                       => ($crate::print!("\n"));
    ($fmt:expr)              => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}
