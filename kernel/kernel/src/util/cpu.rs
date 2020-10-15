use crate::isr::DescTable;

/// Return the value of CR2, which contains the virtual address that caused the last page fault
#[inline(always)]
pub fn read_cr2() -> usize {
    let res: usize;
    unsafe {
        llvm_asm!("mov %cr2, $0" : "=r"(res))
    };
    res
}

/// Informs the hardware about the interrupt descriptor table
#[inline(always)]
pub unsafe fn load_idt(idt: &DescTable) {
    llvm_asm!("lidt ($0)" : : "r"(idt) : : "volatile");
}

/// Informs the hardware about the global descriptor table
#[inline(always)]
pub unsafe fn load_gdt(gdt: &DescTable) {
    llvm_asm!("lgdt ($0)" : : "r"(gdt) : : "volatile");
}

/// Informs the hardware about the task state segment
#[inline(always)]
pub unsafe fn load_tss(off: usize) {
    llvm_asm!("ltr $0" : : "m"(off) : : "volatile");
}
