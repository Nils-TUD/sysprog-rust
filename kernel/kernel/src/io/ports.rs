pub type Port = u16;

/// Reads a byte from the given I/O port
pub unsafe fn read_byte(port: Port) -> u8 {
    let val: u8;
    llvm_asm!("inb $1, $0" : "={al}"(val) : "N{dx}"(port) :: "volatile");
    val
}

/// Writes byte `val` to the given I/O port
pub unsafe fn write_byte(port: Port, val: u8) {
    llvm_asm!("outb $1, $0" :: "N{dx}"(port), "{al}"(val) :: "volatile");
}
