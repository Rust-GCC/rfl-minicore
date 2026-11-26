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

    // NOTE: This is also not from Rust 1.49 but from a more recent version of core
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
    use crate::clone::Clone;
    use crate::marker::{Copy, Sized};

    pub const fn size_of<T>() -> usize {
        crate::intrinsics::size_of::<T>()
    }

    #[stable(feature = "manually_drop", since = "1.20.0")]
    #[lang = "manually_drop"]
    #[derive(Copy, Clone)]
    // , Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct ManuallyDrop<T: ?Sized> {
        _value: T,
    }

    #[stable(feature = "maybe_uninit", since = "1.36.0")]
    // Lang item so we can wrap other types in it. This is useful for generators.
    #[lang = "maybe_uninit"]
    #[derive(Copy)]
    #[repr(transparent)]
    pub union MaybeUninit<T> {
        uninit: (),
        value: ManuallyDrop<T>,
    }

    #[stable(feature = "maybe_uninit", since = "1.36.0")]
    impl<T: Copy> Clone for MaybeUninit<T> {
        #[inline(always)]
        fn clone(&self) -> Self {
            // Not calling `T::clone()`, we cannot know if we are initialized enough for that.
            *self
        }
    }
}

pub mod panicking {
    #[lang = "panic"] // needed by codegen for panic on overflow and other `Assert` MIR terminators
    pub const fn panic(_: &'static str) -> ! {
        // FIXME: We need Rust-GCC/gccrs#5118 to be fixed before this can be implemented
        // properly, otherwise asserting in const contexts triggers an error about calling non-const
        // functions from const contexts.
        // FIXME: Display a message for now?

        // super::intrinsics::abort()
    }

    pub const fn panic_str(_: &str) -> ! {
        // FIXME: Likewise ^

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

pub mod num {}

pub mod marker {
    use crate::clone::Clone;

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "sized"]
    #[rustc_on_unimplemented(
        message = "the size for values of type `{Self}` cannot be known at compilation time",
        label = "doesn't have a size known at compile-time"
    )]
    #[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
    #[rustc_specialization_trait]
    pub trait Sized {
        // Empty.
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "copy"]
    // FIXME(matthewjasper) This allows copying a type that doesn't implement
    // `Copy` because of unsatisfied lifetime bounds (copying `A<'_>` when only
    // `A<'static>: Copy` and `A<'_>: Clone`).
    // We have this attribute here for now only because there are quite a few
    // existing specializations on `Copy` that already exist in the standard
    // library, and there's no way to safely have this behavior right now.
    #[rustc_unsafe_specialization_marker]
    pub trait Copy: Clone {
        // Empty.
    }

    #[lang = "phantom_data"]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub struct PhantomData<T: ?Sized>;
}

pub mod clone {
    use crate::marker::Sized;

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "clone"]
    pub trait Clone: Sized {
        /// Returns a copy of the value.
        ///
        /// # Examples
        ///
        /// ```
        /// let hello = "Hello"; // &str implements Clone
        ///
        /// assert_eq!("Hello", hello.clone());
        /// ```
        #[stable(feature = "rust1", since = "1.0.0")]
        #[must_use = "cloning is often expensive and is not expected to have side effects"]
        fn clone(&self) -> Self;

        /// Performs copy-assignment from `source`.
        ///
        /// `a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality,
        /// but can be overridden to reuse the resources of `a` to avoid unnecessary
        /// allocations.
        #[inline]
        #[stable(feature = "rust1", since = "1.0.0")]
        fn clone_from(&mut self, source: &Self) {
            *self = source.clone()
        }
    }

    /// Derive macro generating an impl of the trait `Clone`.
    #[rustc_builtin_macro]
    #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
    #[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
    pub macro Clone($item:item) {
        /* compiler built-in */
    }
}

pub mod convert {
    use crate::clone::Clone;

    #[stable(feature = "convert_infallible", since = "1.34.0")]
    #[derive(Copy)]
    pub enum Infallible {}

    #[stable(feature = "convert_infallible", since = "1.34.0")]
    impl Clone for Infallible {
        fn clone(&self) -> Infallible {
            match *self {}
        }
    }
}

pub mod cell {
    use crate::marker::Sized;

    #[lang = "unsafe_cell"]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[repr(transparent)]
    #[repr(no_niche)] // rust-lang/rust#68303.
    pub struct UnsafeCell<T: ?Sized> {
        _value: T,
    }
}

pub mod pin {
    #[stable(feature = "pin", since = "1.33.0")]
    #[lang = "pin"]
    #[fundamental]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Pin<P> {
        _pointer: P,
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

    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! concat {
        ($($e:expr),* $(,)?) => {{ /* compiler built-in */ }};
    }

    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! stringify {
        ($($t:tt)*) => {
            /* compiler built-in */
        };
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
        };
        ($fmt:expr, $($arg:tt)+) => {
            // NOTE: This is wrong, but we don't care since we just want to abort
            $crate::panicking::panic_str($fmt)
        };
    }
}
