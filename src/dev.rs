use libc::c_int;
use nix::ioctl_none;
use nix::ioctl_read;
use nix::ioctl_readwrite;
use nix::errno::Errno;

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
ioctl_none!(dune_get_syscall, DUNE_IOC_MAGIC,DUNE_IOC_GET_SYSCALL);
ioctl_read!(dune_get_layout, DUNE_IOC_MAGIC, DUNE_IOC_GET_LAYOUT, DuneLayout);
ioctl_readwrite!(dune_trap_enable, DUNE_IOC_MAGIC, DUNE_IOC_TRAP_ENABLE, DuneTrapConfig);
ioctl_none!(dune_trap_disable, DUNE_IOC_MAGIC, DUNE_IOC_TRAP_DISABLE);

pub trait Device {
    fn fd(&self) -> c_int;
    fn open(&mut self, path: &str) -> Result<i32>;
    fn close(&self) -> Result<i32>;
    fn ioctl<T>(&self, request: u64, arg: *mut T) -> Result<i32>;
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

    fn ioctl<T>(&self, request: u64, arg: *mut T) -> Result<i32> {
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

    fn ioctl<T>(&self, request: u64, arg: *mut T) -> Result<i32> {
        self.device.ioctl(request, arg)
    }
}

// The following constants and ioctl definitions are already included in the code above
// so there's no need to redefine them here.

// pub const DUNE_MINOR: u32 = 233;
// const DUNE_IOC_MAGIC: u8 = b'd';

pub const IOCTL_DUNE_ENTER: u64 = 0x80b0e901;
pub const DUNE_ENTER: u64 = 4;
pub const DUNE_GET_SYSCALL: u64 = 0;
pub const DUNE_GET_LAYOUT: u64 = 1;
pub const DUNE_TRAP_ENABLE: u64 = 2;
pub const DUNE_TRAP_DISABLE: u64 = 3;

pub const DUNE_SIGNAL_INTR_BASE: u64 = 200;