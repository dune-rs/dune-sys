use libc::c_int;
use nix::ioctl_none;
use nix::ioctl_read;
use nix::ioctl_readwrite;
use nix::errno::Errno;

use crate::dune::DuneConfig;
use crate::dune::DuneLayout;
use crate::debug::DuneTrapConfig;

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

#[derive(Debug)]
pub enum DuneErrorCode {
    IoctlError(Errno),
    OpenError(Errno),
    CloseError(Errno),
}

pub struct DuneDevice {
    fd: c_int,
}

impl DuneDevice {
    pub fn new() -> Result<Self, Errno> {
        let fd = unsafe { libc::open("/dev/dune\0".as_ptr() as *const i8, libc::O_RDWR) };
        if fd < 0 {
            return Err(Errno::last());
        }
        Ok(Self { fd })
    }

    pub fn enter(&self, config: &mut DuneConfig) -> Result<i32, Errno> {
        unsafe {
            dune_enter(self.fd, config)
        }
    }

    pub fn get_syscall(&self) -> Result<i32, Errno> {
        unsafe {
            dune_get_syscall(self.fd)
        }
    }

    pub fn get_layout(&self, layout: &mut DuneLayout) -> Result<i32, Errno> {
        unsafe {
            dune_get_layout(self.fd, layout)
        }
    }

    pub fn trap_enable(&self, config: &DuneTrapConfig) -> Result<i32, Errno> {
        unsafe {
            dune_trap_enable(self.fd, config as *const _ as *mut _)
        }
    }

    pub fn trap_disable(&self) -> Result<i32, Errno> {
        unsafe {
            dune_trap_disable(self.fd)
        }
    }
}
// The following constants and ioctl definitions are already included in the code above
// so there's no need to redefine them here.

// pub const DUNE_MINOR: u32 = 233;
// const DUNE_IOC_MAGIC: u8 = b'd';

pub const DUNE_ENTER: u64 = 4;
pub const DUNE_GET_SYSCALL: u64 = 0;
pub const DUNE_GET_LAYOUT: u64 = 1;
pub const DUNE_TRAP_ENABLE: u64 = 2;
pub const DUNE_TRAP_DISABLE: u64 = 3;

pub const DUNE_SIGNAL_INTR_BASE: u64 = 200;