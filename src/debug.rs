
use std::ffi::c_void;
use crate::funcs;

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct DuneTrapRegs {
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
}

impl DuneTrapRegs {
    funcs!(rax, u64);
    funcs!(rbx, u64);
    funcs!(rcx, u64);
    funcs!(rdx, u64);
    funcs!(rsi, u64);
    funcs!(rdi, u64);
    funcs!(rsp, u64);
    funcs!(rbp, u64);
    funcs!(r8, u64);
    funcs!(r9, u64);
    funcs!(r10, u64);
    funcs!(r11, u64);
    funcs!(r12, u64);
    funcs!(r13, u64);
    funcs!(r14, u64);
    funcs!(r15, u64);
    funcs!(rip, u64);
    funcs!(rflags, u64);
}

pub type DuneTrapNotifyFunc = extern "C" fn(*mut DuneTrapRegs, *mut c_void);

#[no_mangle]
extern "C" fn dummy_notify_func() { }

#[repr(C)]
#[derive(Debug)]
pub struct DuneTrapConfig {
    pub trigger_rip: u64,
    pub notify_func: DuneTrapNotifyFunc,
    pub regs: *mut DuneTrapRegs,
    pub regs_size: u64,
    pub priv_data: *mut c_void,
    pub delay: u8,
}

#[repr(C)]
#[derive(Default)]
pub struct TrapState {
    enabled: u8,
    triggered: u8,
    count: u8,
}

impl TrapState {
    funcs!(enabled, u8);
    funcs!(triggered, u8);
    funcs!(count, u8);
}