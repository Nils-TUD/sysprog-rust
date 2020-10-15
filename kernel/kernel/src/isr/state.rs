use crate::util::cpu;
use core::fmt;

/// The register state that is saved at kernel entry
#[derive(Default)]
#[repr(C, align(4))]
pub struct State {
    // general purpose registers
    pub edi: usize,
    pub esi: usize,
    pub ebp: usize,
    _esp: usize, // kernel mode sp
    pub ebx: usize,
    pub edx: usize,
    pub ecx: usize,
    pub eax: usize,
    // interrupt-number
    pub irq: usize,
    // error-code (for exceptions); default = 0
    pub error: usize,
    // pushed by the CPU
    pub eip: usize,
    pub cs: usize,
    pub eflags: usize,
    pub esp: usize,
    pub ss: usize,
}

pub fn vec_name(vec: usize) -> &'static str {
    match vec {
        0x00 => "Divide by zero",
        0x01 => "Single step",
        0x02 => "Non maskable",
        0x03 => "Breakpoint",
        0x04 => "Overflow",
        0x05 => "Bounds check",
        0x06 => "Invalid opcode",
        0x07 => "Co-proc. n/a",
        0x08 => "Double fault",
        0x09 => "Co-proc seg. overrun",
        0x0A => "Invalid TSS",
        0x0B => "Segment not present",
        0x0C => "Stack exception",
        0x0D => "Gen. prot. fault",
        0x0E => "Page fault",
        0x10 => "Co-processor error",
        _ => "<unknown>",
    }
}

impl fmt::Debug for State {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let cr2 = cpu::read_cr2();
        writeln!(fmt, "State @ {:#x}", self as *const State as usize)?;
        writeln!(fmt, "  vec: {:#x} ({})", { self.irq }, vec_name(self.irq))?;
        writeln!(fmt, "  cr2:    {:#x}", cr2)?;
        writeln!(fmt, "  error:  {:#x}", { self.error })?;
        writeln!(fmt, "  eip:    {:#x}", { self.eip })?;
        writeln!(fmt, "  eflags: {:#x}", { self.eflags })?;
        writeln!(fmt, "  cs:     {:#x}", { self.cs })?;
        writeln!(fmt, "  ss:     {:#x}", { self.ss })?;
        writeln!(fmt, "  edi:    {:#x}", { self.edi })?;
        writeln!(fmt, "  esi:    {:#x}", { self.edi })?;
        writeln!(fmt, "  ebp:    {:#x}", { self.edi })?;
        writeln!(fmt, "  esp:    {:#x}", { self.esp })?;
        writeln!(fmt, "  ebx:    {:#x}", { self.edi })?;
        writeln!(fmt, "  ecx:    {:#x}", { self.edi })?;
        writeln!(fmt, "  edx:    {:#x}", { self.edi })?;
        writeln!(fmt, "  eax:    {:#x}", { self.edi })?;
        Ok(())
    }
}
