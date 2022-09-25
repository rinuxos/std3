//! # STD3
//! STD3 ISN'T FINISHED YET, IF YOU WOULD LIKE TO HELP, PLEASE DO SO [HERE](https://github.com/AtomicGamer9523/std3)
//!
//! The Rust Standard Library is the foundation of portable Rust software, a
//! set of minimal and battle-tested shared abstractions for the [broader Rust
//! ecosystem][crates.io]. It offers core types, like [`Vec<T>`] and
//! [`Option<T>`], library-defined [operations on language
//! primitives](#primitives), [standard macros](#macros), [I/O] and
//! [multithreading], among [many other things][other].
//!
//! `std3` is available to all Rust crates by default. Therefore, the
//! standard library can be accessed in [`use`] statements through the path
//! `std3`, as in [`use std3::env`].
//!
//! # How to read this documentation
//!
//! If you already know the name of what you are looking for, the fastest way to
//! find it is to use the <a href="#" onclick="focusSearchBar();">search
//! bar</a> at the top of the page.
//!
//! Otherwise, you may want to jump to one of these useful sections:
//!
//! * [`std3::*` modules](#modules)
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
//! like [`std3::slice`] and [`std3::cmp`]. Modules' documentation typically
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
//! `std3::i32`] that documents the constant values [`MIN`] and [`MAX`] (rarely
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
//! Common types of I/O, including [files], [TCP], [UDP], are defined in the
//! [`io`], [`fs`], and [`net`] modules.
//!
//! The [`thread`] module contains Rust's threading abstractions. [`sync`]
//! contains further primitive shared memory types, including [`atomic`] and
//! [`mpsc`], which contains the channel types for message passing.
//!
//! [I/O]: io
//! [`MIN`]: i32::MIN
//! [`MAX`]: i32::MAX
//! [page for the module `std3::i32`]: crate::i32
//! [TCP]: net::TcpStream
//! [The Rust Prelude]: prelude
//! [UDP]: net::UdpSocket
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
//! [`std3::cmp`]: cmp
//! [`std3::slice`]: mod@slice
//! [`use std3::env`]: env/index.html
//! [`use`]: ../book/ch07-02-defining-modules-to-control-scope-and-privacy.html
//! [crates.io]: https://crates.io
//! [deref-coercions]: ../book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
//! [files]: fs::File
//! [multithreading]: thread
//! [other]: #what-is-in-the-standard-library-documentation
//! [primitive types]: ../book/ch03-02-data-types.html
//! [rust-discord]: https://discord.gg/rust-lang
//! [array]: prim@array
//! [slice]: prim@slice

#![no_std]
#![doc(html_logo_url = "https://www.api.linkrbot.com/cdn/std3.png")]

#![feature(ready_macro)]
#![feature(never_type)]
#![feature(poll_ready)]



#[doc(hidden)] extern crate alloc as __alloc;
#[doc(hidden)] extern crate core as __core;
#[doc(hidden)] extern crate spin as __spin;

pub use __alloc::alloc;
pub use __core::any;
pub use __core::arch;
pub use __core::array;
pub use __core::ascii;
pub use __alloc::borrow;
pub use __alloc::boxed;
pub use __core::cell;
pub use __core::char;
pub use __core::clone;
pub use __core::cmp;
pub use __alloc::collections;
pub use __core::convert;
pub use __core::default;
pub use __core::f32;
pub use __core::f64;
pub use __core::ffi;
pub use __alloc::fmt;
pub use __core::future;
pub use __core::hash;
pub use __core::hint;
pub use __core::i8;
pub use __core::i16;
pub use __core::i32;
pub use __core::i64;
pub use __core::i128;
pub use __core::isize;
pub use __core::iter;
pub use __core::marker;
pub use __core::mem;
pub use __core::num;
pub use __core::ops;
pub use __core::option;
pub use __core::panic;
pub use __core::pin;
pub use __core::prelude;
pub use __core::primitive;
pub use __core::ptr;
pub use __alloc::rc;
pub use __core::result;
pub use __alloc::slice;
pub use __alloc::str;
pub use __alloc::string;
pub mod sync {
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
    pub use __alloc::sync::{Arc, Weak};
    pub use __core::sync::atomic;
}
pub mod task {
    pub use __core::task::{ready, Ready, Context, RawWaker, RawWakerVTable, Waker, Poll};
    pub use __alloc::task::Wake;
}
pub use __core::time;
pub use __core::u8;
pub use __core::u16;
pub use __core::u32;
pub use __core::u64;
pub use __core::u128;
pub use __core::usize;
pub use __alloc::vec;

mod non_builtin;
pub use non_builtin::*;


#[doc(hidden)]
pub mod __reexports {
    pub use __spin::{lock_api,relax,portable_atomic};
}
