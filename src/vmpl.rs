use crate::funcs;
use x86_64::{PhysAddr, VirtAddr};

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VmsaSeg {
    // Define the fields of VmsaSeg here
    // For example:
    selector: u16,
    attrib: u16,
    limit: u32,
    base: u64,
}

impl VmsaSeg {
    // Define the methods of VmsaSeg here
    // For example:
    pub fn new() -> Self {
        Self {
            selector: 0,
            attrib: 0,
            limit: 0,
            base: 0,
        }
    }

    funcs!(selector, u16);
    funcs!(attrib, u16);
    funcs!(limit, u32);
    funcs!(base, u64);
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VcpuConfig {
    fs: VmsaSeg,
    gs: VmsaSeg,
    gdtr: VmsaSeg,
    idtr: VmsaSeg,
    tr: VmsaSeg,
    lstar: u64,
}

impl VcpuConfig {

    funcs!(fs, VmsaSeg);
    funcs!(gs, VmsaSeg);
    funcs!(gdtr, VmsaSeg);
    funcs!(idtr, VmsaSeg);
    funcs!(tr, VmsaSeg);
    funcs!(lstar, u64);
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct GetPages {
    num_pages: u64,
    mapping: u64,
    phys: u64,
}

impl GetPages {

    funcs!(num_pages, u64);
    funcs!(mapping, u64);
    funcs!(phys, u64);

    pub fn new() -> Self {
        Self { num_pages: 0, mapping: 0, phys: 0 }
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct VmplArgs {
    gva: u64,
    page_size: u32,
    attrs: u32,
    nr_pages: u32,
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct VmplLayout {
    phys_base: PhysAddr,
    phys_end: PhysAddr,
    mmap_base: VirtAddr,
    mmap_end: VirtAddr,
}

impl VmplLayout {
    pub fn new() -> Self {
        Self {
            phys_base: PhysAddr::new(0),
            phys_end: PhysAddr::new(0),
            mmap_base: VirtAddr::new(0),
            mmap_end: VirtAddr::new(0),
        }
    }

    funcs!(phys_base, PhysAddr);
    funcs!(phys_end, PhysAddr);
    funcs!(mmap_base, VirtAddr);
    funcs!(mmap_end, VirtAddr);
}

pub const SEIMI_PGD_USER: u64 = 253;
pub const SEIMI_PGD_SUPER: u64 = 252;
pub const SEIMI_MMAP_BASE_USER: u64 = SEIMI_PGD_USER << 39;
pub const SEIMI_MMAP_BASE_SUPER: u64 = SEIMI_PGD_SUPER << 39;

// pub const PGTABLE_MMAP_BASE: u64 = SEIMI_MMAP_BASE_USER + 0x200000000;
pub const PGTABLE_MMAP_BASE: u64 = 0x200000000;
pub const PGTABLE_MMAP_SIZE: u64 = 0x480000000;
pub const PGTABLE_MMAP_END: u64 = PGTABLE_MMAP_BASE + PGTABLE_MMAP_SIZE;
// const PGTABLE_MMAP_BASE: VirtAddr = VirtAddr::new(0x100000000);
// const PGTABLE_MMAP_END: VirtAddr = VirtAddr::new(0x200000000);

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VmplSeimi {
    pub pgd_user: u64,
    pub pgd_super: u64,
}

impl VmplSeimi {
    pub fn new(pgd_user: u64, pgd_super: u64) -> Self {
        Self { pgd_user, pgd_super }
    }

    funcs!(pgd_user, u64);
    funcs!(pgd_super, u64);
}