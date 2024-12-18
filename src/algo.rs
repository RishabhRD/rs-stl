// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! # Algorithms module
//!
//! The `algo` module provides a collection of STL algorithms. These algorithms
//! explicitly accepts Positions. For more compact algorithm overloads that
//! just accepts ranges, or infix notation (by .) use `rng` module.
//!
//! NOTE: In documentation of any function that accepts position with notation
//! like [start, end), if doc talks in term of full range, it should be considered
//! as rng elements from [start, end) position only.

#[doc(hidden)]
pub mod find;
#[doc(inline)]
pub use find::*;

#[doc(hidden)]
pub mod count;
#[doc(inline)]
pub use count::*;

#[doc(hidden)]
pub mod of;
#[doc(inline)]
pub use of::*;

#[doc(hidden)]
pub mod mismatch;
#[doc(inline)]
pub use mismatch::*;

#[doc(hidden)]
pub mod adjacent_find;
#[doc(inline)]
pub use adjacent_find::*;

#[doc(hidden)]
pub mod equals;
#[doc(inline)]
pub use equals::*;

#[doc(hidden)]
pub mod copy;
#[doc(inline)]
pub use copy::*;

#[doc(hidden)]
pub mod transform;
#[doc(inline)]
pub use transform::*;

#[doc(hidden)]
pub mod replace;
#[doc(inline)]
pub use replace::*;

#[doc(hidden)]
pub mod fill;
#[doc(inline)]
pub use fill::*;

#[doc(hidden)]
pub mod remove;
#[doc(inline)]
pub use remove::*;

#[doc(hidden)]
pub mod unique;
#[doc(inline)]
pub use unique::*;

#[doc(hidden)]
pub mod reverse;
#[doc(inline)]
pub use reverse::*;

#[doc(hidden)]
pub mod rotate;
#[doc(inline)]
pub use rotate::*;
