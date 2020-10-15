mod desc;
mod pic;
mod state;

pub use desc::DescTable;
pub use state::State;

use crate::isr::desc::{
    Desc, DescType, Granularity, Segment, TSSInner, DPL, GDT, IDT, ISR_COUNT, TSS,
};
use crate::println;
use crate::util::{cpu, StaticCell};
use core::intrinsics;

extern "C" {
    // exceptions
    fn isr_0();
    fn isr_1();
    fn isr_2();
    fn isr_3();
    fn isr_4();
    fn isr_5();
    fn isr_6();
    fn isr_7();
    fn isr_8();
    fn isr_9();
    fn isr_10();
    fn isr_11();
    fn isr_12();
    fn isr_13();
    fn isr_14();
    fn isr_15();
    fn isr_16();
    fn isr_17();
    fn isr_18();
    fn isr_19();
    fn isr_20();
    fn isr_21();
    fn isr_22();
    fn isr_23();
    fn isr_24();
    fn isr_25();
    fn isr_26();
    fn isr_27();
    fn isr_28();
    fn isr_29();
    fn isr_30();
    fn isr_31();
    // interrupts
    fn isr_32();
    fn isr_33();
    fn isr_34();
    fn isr_35();
    fn isr_36();
    fn isr_37();
    fn isr_38();
    fn isr_39();
    fn isr_40();
    fn isr_41();
    fn isr_42();
    fn isr_43();
    fn isr_44();
    fn isr_45();
    fn isr_46();
    fn isr_47();
    fn isr_48();
    // the handler for a other interrupts
    fn isr_null();
}

global_asm!(include_str!("entry.S"));

static GDT: StaticCell<GDT> = StaticCell::new(GDT::default());
static IDT: StaticCell<IDT> = StaticCell::new(IDT::default());
static TSS: StaticCell<TSS> = StaticCell::new(TSS::new(0, 0));

type IsrFunc = extern "C" fn(state: &mut State);

static ISRS: StaticCell<[IsrFunc; ISR_COUNT]> = StaticCell::new([unexpected_irq; ISR_COUNT]);

#[no_mangle]
pub extern "C" fn isr_handler(state: &mut State) {
    ISRS[state.irq](state);
}

#[no_mangle]
pub extern "C" fn unexpected_irq(state: &mut State) {
    println!("Unexpected IRQ {} ({})", state.irq, state::vec_name(state.irq));
    eoi(state.irq);
}

/// Initializes interrupt handling
pub fn init() {
    pic::init();

    // initialize GDT
    let gdt = &mut GDT.get_mut().inner;
    gdt.kcode = Desc::new_flat(Granularity::PAGES, DescType::CODE_XR, DPL::KERNEL);
    gdt.kdata = Desc::new_flat(Granularity::PAGES, DescType::DATA_RW, DPL::KERNEL);
    gdt.ucode = Desc::new_flat(Granularity::PAGES, DescType::CODE_XR, DPL::USER);
    gdt.udata = Desc::new_flat(Granularity::PAGES, DescType::DATA_RW, DPL::USER);
    gdt.tss = Desc::new_tss(
        TSS.get() as *const _ as *const u8 as usize,
        intrinsics::size_of::<TSSInner>() - 1,
        Granularity::BYTES,
        DPL::KERNEL,
    );

    // load GDT and TSS
    let gdt_tbl = DescTable::new(intrinsics::size_of::<GDT>(), gdt as *const _ as *const u8);
    let tss_off = Segment::TSS as usize * intrinsics::size_of::<Desc>();
    unsafe {
        cpu::load_gdt(&gdt_tbl);
        cpu::load_tss(tss_off);
    }

    // setup the idt
    let idt = IDT.get_mut();
    idt.set(0, isr_0, DPL::KERNEL);
    idt.set(1, isr_1, DPL::KERNEL);
    idt.set(2, isr_2, DPL::KERNEL);
    idt.set(3, isr_3, DPL::KERNEL);
    idt.set(4, isr_4, DPL::KERNEL);
    idt.set(5, isr_5, DPL::KERNEL);
    idt.set(6, isr_6, DPL::KERNEL);
    idt.set(7, isr_7, DPL::KERNEL);
    idt.set(8, isr_8, DPL::KERNEL);
    idt.set(9, isr_9, DPL::KERNEL);
    idt.set(10, isr_10, DPL::KERNEL);
    idt.set(11, isr_11, DPL::KERNEL);
    idt.set(12, isr_12, DPL::KERNEL);
    idt.set(13, isr_13, DPL::KERNEL);
    idt.set(14, isr_14, DPL::KERNEL);
    idt.set(15, isr_15, DPL::KERNEL);
    idt.set(16, isr_16, DPL::KERNEL);
    idt.set(17, isr_17, DPL::KERNEL);
    idt.set(18, isr_18, DPL::KERNEL);
    idt.set(19, isr_19, DPL::KERNEL);
    idt.set(20, isr_20, DPL::KERNEL);
    idt.set(21, isr_21, DPL::KERNEL);
    idt.set(22, isr_22, DPL::KERNEL);
    idt.set(23, isr_23, DPL::KERNEL);
    idt.set(24, isr_24, DPL::KERNEL);
    idt.set(25, isr_25, DPL::KERNEL);
    idt.set(26, isr_26, DPL::KERNEL);
    idt.set(27, isr_27, DPL::KERNEL);
    idt.set(28, isr_28, DPL::KERNEL);
    idt.set(29, isr_29, DPL::KERNEL);
    idt.set(30, isr_30, DPL::KERNEL);
    idt.set(31, isr_31, DPL::KERNEL);
    idt.set(32, isr_32, DPL::KERNEL);
    idt.set(33, isr_33, DPL::KERNEL);
    idt.set(34, isr_34, DPL::KERNEL);
    idt.set(35, isr_35, DPL::KERNEL);
    idt.set(36, isr_36, DPL::KERNEL);
    idt.set(37, isr_37, DPL::KERNEL);
    idt.set(38, isr_38, DPL::KERNEL);
    idt.set(39, isr_39, DPL::KERNEL);
    idt.set(40, isr_40, DPL::KERNEL);
    idt.set(41, isr_41, DPL::KERNEL);
    idt.set(42, isr_42, DPL::KERNEL);
    idt.set(43, isr_43, DPL::KERNEL);
    idt.set(44, isr_44, DPL::KERNEL);
    idt.set(45, isr_45, DPL::KERNEL);
    idt.set(46, isr_46, DPL::KERNEL);
    idt.set(47, isr_47, DPL::KERNEL);
    idt.set(48, isr_48, DPL::USER);

    // all other interrupts
    for i in 49..256 {
        idt.set(i, isr_null, DPL::KERNEL);
    }

    // now we can use our idt
    let idt_tbl = DescTable::new(ISR_COUNT * intrinsics::size_of::<Desc>(), IDT.offset());
    unsafe {
        cpu::load_idt(&idt_tbl);
    }
}

/// Register `func` as the interrupt service routine for vector `idx`
pub fn set_isr(idx: usize, func: IsrFunc) {
    ISRS.get_mut()[idx] = func;
}

/// Set the kernel stack pointer for the next kernel entry
pub fn set_entry_sp(sp: usize) {
    TSS.get_mut().set_entry_sp(sp);
}

/// Report end of interrupt to the hardware
pub fn eoi(irq: usize) {
    pic::eoi(irq);
}
