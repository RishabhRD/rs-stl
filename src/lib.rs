// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! # STL module
//!
//! The `stl` module provides core traits that define regularity, ranges, etc.
//! This module also provides implementation of those traits for many commmon
//! data structures. Currently supported data structures are:
//! - `[T;n]` (array)
//! - `[T]` (slice)
//! - `Vec<T>` (Vec)

#[doc(hidden)]
pub mod algo;
#[doc(hidden)]
pub mod array_slice;
#[doc(hidden)]
pub mod core;
#[doc(hidden)]
pub mod iter;
#[doc(hidden)]
pub mod slice;
#[doc(hidden)]
pub mod trait_impl;
#[doc(hidden)]
pub mod util;

#[doc(inline)]
pub use array_slice::*;
#[doc(inline)]
pub use core::*;
#[doc(inline)]
pub use iter::*;
#[doc(inline)]
pub use slice::*;
