use libc::c_int;
use nix::ioctl_none;
use nix::ioctl_read;
use nix::ioctl_readwrite;
use nix::errno::Errno;
use nix::ioctl_write_ptr;

use crate::dune::DuneConfig;
use crate::dune::DuneLayout;
use crate::debug::DuneTrapConfig;
use crate::funcs;
use crate::DuneTrapRegs;
use crate::IdtDescriptor;
use crate::IDT_ENTRIES;
use crate::Result;

/*
 * IOCTL interface
 */
/* FIXME: this must be reserved in miscdevice.h */
pub const DUNE_MINOR: u32 = 233;

const DUNE_IOC_MAGIC: u8 = b'd';

const DUNE_IOC_ENTER: u8 = 0x01;
const DUNE_IOC_GET_SYSCALL: u8 = 0x02;
const DUNE_IOC_GET_LAYOUT: u8 = 0x03;
const DUNE_IOC_TRAP_ENABLE: u8 = 0x04;
const DUNE_IOC_TRAP_DISABLE: u8 = 0x05;

ioctl_read!(dune_enter, DUNE_IOC_MAGIC, DUNE_IOC_ENTER, DuneConfig);
ioctl_read!(dune_get_syscall, DUNE_IOC_MAGIC,DUNE_IOC_GET_SYSCALL, u64);
ioctl_read!(dune_get_layout, DUNE_IOC_MAGIC, DUNE_IOC_GET_LAYOUT, DuneLayout);
ioctl_readwrite!(dune_trap_enable, DUNE_IOC_MAGIC, DUNE_IOC_TRAP_ENABLE, DuneTrapConfig);
ioctl_none!(dune_trap_disable, DUNE_IOC_MAGIC, DUNE_IOC_TRAP_DISABLE);

pub trait Device : Send + Sync {
    fn fd(&self) -> c_int;
    fn open(&mut self, path: &str) -> Result<i32>;
    fn close(&self) -> Result<i32>;
    fn ioctl<T>(&self, request: i32, arg: *mut T) -> Result<i32>;
}

#[derive(Debug, Copy, Clone)]
pub struct BaseDevice {
    fd: c_int,
}

impl BaseDevice {
    funcs!(fd, c_int);

    pub fn new() -> Self {
        Self { fd: -1 }
    }
}

impl Device for BaseDevice {

    fn fd(&self) -> c_int {
        self.fd
    }

    fn open(&mut self, path: &str) -> Result<i32> {
        let fd = unsafe { libc::open(path.as_ptr() as *const i8, libc::O_RDWR) };
        if fd < 0 {
            return Err(crate::Error::LibcError(Errno::last()));
        }
        self.fd = fd;
        Ok(fd)
    }

    fn close(&self) -> Result<i32> {
        unsafe {
            let ret = libc::close(self.fd);
            if ret < 0 {
                return Err(crate::Error::LibcError(Errno::last()));
            }
        }
        Ok(0)
    }

    fn ioctl<T>(&self, request: i32, arg: *mut T) -> Result<i32> {
        unsafe {
            let ret = libc::ioctl(self.fd, request, arg);
            if ret < 0 {
                return Err(crate::Error::LibcError(Errno::last()));
            }
        }
        Ok(0)
    }
}

#[allow(dead_code)]
pub trait WithInterrupt {
    fn get_idt<'a>(&self) -> &[IdtDescriptor; IDT_ENTRIES];
    fn get_idt_mut<'a>(&mut self) -> &mut [IdtDescriptor; IDT_ENTRIES];
    fn get_trap_regs_mut<'a>(&mut self) -> &mut DuneTrapRegs;
}

#[derive(Debug, Copy, Clone)]
pub struct BaseSystem {
    device: BaseDevice,
    #[allow(dead_code)]
    idt: [IdtDescriptor; IDT_ENTRIES],
    #[allow(dead_code)]
    trap_regs: DuneTrapRegs,
}

impl BaseSystem {

    pub fn new() -> Self {
        Self {
            device: BaseDevice::new(),
            idt: [IdtDescriptor::default(); IDT_ENTRIES],
            trap_regs: DuneTrapRegs::default(),
        }
    }
}

impl WithInterrupt for BaseSystem {

    fn get_idt<'a>(&self) -> &[IdtDescriptor; IDT_ENTRIES] {
        &self.idt
    }

    fn get_idt_mut<'a>(&mut self) -> &mut [IdtDescriptor; IDT_ENTRIES] {
        &mut self.idt
    }

    fn get_trap_regs_mut<'a>(&mut self) -> &mut DuneTrapRegs {
        &mut self.trap_regs
    }
}

impl Device for BaseSystem {

    fn fd(&self) -> c_int {
        self.device.fd()
    }

    fn open(&mut self, path: &str) -> Result<i32> {
        self.device.open(path)
    }

    fn close(&self) -> Result<i32> {
        self.device.close()
    }

    fn ioctl<T>(&self, request: i32, arg: *mut T) -> Result<i32> {
        self.device.ioctl(request, arg)
    }
}

// The following constants and ioctl definitions are already included in the code above
// so there's no need to redefine them here.

// pub const DUNE_MINOR: u32 = 233;
// const DUNE_IOC_MAGIC: u8 = b'd';

pub const IOCTL_DUNE_ENTER: u64 = 0xc0b0e901;
pub const DUNE_ENTER: u64 = 4;
pub const DUNE_GET_SYSCALL: u64 = 0;
pub const DUNE_GET_LAYOUT: u64 = 1;
pub const DUNE_TRAP_ENABLE: u64 = 2;
pub const DUNE_TRAP_DISABLE: u64 = 3;

pub const DUNE_SIGNAL_INTR_BASE: u64 = 200;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct VmplArgs {
    gva: u64,
    page_size: u32,
    attrs: u32,
    nr_pages: u32,
}

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
#[derive(Debug, Copy, Clone)]
struct GetPages {
    num_pages: u64,
    mapping: u64,
    phys: u64,
}

impl GetPages {

    funcs!(num_pages, u64);
    funcs!(mapping, u64);
    funcs!(phys, u64);
}

pub const SEIMI_PGD_USER: u64 = 253;
pub const SEIMI_PGD_SUPER: u64 = 252;
pub const SEIMI_MMAP_BASE_USER: u64 = SEIMI_PGD_USER << 39;
pub const SEIMI_MMAP_BASE_SUPER: u64 = SEIMI_PGD_SUPER << 39;

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

const VMPL_IOCTL_MAGIC: u8 = b'k';

/*
 * IOCTL interface
 */
ioctl_none!(vmpl_create_vm, VMPL_IOCTL_MAGIC, 0x10);
ioctl_readwrite!(vmpl_set_pgtable_vmpl, VMPL_IOCTL_MAGIC, 0x11, VmplArgs);
ioctl_readwrite!(vmpl_set_page_vmpl, VMPL_IOCTL_MAGIC, 0x12, VmplArgs);
ioctl_write_ptr!(vmpl_create_vcpu, VMPL_IOCTL_MAGIC, 0x20, VcpuConfig);
ioctl_readwrite!(vmpl_vmpl_run, VMPL_IOCTL_MAGIC, 0x14, DuneConfig);
ioctl_read!(vmpl_get_ghcb, VMPL_IOCTL_MAGIC, 0x15, u64);
ioctl_read!(vmpl_get_cr3, VMPL_IOCTL_MAGIC, 0x16, u64);
ioctl_readwrite!(vmpl_get_pages, VMPL_IOCTL_MAGIC, 0x17, GetPages);
ioctl_readwrite!(vmpl_set_seimi, VMPL_IOCTL_MAGIC, 0x18, VmplSeimi);
ioctl_write_ptr!(vmpl_set_config, VMPL_IOCTL_MAGIC, 0x21, VcpuConfig);
ioctl_read!(vmpl_get_config, VMPL_IOCTL_MAGIC, 0x22, VcpuConfig);

// #define VMPL_IOCTL_CREATE_VM        _IO(VMPL_IOCTL_MAGIC, 0x10)
// #define VMPL_IOCTL_SET_PGTABLE_VMPL _IOW(VMPL_IOCTL_MAGIC, 0x11, struct vmpl_args_t)
// #define VMPL_IOCTL_SET_PAGE_VMPL    _IOW(VMPL_IOCTL_MAGIC, 0x12, struct vmpl_args_t)
// #define VMPL_IOCTL_CREATE_VCPU  _IOW(VMPL_IOCTL_MAGIC, 0x20, struct vcpu_config)
// #define VMPL_IOCTL_VMPL_RUN     _IOWR(VMPL_IOCTL_MAGIC, 0x14, struct dune_config)
// #define VMPL_IOCTL_GET_GHCB     _IOR(VMPL_IOCTL_MAGIC, 0x15, uint64_t)
// #define VMPL_IOCTL_GET_CR3      _IOR(VMPL_IOCTL_MAGIC, 0x16, uint64_t)
// #define VMPL_IOCTL_GET_PAGES    _IOWR(VMPL_IOCTL_MAGIC, 0x17, struct get_pages_t)
// #define VMPL_IOCTL_SET_SEIMI    _IOW(VMPL_IOCTL_MAGIC, 0x18, uint64_t)
// #define VMPL_IOCTL_SET_CONFIG     _IOW(VMPL_IOCTL_MAGIC, 0x21, struct vcpu_config)
// #define VMPL_IOCTL_GET_CONFIG     _IOR(VMPL_IOCTL_MAGIC, 0x22, struct vcpu_config)