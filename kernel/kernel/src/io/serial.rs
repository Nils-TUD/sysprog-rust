use crate::io::ports::{self, Port};

const COM1: Port = 0x3F8;

const DLR_LO: Port = 0;
const DLR_HI: Port = 1;
const IER: Port = 1; // interrupt enable register
const FCR: Port = 2; // FIFO control register
const LCR: Port = 3; // line control register
const MCR: Port = 4; // modem control register

/// Inits the serial line
pub fn init() {
    unsafe {
        ports::write_byte(COM1 + LCR, 0x80); // Enable DLAB (set baud rate divisor)
        ports::write_byte(COM1 + DLR_LO, 0x01); // Set divisor to 1 (lo byte) 115200 baud
        ports::write_byte(COM1 + DLR_HI, 0x00); //                  (hi byte)
        ports::write_byte(COM1 + LCR, 0x03); // 8 bits, no parity, one stop bit
        ports::write_byte(COM1 + IER, 0); // disable interrupts
        ports::write_byte(COM1 + FCR, 7);
        ports::write_byte(COM1 + MCR, 3);
    }
}

/// Writes the given character to the serial line
pub fn write_char(c: u8) {
    if c == b'\0' {
        return;
    }

    if c == b'\n' {
        write_char(b'\r');
    }

    loop {
        if (unsafe { ports::read_byte(COM1 + 5) } & 0x20) != 0 {
            break;
        }
    }

    unsafe {
        ports::write_byte(COM1, c);
    }
}
