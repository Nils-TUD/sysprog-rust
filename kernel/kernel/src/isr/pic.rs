//! Programmable Interrupt Controller

use crate::io::ports::{self, Port};

const MASTER: Port = 0x20;
const SLAVE: Port = 0xA0;

const MASTER_CMD: Port = MASTER;
const MASTER_DATA: Port = MASTER + 1;

const SLAVE_CMD: Port = SLAVE;
const SLAVE_DATA: Port = SLAVE + 1;

const EOI: u8 = 0x20;

const ICW1_NEED_ICW4: u8 = 0x01;
const ICW1_INIT: u8 = 0x10;
const ICW4_8086: u8 = 0x01;

const REMAP_MASTER: u8 = 0x20;
const REMAP_SLAVE: u8 = 0x28;

/// Initializes the PIC
pub fn init() {
    unsafe {
        // starts the initialization. we want to send a ICW4
        ports::write_byte(MASTER_CMD, ICW1_INIT | ICW1_NEED_ICW4);
        ports::write_byte(SLAVE_CMD, ICW1_INIT | ICW1_NEED_ICW4);
        // remap the irqs to 0x20 and 0x28
        ports::write_byte(MASTER_DATA, REMAP_MASTER);
        ports::write_byte(SLAVE_DATA, REMAP_SLAVE);
        // continue
        ports::write_byte(MASTER_DATA, 4);
        ports::write_byte(SLAVE_DATA, 2);

        // we want to use 8086 mode
        ports::write_byte(MASTER_DATA, ICW4_8086);
        ports::write_byte(SLAVE_DATA, ICW4_8086);

        // enable all interrupts (set masks to 0)
        ports::write_byte(MASTER_DATA, 0x00);
        ports::write_byte(SLAVE_DATA, 0x00);
    }
}

/// Report end of interrupt to the PIC
pub fn eoi(irq: usize) {
    // do we have to send EOI?
    if irq as u8 >= REMAP_MASTER && irq as u8 <= REMAP_MASTER + 16 {
        if irq as u8 >= REMAP_SLAVE {
            // notify the slave
            unsafe {
                ports::write_byte(SLAVE_CMD, EOI);
            }
        }

        // notify the master
        unsafe {
            ports::write_byte(MASTER_CMD, EOI);
        }
    }
}
