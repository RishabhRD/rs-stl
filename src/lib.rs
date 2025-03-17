// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! # STL module
//!
//! The `stl` module provides core traits that define regularity, ranges, etc.
//! This module also provides implementation of those traits for many commmon
//! data structures. Currently supported data structures are:
//! - `[T;n]` (array)
//! - `[T]` (slice)
//! - `Vec<T>` (Vec)

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

#[doc(hidden)]
pub(crate) mod std_impl;
