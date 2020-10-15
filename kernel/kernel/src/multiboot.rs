use crate::util;
use core::fmt;
use core::intrinsics;
use core::iter;

const FL_MODULES: u32 = 0b00001000;
const FL_MEMMAP: u32 = 0b01000000;

/// MultiBoot header, provided by the boot loader
#[repr(C, packed)]
pub struct Header {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    pub mmap_len: u32,
    pub mmap_addr: u32,
    pub drives_len: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub loader_name: u32,
}

/// A boot module
#[repr(C, packed)]
pub struct Module {
    /// The modules (physical) start address
    pub start_addr: u32,
    /// The modules (physical) end address
    pub end_addr: u32,
    cmdline: u32,
    _reserved: u32,
}

impl Module {
    /// The size of the module in bytes
    pub fn size(&self) -> usize {
        self.end_addr as usize - self.start_addr as usize
    }

    /// The modules command line
    pub fn cmdline(&self) -> &'static str {
        // safety: we trust the boot loader
        unsafe { util::cstr_to_str(self.cmdline as *const i8) }
    }
}

#[repr(C, packed)]
pub struct Memory {
    size: u32,
    pub addr: u64,
    pub len: u64,
    pub ty: u32,
}

impl Memory {
    /// Returns free if this memory region is available for use
    pub fn is_free(&self) -> bool {
        self.ty == 1
    }
}

impl Header {
    /// Returns true if modules are available
    pub fn have_modules(&self) -> bool {
        (self.flags & FL_MODULES) != 0
    }

    /// Returns true if the memory map is available
    pub fn have_mmap(&self) -> bool {
        (self.flags & FL_MEMMAP) != 0
    }

    /// Returns the kernels command line
    pub fn cmdline(&self) -> &'static str {
        // safety: we trust the boot loader
        unsafe { util::cstr_to_str(self.cmdline as *const i8) }
    }

    /// Returns an iterator for all boot modules
    pub fn mods(&self) -> ModIterator {
        assert!(self.have_modules());
        ModIterator {
            addr: self.mods_addr as usize,
            end: self.mods_addr as usize
                + intrinsics::size_of::<Module>() * self.mods_count as usize,
        }
    }

    /// Returns an iterator for all memory regions
    pub fn mmap(&self) -> MmapIterator {
        assert!(self.have_mmap());
        MmapIterator {
            addr: self.mmap_addr as usize,
            end: self.mmap_addr as usize + self.mmap_len as usize,
        }
    }
}

/// The iterator for boot modules
pub struct ModIterator {
    addr: usize,
    end: usize,
}

impl iter::Iterator for ModIterator {
    type Item = &'static Module;

    fn next(&mut self) -> Option<Self::Item> {
        if self.addr < self.end {
            let addr = self.addr;
            self.addr += intrinsics::size_of::<Module>();
            // safety: we trust the boot loader
            unsafe { Some(intrinsics::transmute(addr)) }
        }
        else {
            None
        }
    }
}

/// The iterator for memory regions
pub struct MmapIterator {
    addr: usize,
    end: usize,
}

impl iter::Iterator for MmapIterator {
    type Item = &'static Memory;

    fn next(&mut self) -> Option<Self::Item> {
        if self.addr < self.end {
            let mem = self.addr as *const Memory;
            // safety: we trust the boot loader
            unsafe {
                self.addr += (*mem).size as usize + 4;
                Some(intrinsics::transmute(mem))
            }
        }
        else {
            None
        }
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "MultiBoot {{")?;
        writeln!(f, "  flags: {:#x}", { self.flags })?;
        writeln!(
            f,
            "  memory: {:#x} lower, {:#x} upper",
            { self.mem_lower },
            { self.mem_upper }
        )?;
        writeln!(f, "  cmdline: {}", self.cmdline())?;

        writeln!(f, "  mods: [")?;
        for bmod in self.mods() {
            writeln!(
                f,
                "    addr: {:#x}, size: {:#x}, cmdline: {}",
                { bmod.start_addr },
                bmod.size(),
                bmod.cmdline(),
            )?;
        }
        writeln!(f, "  ]")?;

        writeln!(f, "  memory map: [")?;
        for mem in self.mmap() {
            writeln!(
                f,
                "    addr: {:#x}, size: {:#x}, type: {:#x}",
                { mem.addr },
                { mem.len },
                { mem.ty },
            )?;
        }
        writeln!(f, "  ]")?;
        writeln!(f, "}}")
    }
}
