use std::mem::offset_of;

use crate::funcs;
use x86_64::{PhysAddr, VirtAddr};

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DuneConfig {
    ret: i64,
    rax: i64,
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
    funcs!(rax, i64);
    funcs!(rdi, u64);
    funcs!(rsi, u64);
    funcs!(rdx, u64);
    funcs!(rcx, u64);
    funcs!(r8, u64);
    funcs!(r9, u64);
    funcs!(r10, u64);
    funcs!(ret, i64);
    funcs!(rip, u64);
    funcs!(rsp, u64);
    funcs!(rflags, u64);
    funcs!(cr3, u64);
    funcs!(status, i64);
    funcs!(vcpu, u64);
}

pub const DUNE_CFG_RET: usize = offset_of!(DuneConfig, ret);
pub const DUNE_CFG_RAX: usize = offset_of!(DuneConfig, rax);
pub const DUNE_CFG_RBX: usize = offset_of!(DuneConfig, rbx);
pub const DUNE_CFG_RCX: usize = offset_of!(DuneConfig, rcx);
pub const DUNE_CFG_RDX: usize = offset_of!(DuneConfig, rdx);
pub const DUNE_CFG_RSI: usize = offset_of!(DuneConfig, rsi);
pub const DUNE_CFG_RDI: usize = offset_of!(DuneConfig, rdi);
pub const DUNE_CFG_RSP: usize = offset_of!(DuneConfig, rsp);
pub const DUNE_CFG_RBP: usize = offset_of!(DuneConfig, rbp);
pub const DUNE_CFG_R8: usize = offset_of!(DuneConfig, r8);
pub const DUNE_CFG_R9: usize = offset_of!(DuneConfig, r9);
pub const DUNE_CFG_R10: usize = offset_of!(DuneConfig, r10);
pub const DUNE_CFG_R11: usize = offset_of!(DuneConfig, r11);
pub const DUNE_CFG_R12: usize = offset_of!(DuneConfig, r12);
pub const DUNE_CFG_R13: usize = offset_of!(DuneConfig, r13);
pub const DUNE_CFG_R14: usize = offset_of!(DuneConfig, r14);
pub const DUNE_CFG_R15: usize = offset_of!(DuneConfig, r15);
pub const DUNE_CFG_RIP: usize = offset_of!(DuneConfig, rip);
pub const DUNE_CFG_RFLAGS: usize = offset_of!(DuneConfig, rflags);
pub const DUNE_CFG_CR3: usize = offset_of!(DuneConfig, cr3);
pub const DUNE_CFG_STATUS: usize = offset_of!(DuneConfig, status);
pub const DUNE_CFG_VCPU: usize = offset_of!(DuneConfig, vcpu);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DuneLayout {
    phys_limit: PhysAddr,
    base_map: VirtAddr,
    base_stack: VirtAddr,
}

impl DuneLayout {
    funcs!(phys_limit, PhysAddr);
    funcs!(base_map, VirtAddr);
    funcs!(base_stack, VirtAddr);

    #[allow(dead_code)]
    fn new(phys_limit: PhysAddr, base_map: VirtAddr, base_stack: VirtAddr) -> Self {
        Self {
            phys_limit,
            base_map,
            base_stack,
        }
    }
}

impl Default for DuneLayout {
    fn default() -> Self {
        Self {
            phys_limit: PhysAddr::new(0),
            base_map: VirtAddr::new(0),
            base_stack: VirtAddr::new(0),
        }
    }
}

pub const GPA_STACK_SIZE: u64 = 1 << 30; // 1 gigabyte
pub const GPA_MAP_SIZE: u64 = (1 << 36) - GPA_STACK_SIZE; // 63 gigabytes

pub const DUNE_RET_NONE: i64 = 0;
pub const DUNE_RET_EXIT: i64 = 1;
pub const DUNE_RET_SYSCALL: i64 = 2;
pub const DUNE_RET_INTERRUPT: i64 = 3;
pub const DUNE_RET_SIGNAL: i64 = 4;
pub const DUNE_RET_EPT_VIOLATION: i64 = 5;
pub const DUNE_RET_NOENTER: i64 = 6;
pub const DUNE_RET_UNHANDLED_VMEXIT: i64 = 7;

pub enum DuneRetCode {
    None = 0,
    Exit = 1,
    Syscall = 2,
    Interrupt = 3,
    Signal = 4,
    EptViolation = 5,
    NoEnter = 6,
    UnhandledVmexit = 7,
    Unknown,
}

impl From<i64> for DuneRetCode {
    fn from(code: i64) -> Self {
        match code {
            DUNE_RET_NONE => DuneRetCode::None,
            DUNE_RET_EXIT => DuneRetCode::Exit,
            DUNE_RET_SYSCALL => DuneRetCode::Syscall,
            DUNE_RET_EPT_VIOLATION => DuneRetCode::EptViolation,
            DUNE_RET_INTERRUPT => DuneRetCode::Interrupt,
            DUNE_RET_SIGNAL => DuneRetCode::Signal,
            DUNE_RET_UNHANDLED_VMEXIT => DuneRetCode::UnhandledVmexit,
            DUNE_RET_NOENTER => DuneRetCode::NoEnter,
            _ => DuneRetCode::Unknown,
        }
    }
}