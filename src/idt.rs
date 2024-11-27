use crate::funcs;

#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct IdtDescriptor {
    low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    middle: u16,
    high: u32,
    zero: u32,
}

impl IdtDescriptor {
    funcs!(low, u16);
    funcs!(selector, u16);
    funcs!(ist, u8);
    funcs!(type_attr, u8);
    funcs!(middle, u16);
    funcs!(high, u32);
    funcs!(zero, u32);

    pub fn new() -> Self {
        IdtDescriptor::default()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.low = 0;
        self.selector = 0;
        self.ist = 0;
        self.type_attr = 0;
        self.middle = 0;
        self.high = 0;
        self.zero = 0;
        self
    }

    pub fn set_idt_addr(&mut self, addr: usize) -> &mut Self {
        self.low = (addr & 0xFFFF) as u16;
        self.middle = ((addr >> 16) & 0xFFFF) as u16;
        self.high = ((addr >> 32) & 0xFFFFFFFF) as u32;
        self
    }
}

impl AsRef<[u8]> for IdtDescriptor {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const _ as *const u8,
                core::mem::size_of::<IdtDescriptor>(),
            )
        }
    }
}

pub const IDT_ENTRIES: usize = 256;