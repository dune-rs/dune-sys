#[macro_use]
pub mod trap;
#[macro_use]
pub mod debug;
#[macro_use]
pub mod dune;
#[macro_use]
pub mod dev;

pub use crate::trap::*;
pub use crate::debug::*;
pub use crate::dune::*;
pub use crate::dev::*;

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
