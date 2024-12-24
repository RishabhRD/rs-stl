// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! # Ranges module
//!
//! The `rng` module provides a collection of range version of STL algorithms
//! as well as some infix version for some subset of algorithms.
//!
//! If infix version of range algorithm is supported, then it is in following
//! format:
//! ```rust
//! use stl::*;
//! use rng::infix::*;
//!
//! let arr = [1, 2, 3];
//!
//! let cnt = rng::count_if(&arr, |x| x % 2 == 1);
//! assert_eq!(cnt, 2);
//! let cnt = arr.count_if(|x| x % 2 == 1);
//! assert_eq!(cnt, 2);
//! ```
//!
//! For all algorithms it is mentioned in doc if infix version is supported or
//! not for that algorithm. Or open infix module to see all supported infix
//! algorithms.
//!
//! For using infix version of supported algorithms use:
//! ```rust
//! use stl::rng::infix::*;
//! ```

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

#[doc(hidden)]
pub mod minmax;
#[doc(inline)]
pub use minmax::*;

#[doc(hidden)]
pub mod partition;
#[doc(inline)]
pub use partition::*;

#[doc(hidden)]
pub mod heap;
#[doc(inline)]
pub use heap::*;

#[doc(hidden)]
pub mod merge;
#[doc(inline)]
pub use merge::*;

pub mod infix {
    //! Defines extension traits to support infix version of range algorithms.
    //!
    //! For preconditions and postcondition of any algorithm see `rng` module
    //! functions.

    #[doc(inline)]
    pub use super::adjacent_find::infix::*;
    #[doc(inline)]
    pub use super::count::infix::*;
    #[doc(inline)]
    pub use super::equals::infix::*;
    #[doc(inline)]
    pub use super::fill::infix::*;
    #[doc(inline)]
    pub use super::find::infix::*;
    #[doc(inline)]
    pub use super::heap::infix::*;
    #[doc(inline)]
    pub use super::minmax::infix::*;
    #[doc(inline)]
    pub use super::of::infix::*;
    #[doc(inline)]
    pub use super::partition::infix::*;
    #[doc(inline)]
    pub use super::remove::infix::*;
    #[doc(inline)]
    pub use super::replace::infix::*;
    #[doc(inline)]
    pub use super::reverse::infix::*;
    #[doc(inline)]
    pub use super::rotate::infix::*;
    #[doc(inline)]
    pub use super::unique::infix::*;
}
