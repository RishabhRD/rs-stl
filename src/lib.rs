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
//!
//! ## Design
//!
//! rs-stl is port of C++ STL to rust. C++ STL works over abstraction of iterators.
//! Iterators in C++ are generalization of pointers. However, pointers have
//! reference semantics so does iterators. Most STL algorithms need 2 iterators
//! to work upon. However, this model can't be adopted to rust. As an example,
//! in reverse example algorithm needs 2 mutable iterators to data structure.
//! However, then there would 2 mutable borrows from same data structure at
//! same time. That is not possible in rust.
//!
//! Thus rs-stl works with idea of `Positions`. Positions are generalization of
//! indexes as iterators were generalization of pointers.
//!
//! Considering example of array. In C++ there are 2 ways to traverse array `arr`:
//! 1. Pointer -> *arr, ++arr
//! 2. Indexes -> arr[i], ++i;
//!
//! As iterators are all about abstraction to traverse linear range, a similar
//! alternative is required. The above array example suggests, indexes are the
//! one. Which is generalized and is called `Position` in rs-stl. Position
//! type doesn't need to be integers and can be any type that follows below
//! trait requirements.
//!
//! As from above example, its clear that Position doesn't have reference semantics.
//! Position doesn't borrow from data structure. Thus multiple positions can
//! be passed to algorithms. Also data structures are required to access
//! element at any position. Thus, this solution would not have problem of
//! dangling iterator.
//!
//! rs-stl works with **ranges**. Range models linear sequence of elements.
//!
//! ```text
//!   _ _ _ _ _
//!
//!   ^          ^
//!   |          |
//! start   end/is_end(pos)
//! ```
//!
//! Every range has a `start` position, that is position of first element in range,
//! and an `end` position, that is position just after last element in range. Many
//! times end position is not known upfront, instead end is identified by `rng.is_end(i)`
//! expression.
//!
//! By default, rs-stl assumes that end position is not known upfront. If end position
//! is known upfront, that is an enhanced capability of `BoundedRange`.
//! end position for non-bounded range can be known by advancing start position
//! until it stops satisfying is_end.
//!
//! To get start position in range `rng`, `rng.start()` can be used. For checking
//! if current position is end position use `rng.is_end(pos)`. If range is bounded
//! use `rng.end()` to get the end position.
//!
//! To access any element at position `i` in `rng` do:
//! - `rng.at(&i)` -> for immutable access
//! - `rng.at_mut(&i)` -> for mutable access
//!
//! NOTE: end position can not be accessed for element with above methods.
//!
//! To get to next position from current position `i` in range `rng`,
//! `rng.after(i)` can be used.
//!
//! See the trait docs for more information.
//!
//! ## How to use
//!
//! rs-stl supports following operation with ranges:
//! 1. Algorithms (algo and rng module)
//!
//! Please look at module docs for more information for the same.
//!
//!
//! ### Algorithms
//!
//! Let's take an example of `std::count_if` in rs-stl, how to use this algorithm.
//! 1. algo module
//! 2. rng module
//!
//! algo module contains algorithms which require position start and end to
//! be passed explicitly. Using that:
//! ```rust
//! use stl::*;
//!
//! let arr = [1, 2, 3];
//! let cnt = algo::count_if(&arr, arr.start(), arr.end(), |x| x % 2 == 1);
//! assert_eq!(cnt, 2);
//! ```
//!
//! rng module contains algo module algorithms overload, which doesn't need
//! start and end positions to be passed explicitly.
//! ```rust
//! use stl::*;
//!
//! let arr = [1, 2, 3];
//! let cnt = rng::count_if(&arr, |x| x % 2 == 1);
//! assert_eq!(cnt, 2);
//! ```
//!
//! For many algorithms rng module also provides infix overload inside rng::infix
//! module.
//! ```rust
//! use stl::*;
//! use rng::infix::*;
//!
//! let arr = [1, 2, 3];
//! let cnt = arr.count_if(|x| x % 2 == 1);
//! assert_eq!(cnt, 2);
//! ```
pub mod algo;
pub mod array;
#[doc(hidden)]
pub mod core;
pub mod rng;
pub mod slice;
pub mod vec;

#[doc(inline)]
pub use core::*;
