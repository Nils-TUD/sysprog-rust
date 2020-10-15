#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![no_std]

extern crate rlibc;

mod support;

fn syscall1(no: usize, arg1: usize) -> usize {
    let mut res = no;
    unsafe {
        llvm_asm!("int $$0x30" : "+{eax}"(res) : "{ecx}"(arg1) : : "volatile");
    }
    res
}

fn sem_down() {
    syscall1(2, 0);
}

fn sem_up() {
    syscall1(1, 0);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut no = 1;
    loop {
        sem_down();
        println!("Hello World {}!", no);
        sem_up();
        no += 1;
    }
}
