use std::mem::offset_of;

use crate::{funcs, funcs_vec};


#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Tptr {
    limit: u16,
    base: u64,
}

impl Tptr {
    funcs!(limit, u16);
    funcs!(base, u64);
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Tss {
    reserved0: u32,
    pub tss_rsp: [u64; 3], // Stack pointer for CPL 0, 1, 2
    reserved1: u64,
    tss_ist: [u64; 7], // Note: tss_ist[0] is ignored
    reserved2: u64,
    reserved3: u16,
    tss_iomb: u16, // I/O map base
    tss_iopb: [u8; 0],
}

pub const TSS_IOPB : usize = offset_of!(Tss, tss_iopb);

impl Tss {

    funcs!(tss_iomb, u16);
    funcs_vec!(tss_rsp, u64);
    funcs_vec!(tss_ist, u64);
}