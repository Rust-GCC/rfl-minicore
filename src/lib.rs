#![feature(
    no_core,
    lang_items,
    rustc_attrs,
    decl_macro,
    prelude_import,
    intrinsics,
    staged_api
)]
#![no_core]

pub mod marker {
    #[lang = "copy"]
    pub trait Copy {}
}

#[lang = "sized"]
pub trait Sized {}

#[prelude_import]
#[allow(unused)]
use prelude::rust_2021::*;

mod prelude {
    pub mod rust_2021 {
        pub use crate::option::Option::{self, None, Some};
        pub use crate::result::Result::{self, Err, Ok};

        pub use crate::macros::*;
    }
}

pub mod option {
    pub enum Option<T> {
        Some(T),
        None,
    }
}

pub mod result {
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

pub mod ffi {
    #[repr(u8)]
    pub enum c_void {
        __variant1,
        __variant2,
    }

    #[lang = "CStr"]
    #[repr(transparent)]
    pub struct CStr {
        _inner: [c_char],
    }

    // NOTE: These types are in `std` in Rust 1.49, but RfL expects a more recent version of
    // Rust which defines the FFI types within `core`. So we'll need to patch them in for the version
    // of `core` we use for compiling RfL
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i64;
    pub type c_ulong = u64;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
}

pub mod mem {
    pub const fn size_of<T>() -> usize {
        crate::intrinsics::size_of::<T>()
    }
}

pub mod panicking {
    #[lang = "panic"] // needed by codegen for panic on overflow and other `Assert` MIR terminators
    pub const fn panic(_: &'static str) -> ! {
        // super::intrinsics::abort()
    }

    pub const fn panic_str(_: &str) -> ! {
        // super::intrinsics::abort()
    }
}

pub mod intrinsics {
    extern "rust-intrinsic" {
        #[rustc_const_stable(feature = "const_size_of", since = "1.40.0")]
        pub fn size_of<T>() -> usize;

        #[rustc_const_stable(feature = "const_abort", since = "1.40.0")]
        pub fn abort() -> !;
    }
}

#[macro_use]
pub mod macros {
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! assert {
        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};
        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};
    }

    #[macro_export]
    macro_rules! panic {
        () => {
            $crate::panic!("explicit panic")
        };
        ($msg:literal $(,)?) => {
            $crate::panicking::panic($msg)
        };
        ($msg:expr $(,)?) => {
            $crate::panicking::panic_str($msg)
        }; // NOTE: Do we need it?
           // ($fmt:expr, $($arg:tt)+) => (
           //     $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))
           // );
    }
}
