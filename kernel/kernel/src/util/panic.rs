use crate::{print, println};
use core::panic::PanicInfo;

/// Is called by Rust in case of panics
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        print!(
            "PANIC at {}, line {}, column {}: ",
            loc.file(),
            loc.line(),
            loc.column()
        );
    }
    else {
        print!("PANIC at unknown location: ");
    }

    if let Some(msg) = info.message() {
        print!("{}", *msg);
    }
    println!("");

    loop {}
}
