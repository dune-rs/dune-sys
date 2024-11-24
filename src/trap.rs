use crate::funcs;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct DuneTf {
    /* manually saved, arguments */
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,

    /* saved by C calling conventions */
    rbx: u64,
    rbp: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,

    /* system call number, ret */
    rax: u64,

    /* exception frame */
    err: u32,
    pad1: u32,
    rip: u64,
    cs: u16,
    pad2: [u16; 3],
    rflags: u64,
    rsp: u64,
    ss: u16,
    pad3: [u16; 3],
}

impl DuneTf {
    funcs!(rdi, u64);
    funcs!(rsi, u64);
    funcs!(rdx, u64);
    funcs!(rcx, u64);
    funcs!(r8, u64);
    funcs!(r9, u64);
    funcs!(r10, u64);
    funcs!(r11, u64);
    funcs!(rbx, u64);
    funcs!(rbp, u64);
    funcs!(r12, u64);
    funcs!(r13, u64);
    funcs!(r14, u64);
    funcs!(r15, u64);
    funcs!(rax, u64);
    funcs!(err, u32);
    funcs!(rip, u64);
    funcs!(cs, u16);
    funcs!(rflags, u64);
    funcs!(rsp, u64);
    funcs!(ss, u16);
}

