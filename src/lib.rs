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
//! - `str`
//! - `String`

pub mod algo;
pub mod array;
#[doc(hidden)]
pub mod core;
pub mod rng;
pub mod slice;
pub mod str;
#[doc(hidden)]
pub mod util;
pub mod vec;

#[doc(inline)]
pub use core::*;

pub mod view;
pub use view::STLMutableViewExt;
pub use view::STLViewExt;
