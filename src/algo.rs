// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! # Algorithms module
//!
//! The `algo` module provides STL algorithms.
//!
//! Regarding transformation algorithms, there are 2 kinds of algorithms:
//! 1. eager algorithms: that eagerly transforms the collection.
//! 2. lazy algorithms: that returns a lazy collection which would apply transformation
//!    on element access. These algorithms are suffixed with _lazy.
//!
//! Many algorithms provide their infix versions, every algorithm mentions if
//! infix is supported for that algorithm in doc.

#[doc(hidden)]
pub mod find;
#[doc(inline)]
pub use find::*;

pub mod infix {
    pub use super::find::infix::*;
}
