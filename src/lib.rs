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
//!
//! In upcoming future more data structures will be supported. If supporting
//! specific data structure would not be possible due to some design choices,
//! alternate data structures would be provided for the same.

pub mod algo;
pub mod array;
#[doc(hidden)]
pub mod core;
pub mod rng;
pub mod slice;
pub mod vec;

#[doc(inline)]
pub use core::*;
