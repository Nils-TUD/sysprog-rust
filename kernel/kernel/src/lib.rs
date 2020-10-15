#![feature(core_intrinsics)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![no_std]

mod io;
mod isr;
mod mem;
mod multiboot;
mod pit;
mod syscalls;
mod task;
mod util;

use crate::isr::State;
use crate::task::elf;
use crate::util::StaticCell;

global_asm!(include_str!("start.S"));

pub extern "C" fn timer_irq(state: &mut State) {
    static TICKS: StaticCell<u32> = StaticCell::new(0);
    *TICKS.get_mut() += 1;

    // report EOI before scheduling
    isr::eoi(state.irq);
    if *TICKS % 10 == 0 {
        task::schedule();
    }
}

#[no_mangle]
pub extern "C" fn init(mb: &multiboot::Header, ustack: &mut usize) -> usize {
    io::vga::init();
    io::serial::init();
    mem::init(mb);
    syscalls::init();

    // init interrupt handling
    isr::init();
    pit::init();
    isr::set_isr(0x20, timer_irq);

    // start tasks for all boot modules
    for bmod in mb.mods() {
        let entry = elf::load(bmod.start_addr as usize);
        let tid = task::create(entry);
        crate::println!(
            "task{}: using boot module {} at {:#x} with {} bytes",
            task::get(tid).unwrap().id(),
            bmod.cmdline(),
            { bmod.start_addr },
            bmod.size()
        );
    }

    // prepare the execution of the first task
    let first = task::current();
    *ustack = first.ustack_end();
    isr::set_entry_sp(first.kstack_end());

    // return entry point
    first.entry()
}
