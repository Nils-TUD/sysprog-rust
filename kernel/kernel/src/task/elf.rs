use rlibc;

#[repr(C, packed)]
struct Header {
    pub ei_magic: u32,
    pub ei_class: u8,
    pub ei_data: u8,
    pub ei_version: u8,
    pub ei_osabi: u8,
    pub ei_abiversion: u8,
    ei_pad: [u8; 7],
    pub ty: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: usize,
    pub ph_offset: usize,
    pub sh_offset: usize,
    pub flags: u32,
    pub eh_size: u16,
    pub ph_size: u16,
    pub ph_count: u16,
    pub sh_size: u16,
    pub sh_count: u16,
    pub strtab: u16,
}

#[repr(C, packed)]
struct PH32 {
    pub ty: u32,
    pub f_offs: u32,
    pub v_addr: u32,
    pub p_addr: u32,
    pub f_size: u32,
    pub m_size: u32,
    pub flags: u32,
    pub align: u32,
}

const MAGIC: u32 = 0x464C457F;
const PH_LOAD: u32 = 1;

pub fn load(addr: usize) -> usize {
    let head = addr as *const Header;
    // safety: we trust the boot loader
    unsafe {
        assert!((*head).ei_magic == MAGIC);

        let mut ph = (addr + (*head).ph_offset) as *const PH32;
        for _ in 0..(*head).ph_count {
            // we are only interested in load segments
            if (*ph).ty == PH_LOAD {
                crate::println!("Loading {} bytes to {:#x}", (*ph).m_size, (*ph).p_addr);

                // copy from file
                rlibc::memcpy(
                    (*ph).p_addr as *mut u8,
                    (addr + (*ph).f_offs as usize) as *const u8,
                    (*ph).f_size as usize,
                );

                // zero the rest
                rlibc::memset(
                    ((*ph).p_addr + (*ph).f_size) as usize as *mut u8,
                    0,
                    (*ph).m_size as usize - (*ph).f_size as usize,
                );
            }
            ph = ph.add(1);
        }

        (*head).entry
    }
}
