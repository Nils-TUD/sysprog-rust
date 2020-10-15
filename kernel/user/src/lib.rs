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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut no = 1;
    loop {
        println!("Hello World {}!", no);
        no += 1;
    }
}
