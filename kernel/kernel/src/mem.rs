use crate::multiboot;
use crate::util::StaticCell;

pub const PAGE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_BITS;

struct Region {
    addr: u64,
    size: u64,
}

extern "C" {
    static bss_end: u8;
}

static REG: StaticCell<Region> = StaticCell::new(Region { addr: 0, size: 0 });

/// Initializes the physical memory manager using the given MultiBoot header
pub fn init(mb: &multiboot::Header) {
    // find the largest chunk of free memory
    let mem = mb
        .mmap()
        .filter(|m| m.is_free())
        .max_by(|a, b| { a.len }.cmp(&{ b.len }))
        .unwrap();

    let mut mem_begin = mem.addr;
    let mem_end = mem.addr + mem.len;

    // start behind modules
    for m in mb.mods() {
        if m.start_addr as u64 >= mem_begin && (m.start_addr as u64) < mem_end {
            mem_begin = m.end_addr as u64;
        }
    }
    // and our own memory
    let kernel_end = unsafe { &bss_end as *const u8 as usize } as u64;
    if kernel_end >= mem_begin && kernel_end < mem_end {
        mem_begin = kernel_end;
    }

    // page-align it
    mem_begin = (mem_begin + PAGE_SIZE as u64 - 1) & !(PAGE_SIZE as u64 - 1);

    crate::println!("Using physical memory {:#x}..{:#x}", mem_begin, mem_end);
    REG.set(Region {
        addr: mem_begin,
        size: mem_end - mem_begin,
    });
}

/// Allocate a single frame and return the physical address
pub fn alloc_page() -> usize {
    assert!(REG.addr + PAGE_SIZE as u64 <= REG.addr + REG.size);
    REG.get_mut().addr += PAGE_SIZE as u64;
    REG.addr as usize - PAGE_SIZE
}
