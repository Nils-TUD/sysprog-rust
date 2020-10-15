//! Programmable Interval Timer

use crate::io::ports::{self, Port};

const FREQ: u32 = 1193182;

const CTRL: Port = 0x43;
const CHAN0DIV: Port = 0x40;

const CTRL_CHAN0: u8 = 0x0;
const CTRL_RWLOHI: u8 = 0x30;
const CTRL_MODE2: u8 = 0x4;
const CTRL_CNTBIN16: u8 = 0x0;

pub fn init() {
    // change timer divisor and enable periodic mode
    let freq = FREQ / 200;
    unsafe {
        ports::write_byte(CTRL, CTRL_CHAN0 | CTRL_RWLOHI | CTRL_MODE2 | CTRL_CNTBIN16);
        ports::write_byte(CHAN0DIV, (freq & 0xFF) as u8);
        ports::write_byte(CHAN0DIV, (freq >> 8) as u8);
    }
}
