//! # STD3
//! STD3 ISN'T FINISHED YET, IF YOU WOULD LIKE TO HELP, PLEASE DO SO
//! [HERE](https://github.com/AtomicGamer9523/std3)
//!
//! The Rust Standard Library is the foundation of portable Rust software, a
//! set of minimal and battle-tested shared abstractions for the [broader Rust
//! ecosystem][crates.io]. It offers core types, like [`Vec<T>`] and
//! [`Option<T>`], library-defined [operations on language
//! primitives](#primitives), [standard macros](#macros), [I/O] and
//! [multithreading], among [many other things][other].
//!
//! `core` is available to all Rust crates by default. Therefore, the
//! standard library can be accessed in [`use`] statements through the path
//! `core`, as in [`use core::env`].
//!
//! # How to read this documentation
//!
//! If you already know the name of what you are looking for, the fastest way to
//! find it is to use the <a href="#" onclick="focusSearchBar();">search
//! bar</a> at the top of the page.
//!
//! Otherwise, you may want to jump to one of these useful sections:
//!
//! * [`core::*` modules](#modules)
//! * [Primitive types](#primitives)
//! * [Standard macros](#macros)
//! * [The Rust Prelude]
//!
//! If this is your first time, the documentation for the standard library is
//! written to be casually perused. Clicking on interesting things should
//! generally lead you to interesting places. Still, there are important bits
//! you don't want to miss, so read on for a tour of the standard library and
//! its documentation!
//!
//! Once you are familiar with the contents of the standard library you may
//! begin to find the verbosity of the prose distracting. At this stage in your
//! development you may want to press the `[-]` button near the top of the
//! page to collapse it into a more skimmable view.
//!
//! While you are looking at that `[-]` button also notice the `source`
//! link. Rust's API documentation comes with the source code and you are
//! encouraged to read it. The standard library source is generally high
//! quality and a peek behind the curtains is often enlightening.
//!
//! # What is in the standard library documentation?
//!
//! First of all, The Rust Standard Library is divided into a number of focused
//! modules, [all listed further down this page](#modules). These modules are
//! the bedrock upon which all of Rust is forged, and they have mighty names
//! like [`core::slice`] and [`core::cmp`]. Modules' documentation typically
//! includes an overview of the module along with examples, and are a smart
//! place to start familiarizing yourself with the library.
//!
//! Second, implicit methods on [primitive types] are documented here. This can
//! be a source of confusion for two reasons:
//!
//! 1. While primitives are implemented by the compiler, the standard library
//!    implements methods directly on the primitive types (and it is the only
//!    library that does so), which are [documented in the section on
//!    primitives](#primitives).
//! 2. The standard library exports many modules *with the same name as
//!    primitive types*. These define additional items related to the primitive
//!    type, but not the all-important methods.
//!
//! So for example there is a [page for the primitive type
//! `i32`](primitive::i32) that lists all the methods that can be called on
//! 32-bit integers (very useful), and there is a [page for the module
//! `core::i32`] that documents the constant values [`MIN`] and [`MAX`] (rarely
//! useful).
//!
//! Note the documentation for the primitives [`str`] and [`[T]`][prim@slice] (also
//! called 'slice'). Many method calls on [`String`] and [`Vec<T>`] are actually
//! calls to methods on [`str`] and [`[T]`][prim@slice] respectively, via [deref
//! coercions][deref-coercions].
//!
//! Third, the standard library defines [The Rust Prelude], a small collection
//! of items - mostly traits - that are imported into every module of every
//! crate. The traits in the prelude are pervasive, making the prelude
//! documentation a good entry point to learning about the library.
//!
//! And finally, the standard library exports a number of standard macros, and
//! [lists them on this page](#macros) (technically, not all of the standard
//! macros are defined by the standard library - some are defined by the
//! compiler - but they are documented here the same). Like the prelude, the
//! standard macros are imported by default into all crates.
//!
//! # Contributing changes to the documentation
//!
//! Check out the rust contribution guidelines [here](
//! https://rustc-dev-guide.rust-lang.org/contributing.html#writing-documentation).
//! The source for this documentation can be found on
//! [GitHub](https://github.com/rust-lang/rust).
//! To contribute changes, make sure you read the guidelines first, then submit
//! pull-requests for your suggested changes.
//!
//! Contributions are appreciated! If you see a part of the docs that can be
//! improved, submit a PR, or chat with us first on [Discord][rust-discord]
//! #docs.
//!
//! # A Tour of The Rust Standard Library
//!
//! The rest of this crate documentation is dedicated to pointing out notable
//! features of The Rust Standard Library.
//!
//! ## Containers and collections
//!
//! The [`option`] and [`result`] modules define optional and error-handling
//! types, [`Option<T>`] and [`Result<T, E>`]. The [`iter`] module defines
//! Rust's iterator trait, [`Iterator`], which works with the [`for`] loop to
//! access collections.
//!
//! The standard library exposes three common ways to deal with contiguous
//! regions of memory:
//!
//! * [`Vec<T>`] - A heap-allocated *vector* that is resizable at runtime.
//! * [`[T; N]`][prim@array] - An inline *array* with a fixed size at compile time.
//! * [`[T]`][prim@slice] - A dynamically sized *slice* into any other kind of contiguous
//!   storage, whether heap-allocated or not.
//!
//! Slices can only be handled through some kind of *pointer*, and as such come
//! in many flavors such as:
//!
//! * `&[T]` - *shared slice*
//! * `&mut [T]` - *mutable slice*
//! * [`Box<[T]>`][owned slice] - *owned slice*
//!
//! [`str`], a UTF-8 string slice, is a primitive type, and the standard library
//! defines many methods for it. Rust [`str`]s are typically accessed as
//! immutable references: `&str`. Use the owned [`String`] for building and
//! mutating strings.
//!
//! For converting to strings use the [`format!`] macro, and for converting from
//! strings use the [`FromStr`] trait.
//!
//! Data may be shared by placing it in a reference-counted box or the [`Rc`]
//! type, and if further contained in a [`Cell`] or [`RefCell`], may be mutated
//! as well as shared. Likewise, in a concurrent setting it is common to pair an
//! atomically-reference-counted box, [`Arc`], with a [`Mutex`] to get the same
//! effect.
//!
//! The [`collections`] module defines maps, sets, linked lists and other
//! typical collection types, including the common [`HashMap<K, V>`].
//!
//! ## Platform abstractions and I/O
//!
//! Besides basic data types, the standard library is largely concerned with
//! abstracting over differences in common platforms, most notably Windows and
//! Unix derivatives.
//!
//! Common types of I/O, are defined in the [`io`] module.
//!
//! The [`thread`] module contains Rust's threading abstractions. [`sync`]
//! contains further primitive shared memory types, including [`atomic`] and
//! [`mpsc`], which contains the channel types for message passing.
//!
//! [I/O]: io
//! [`MIN`]: i32::MIN
//! [`MAX`]: i32::MAX
//! [page for the module `core::i32`]: crate::i32
//! [The Rust Prelude]: prelude
//! [`Arc`]: sync::Arc
//! [owned slice]: boxed
//! [`Cell`]: cell::Cell
//! [`FromStr`]: str::FromStr
//! [`HashMap<K, V>`]: collections::HashMap
//! [`Mutex`]: sync::Mutex
//! [`Option<T>`]: option::Option
//! [`Rc`]: rc::Rc
//! [`RefCell`]: cell::RefCell
//! [`Result<T, E>`]: result::Result
//! [`Vec<T>`]: vec::Vec
//! [`atomic`]: sync::atomic
//! [`for`]: ../book/ch03-05-control-flow.html#looping-through-a-collection-with-for
//! [`str`]: prim@str
//! [`mpsc`]: sync::mpsc
//! [`core::cmp`]: cmp
//! [`core::slice`]: mod@slice
//! [`use core::env`]: env/index.html
//! [`use`]: ../book/ch07-02-defining-modules-to-control-scope-and-privacy.html
//! [crates.io]: https://crates.io
//! [deref-coercions]: ../book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
//! [multithreading]: thread
//! [other]: #what-is-in-the-standard-library-documentation
//! [primitive types]: ../book/ch03-02-data-types.html
//! [rust-discord]: https://discord.gg/rust-lang
//! [array]: prim@array
//! [slice]: prim@slice

#![no_std]
#![feature(staged_api)]
#![cfg_attr(not(feature = "restricted-std"), stable(feature = "std3", since = "0.1.23"))]
#![cfg_attr(feature = "restricted-std", unstable(feature = "restricted_std3", issue = "none"))]


// Linting
#![doc(html_logo_url = "https://www.api.linkrbot.com/cdn/std3.png")]
#![warn(deprecated_in_future)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(explicit_outlives_requirements)]
#![allow(unknown_lints)]
#![needs_panic_runtime]
#![cfg_attr(not(bootstrap), deny(ffi_unwind_calls))]
#![allow(unused_features)]
#![cfg_attr(test, feature(internal_output_capture, print_internals, update_panic_count, rt))]
#![deny(rustc::existing_doc_keyword)]


// Features
#![feature(ready_macro)]
#![feature(alloc_error_handler)]
#![feature(allocator_internals)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
#![feature(box_syntax)]
#![feature(c_unwind)]
#![feature(cfg_target_thread_local)]
#![feature(concat_idents)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(decl_macro)]
#![feature(deprecated_suggestion)]
#![feature(doc_cfg)]
#![feature(doc_cfg_hide)]
#![feature(doc_masked)]
#![feature(doc_notable_trait)]
#![feature(dropck_eyepatch)]
#![feature(exhaustive_patterns)]
#![feature(intra_doc_pointers)]
#![feature(label_break_value)]
#![feature(lang_items)]
#![feature(let_chains)]
#![feature(let_else)]
#![feature(linkage)]
#![feature(min_specialization)]
#![feature(must_not_suspend)]
#![feature(needs_panic_runtime)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(platform_intrinsics)]
#![feature(prelude_import)]
#![feature(rustc_attrs)]
#![feature(rustdoc_internals)]
#![feature(thread_local)]
#![feature(try_blocks)]
#![feature(array_error_internals)]
#![feature(atomic_mut_ptr)]
#![feature(char_error_internals)]
#![feature(char_internals)]
#![feature(core_intrinsics)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(cstr_internals)]
#![feature(duration_checked_float)]
#![feature(duration_constants)]
#![feature(exact_size_is_empty)]
#![feature(extend_one)]
#![feature(float_minimum_maximum)]
#![feature(hasher_prefixfree_extras)]
#![feature(hashmap_internals)]
#![feature(int_error_internals)]
#![feature(is_some_with)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_write_slice)]
#![feature(mixed_integer_ops)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(panic_can_unwind)]
#![feature(panic_info_message)]
#![feature(panic_internals)]
#![feature(portable_simd)]
#![feature(prelude_2024)]
#![feature(provide_any)]
#![feature(ptr_as_uninit)]
#![feature(raw_os_nonzero)]
#![feature(slice_internals)]
#![feature(slice_ptr_get)]
#![feature(std_internals)]
#![feature(str_internals)]
#![feature(strict_provenance)]
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(get_mut_unchecked)]
#![feature(map_try_insert)]
#![feature(new_uninit)]
#![feature(thin_box)]
#![feature(try_reserve_kind)]
#![feature(vec_into_raw_parts)]
#![feature(slice_concat_trait)]
#![feature(assert_matches)]
#![feature(async_iterator)]
#![feature(c_variadic)]
#![feature(cfg_accessible)]
#![feature(cfg_eval)]
#![feature(concat_bytes)]
#![feature(const_format_args)]
#![feature(core_panic)]
#![feature(custom_test_frameworks)]
#![feature(edition_panic)]
#![feature(format_args_nl)]
#![feature(log_syntax)]
#![feature(once_cell)]
#![feature(saturating_int_impl)]
#![feature(stdsimd)]
#![feature(test)]
#![feature(trace_macros)]
#![feature(core_ffi_c)]
#![feature(poll_ready)]





#[doc(hidden)] extern crate alloc as __alloc;
#[doc(hidden)] extern crate core as __core;
#[doc(hidden)] extern crate spin as __spin;
#[stable(feature = "std3", since = "0.1.23")]
pub extern crate bootloader;





include!("keywords.rs");
include!("primitives.rs");





#[unstable(feature = "env", reason = "not yet implemeted in rinux", issue = "none")]
/// # Thread management
/// ## **NOT IMPLEMENTED**
pub mod env;
#[unstable(feature = "error", reason = "not yet implemeted in rinux", issue = "none")]
/// # Thread management
/// ## **NOT IMPLEMENTED**
pub mod error;
#[unstable(feature = "io", reason = "not yet implemeted in rinux", issue = "none")]
/// # Thread management
/// ## **NOT IMPLEMENTED**
pub mod io;
#[stable(feature = "std3", since = "0.1.23")]
pub mod os;
#[unstable(feature = "path", reason = "not yet implemeted in rinux", issue = "none")]
/// # Thread management
/// ## **NOT IMPLEMENTED**
pub mod path;
#[unstable(feature = "process", reason = "not yet implemeted in rinux", issue = "none")]
/// # Thread management
/// ## **NOT IMPLEMENTED**
pub mod process;
#[unstable(feature = "thread", reason = "not yet implemeted in rinux", issue = "none")]
/// # Thread management
/// ## **NOT IMPLEMENTED**
pub mod thread;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::alloc;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::any;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::arch;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::array;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::ascii;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::borrow;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::boxed;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::cell;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::char;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::clone;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::cmp;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::collections;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::convert;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::default;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::f32;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::f64;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::ffi;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::fmt;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::future;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::hash;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::hint;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::i8;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::i16;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::i32;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::i64;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::i128;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::isize;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::iter;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::marker;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::mem;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::num;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::ops;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::option;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::pin;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::prelude;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::primitive;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::ptr;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::rc;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::result;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::slice;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::str;
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::string;
#[allow(missing_docs)]
#[stable(feature = "std3", since = "0.1.23")]
pub mod sync {
    #[stable(feature = "std3", since = "0.1.23")]
    pub use __spin::{
        barrier::{
            BarrierWaitResult,
            Barrier
        },
        mutex::{
            MutexGuard,
            Mutex
        },
        rwlock::{
            RwLockWriteGuard,
            RwLockReadGuard,
            RwLock
        },
        Once,
        Lazy
    };
    #[stable(feature = "std3", since = "0.1.23")]
    pub use __alloc::sync::{Arc, Weak};
    #[stable(feature = "std3", since = "0.1.23")]
    pub use __core::sync::atomic;
}
#[allow(missing_docs)]
#[stable(feature = "std3", since = "0.1.23")]
pub mod task {
    #[stable(feature = "std3", since = "0.1.23")]
    pub use __core::task::{ready, Ready, Context, RawWaker, RawWakerVTable, Waker, Poll};
    #[stable(feature = "std3", since = "0.1.23")]
    pub use __alloc::task::Wake;
}
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::time;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::u8;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::u16;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::u32;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::u64;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::u128;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::usize;






#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::vec;
#[stable(feature = "std3", since = "0.1.23")]
pub use __core::{
    assert,assert_eq,assert_ne,cfg,column,compile_error,concat,debug_assert,debug_assert_eq,
    debug_assert_ne,env,file,format_args,include,include_bytes,include_str,line,log_syntax,
    matches,module_path,option_env,panic,stringify,todo,unimplemented,unreachable,write,writeln
};
#[stable(feature = "std3", since = "0.1.23")]
pub use __alloc::format;
#[stable(feature = "std3", since = "0.1.23")]
#[allow(deprecated, deprecated_in_future)]
pub use __core::r#try;




// #[cfg(feature = "rinux")]
#[doc(hidden)] extern crate std3proc;

// #[cfg(feature = "rinux")]
#[stable(feature = "std3", since = "0.1.23")]
/// Useful tools for dealing with rinux
pub mod rinux {
    #[stable(feature = "std3", since = "0.1.23")]
    pub use std3proc::main;
}



#[cfg(feature = "reexport")]
pub mod __reexports {
    #[allow(pub_use_of_private_extern_crate)]
    #[cfg(feature = "reexport")]
    pub use __spin as spin;
    #[cfg(feature = "reexport")]
    pub use __spin::{lock_api,relax};

    #[cfg(feature = "reexport")]
    pub extern crate lazy_static;

    #[cfg(feature = "volatile")]
    pub extern crate volatile;

    #[cfg(feature = "x86_64")]
    pub extern crate x86_64;

    #[cfg(feature = "uart_16550")]
    pub extern crate uart_16550;
}
