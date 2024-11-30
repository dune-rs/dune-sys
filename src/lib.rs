pub mod result;
#[macro_use]
pub mod trap;
#[macro_use]
pub mod debug;
#[macro_use]
pub mod dune;
#[macro_use]
pub mod vmpl;
#[macro_use]
pub mod dev;
#[macro_use]
pub mod idt;
#[macro_use]
pub mod tss;

pub use crate::result::*;
pub use crate::trap::*;
pub use crate::debug::*;
pub use crate::dune::*;
pub use crate::vmpl::*;
pub use crate::dev::*;
pub use crate::idt::*;
pub use crate::tss::*;

/// Generate set/get methods for a given struct field and type

#[macro_export]
macro_rules! funcs {
    ($name: ident, $T: ty) => {
        paste::paste! {
            pub fn [<$name>](&self) -> $T {
                self.$name
            }
            pub fn [<set_ $name>](&mut self, value: $T) -> &mut Self {
                self.$name = value;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! funcs_ref {
    ($name: ident, $T: ty) => {
        paste::paste! {
            pub fn $name(&self) -> &$T {
                &self.$name
            }
            pub fn [<set_ $name>](&mut self, value: $T) -> &mut Self {
                self.$name = value;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! funcs_vec {
    ($name: ident, $T: ty) => {
        paste::paste! {
            pub fn $name(&self, idx: usize) -> $T {
                self.$name[idx]
            }
            pub fn [<set_ $name>](&mut self, idx: usize, val: $T) -> &mut Self {
                self.$name[idx] = val;
                self
            }
        }
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! lg_align {
    ($addr:expr) => {
        ($addr + (1 << 30) - 1) & !((1 << 30) - 1)
    };
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
