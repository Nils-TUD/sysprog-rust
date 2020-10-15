use crate::mem::PAGE_BITS;
use int_enum::IntEnum;

pub const ISR_COUNT: usize = 256;

/// Descriptor privilege level
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, IntEnum)]
pub enum DPL {
    KERNEL = 0x0,
    USER   = 0x3,
}

/// Segment numbers
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, IntEnum)]
pub enum Segment {
    KCODE = 1,
    KDATA = 2,
    UCODE = 3,
    UDATA = 4,
    TSS   = 5,
}

/// Descriptor table
#[repr(C, packed)]
pub struct DescTable {
    size: u16, // the size of the table -1 (size=0 is not allowed)
    offset: u64,
}

impl DescTable {
    /// Create a new descriptor table with given size and offset
    pub fn new(size: usize, offset: *const u8) -> Self {
        Self {
            size: (size - 1) as u16,
            offset: offset as u64,
        }
    }
}

/// Generic descriptor
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Desc {
    // limit[0..15]
    limit_low: u16,
    // address[0..15]
    addr_low: u16,
    // address[16..23]
    addr_middle: u8,
    // type + DPL + present
    ty: u8,
    // address[24..31] and other fields, depending on the type of descriptor
    addr_high: u16,
}

/// Descriptor type
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, IntEnum)]
#[allow(non_camel_case_types)]
pub enum DescType {
    NULL          = 0x00,
    SYS_TASK_GATE = 0x05,
    SYS_TSS       = 0x09,
    SYS_INTR_GATE = 0x0E,
    DATA_RO       = 0x10,
    DATA_RW       = 0x12,
    CODE_X        = 0x18,
    CODE_XR       = 0x1A,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, IntEnum)]
enum Bits {
    B32 = 0x00,
    B64 = 0x20,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, IntEnum)]
enum Size {
    S16 = 0x00, // 16 bit protected mode
    S32 = 0x40, // 32 bit protected mode
}

/// Granularity as used in segment descriptors
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, IntEnum)]
pub enum Granularity {
    BYTES = 0x00,
    PAGES = 0x80,
}

/// the function type for kernel entries
type EntryFunc = unsafe extern "C" fn();

impl Desc {
    /// Create a new descriptor with all fields set to 0
    pub const fn default() -> Self {
        Self {
            addr_low: 0,
            addr_middle: 0,
            addr_high: 0,
            limit_low: 0,
            ty: 0,
        }
    }

    /// Create a flat descriptor (start address 0, limit = 4G)
    pub fn new_flat(granu: Granularity, ty: DescType, dpl: DPL) -> Self {
        Self::new(0, !0 >> PAGE_BITS, granu, ty, dpl)
    }

    /// Create a task state segment descriptor
    pub fn new_tss(addr: usize, limit: usize, granu: Granularity, dpl: DPL) -> Self {
        Self::new(addr, limit, granu, DescType::SYS_TSS, dpl)
    }

    /// Create an interrupt descriptor table descriptor
    pub fn new_idt(no: usize, handler: EntryFunc, dpl: DPL) -> Self {
        let func_addr = handler as usize;
        let present = (no != 2 && no != 15) as u8; // reserved by intel
        Self {
            addr_low: (Segment::KCODE as u16) << 3,
            addr_middle: 0,
            addr_high: (func_addr >> 16) as u16,
            limit_low: (func_addr & 0xFFFF) as u16,
            ty: (present << 7) | (dpl.int_value() << 5) | DescType::SYS_INTR_GATE.int_value(),
        }
    }

    fn new(addr: usize, limit: usize, granu: Granularity, ty: DescType, dpl: DPL) -> Self {
        let misc: u16 = (Bits::B32.int_value() | Size::S32.int_value() | granu.int_value()) as u16;
        Self {
            addr_low: addr as u16,
            addr_middle: (addr >> 16) as u8,
            addr_high: ((addr & 0xFF00_0000) >> 16) as u16 | ((limit >> 16) & 0xF) as u16 | misc,
            limit_low: (limit & 0xFFFF) as u16,
            ty: (1 << 7) /* present */ | (dpl.int_value() << 5) | ty.int_value(),
        }
    }
}

/// Inner type for the Task State Segment
#[repr(C, packed)]
pub struct TSSInner {
    _reserved1: u32,
    esp0: u32,
    ss0: u16,
    _reserved2: u16,
    _fields: [u32; 11],
    _reserved3: u16,
    io_bitmap: u16,
}

/// Public type for the Task State Segment.
// we make TSSInner packed and TSS aligned to let the compiler do both
#[repr(C, align(4096))]
pub struct TSS {
    inner: TSSInner,
}

impl TSS {
    /// Create a new TSS with given stack pointer and stack selector for ring 0
    pub const fn new(esp0: usize, ss0: u16) -> Self {
        Self {
            inner: TSSInner {
                _reserved1: 0,
                esp0: esp0 as u32,
                ss0,
                _reserved2: 0,
                _fields: [0; 11],
                _reserved3: 0,
                // an invalid offset for the io-bitmap => not loaded yet
                io_bitmap: 104 + 16,
            },
        }
    }

    /// Set the stack pointer for ring 0
    pub fn set_entry_sp(&mut self, sp: usize) {
        self.inner.esp0 = sp as u32;
        self.inner.ss0 = (Segment::KDATA.int_value() << 3 | DPL::KERNEL.int_value()) as u16;
    }
}

/// Inner type for the Global Descriptor Table
#[repr(C, packed)]
pub struct GDTInner {
    pub null: Desc,
    pub kcode: Desc,
    pub kdata: Desc,
    pub ucode: Desc,
    pub udata: Desc,
    pub tss: Desc,
}

/// Public type for the Global Descriptor Table
#[repr(C, align(8))]
pub struct GDT {
    pub inner: GDTInner,
}

impl GDT {
    /// Create a new GDT with default settings
    pub const fn default() -> Self {
        Self {
            inner: GDTInner {
                null: Desc::default(),
                kcode: Desc::default(),
                kdata: Desc::default(),
                ucode: Desc::default(),
                udata: Desc::default(),
                tss: Desc::default(),
            },
        }
    }
}

/// Interrupt Descriptor Table
#[repr(C, align(8))]
pub struct IDT {
    entries: [Desc; ISR_COUNT],
}

impl IDT {
    /// Create a new IDT with invalid descriptors
    pub const fn default() -> Self {
        Self {
            entries: [Desc::default(); ISR_COUNT],
        }
    }

    /// Returns the offset of the IDT
    pub fn offset(&self) -> *const u8 {
        self.entries.as_ptr() as *const _ as *const u8
    }

    /// Sets entry `idx` for the given handler function and privileged level
    pub fn set(&mut self, idx: usize, handler: EntryFunc, dpl: DPL) {
        self.entries[idx] = Desc::new_idt(idx, handler, dpl);
    }
}
