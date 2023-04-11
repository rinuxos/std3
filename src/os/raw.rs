//! Compatibility module for C platform-specific types. Use [`crate::ffi`] instead.

#[doc(hidden)]
macro_rules! alias_core_ffi {
    ($($t:ident)*) => {$(
        // Make this type alias appear cfg-dependent so that Clippy does not suggest
        // replacing expressions like `0 as c_char` with `0_i8`/`0_u8`. This #[cfg(all())] can be
        // removed after the false positive in https://github.com/rust-lang/rust-clippy/issues/8093
        // is fixed.
        #[cfg(all())]
        #[doc(cfg(all()))]
        #[stable(feature = "std3", since = "0.1.23")]
        pub type $t = crate::__core::ffi::$t;
    )*}
}

alias_core_ffi! {
    c_char c_schar c_uchar
    c_short c_ushort
    c_int c_uint
    c_long c_ulong
    c_longlong c_ulonglong
    c_float
    c_double
    c_void
}