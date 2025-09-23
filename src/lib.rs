// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! # STL module
//!
//! The `stl` module provides formal definition of "Collections" in terms of traits.
//! Based on primitives exposed by traits, the module provides many generic algorithms
//! for types satisfying "Collection" traits.
//!
//! The module also implements the "Collection" related traits for stdlib
//! data structures. Currently "Collection" traits have been implemented for:
//! - `[T;n]` (array)
//! - `[T]` (slice)
//! - `Vec<T>` (Vec)
//! - `Option<T>` (Option)
//! - `Range<T>` (a..b) where `T` is a signed/unsigned integer type.
//! - `RangeInclusive<T>` (a..=b) where `T` is a signed/unsigned integer type.

mod core;
#[doc(inline)]
pub use core::*;

mod slice;
#[doc(inline)]
pub use slice::*;

mod slice_mut;
#[doc(inline)]
pub use slice_mut::*;

mod algo;
#[doc(inline)]
pub use algo::*;

/// All the collections exposed from library.
pub mod collections;

/// All the iterators exposed from library.
pub mod iterators;

#[doc(hidden)]
pub(crate) mod std_impl;

/// Proxy Reference to temporary value.
pub mod value_ref;

mod util;
pub(crate) use util::*;

mod exec;
pub(crate) use exec::*;
