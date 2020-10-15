use crate::io::{serial, vga};
use crate::isr::{self, State};
use crate::println;
use crate::task::{self, sem::Semaphore};
use crate::util::StaticCell;

static SEM: StaticCell<Semaphore> = StaticCell::new(Semaphore::new(1));

pub extern "C" fn syscall(state: &mut State) {
    match state.eax {
        0 => sys_putc(state),
        1 => sys_up(state),
        2 => sys_down(state),

        n => println!("Got syscall {}: from task {}", n, task::current().id()),
    }
}

pub fn init() {
    isr::set_isr(0x30, syscall);
}

fn sys_putc(state: &mut State) {
    let b = state.ecx as u8;
    // write the byte to the screen and the serial line
    vga::VGA_INST.get_mut().write_char(b);
    serial::write_char(b);
    state.eax = 0;
}

fn sys_up(state: &mut State) {
    SEM.get_mut().up();
    state.eax = 0;
}

fn sys_down(state: &mut State) {
    SEM.get_mut().down();
    state.eax = 0;
}
