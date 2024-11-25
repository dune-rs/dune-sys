use crate::funcs;
use x86_64::{PhysAddr, VirtAddr};

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DuneConfig {
    ret: i64,
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rsp: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    rflags: u64,
    cr3: u64,
    status: i64,
    vcpu: u64,
}

impl DuneConfig {
    funcs!(ret, i64);
    funcs!(rip, u64);
    funcs!(rsp, u64);
    funcs!(rflags, u64);
    funcs!(cr3, u64);
    funcs!(status, i64);
    funcs!(vcpu, u64);
}

pub const DUNE_CFG_RET: u64 = 0x00;
pub const DUNE_CFG_RAX: u64 = 0x08;
pub const DUNE_CFG_RBX: u64 = 0x10;
pub const DUNE_CFG_RCX: u64 = 0x18;
pub const DUNE_CFG_RDX: u64 = 0x20;
pub const DUNE_CFG_RSI: u64 = 0x28;
pub const DUNE_CFG_RDI: u64 = 0x30;
pub const DUNE_CFG_RSP: u64 = 0x38;
pub const DUNE_CFG_RBP: u64 = 0x40;
pub const DUNE_CFG_R8: u64 = 0x48;
pub const DUNE_CFG_R9: u64 = 0x50;
pub const DUNE_CFG_R10: u64 = 0x58;
pub const DUNE_CFG_R11: u64 = 0x60;
pub const DUNE_CFG_R12: u64 = 0x68;
pub const DUNE_CFG_R13: u64 = 0x70;
pub const DUNE_CFG_R14: u64 = 0x78;
pub const DUNE_CFG_R15: u64 = 0x80;
pub const DUNE_CFG_RIP: u64 = 0x88;
pub const DUNE_CFG_RFLAGS: u64 = 0x90;
pub const DUNE_CFG_CR3: u64 = 0x98;
pub const DUNE_CFG_STATUS: u64 = 0xa0;
pub const DUNE_CFG_VCPU: u64 = 0xa8;

#[repr(C)]
#[derive(Debug)]
pub struct DuneLayout {
    phys_limit: PhysAddr,
    base_map: VirtAddr,
    base_stack: VirtAddr,
}

impl DuneLayout {
    funcs!(phys_limit, PhysAddr);
    funcs!(base_map, VirtAddr);
    funcs!(base_stack, VirtAddr);
}

pub const GPA_STACK_SIZE: u64 = 1 << 30; // 1 gigabyte
pub const GPA_MAP_SIZE: u64 = (1 << 36) - GPA_STACK_SIZE; // 63 gigabytes

pub const DUNE_RET_EXIT: i64 = 1;
pub const DUNE_RET_EPT_VIOLATION: i64 = 2;
pub const DUNE_RET_INTERRUPT: i64 = 3;
pub const DUNE_RET_SIGNAL: i64 = 4;
pub const DUNE_RET_UNHANDLED_VMEXIT: i64 = 5;
pub const DUNE_RET_NOENTER: i64 = 6;

pub enum DuneRetCode {
    Exit = 1,
    EptViolation = 2,
    Interrupt = 3,
    Signal = 4,
    UnhandledVmexit = 5,
    NoEnter = 6,
    Unknown = 7,
}

impl From<i64> for DuneRetCode {
    fn from(code: i64) -> Self {
        match code {
            DUNE_RET_EXIT => DuneRetCode::Exit,
            DUNE_RET_EPT_VIOLATION => DuneRetCode::EptViolation,
            DUNE_RET_INTERRUPT => DuneRetCode::Interrupt,
            DUNE_RET_SIGNAL => DuneRetCode::Signal,
            DUNE_RET_UNHANDLED_VMEXIT => DuneRetCode::UnhandledVmexit,
            DUNE_RET_NOENTER => DuneRetCode::NoEnter,
            _ => DuneRetCode::Unknown,
        }
    }
}