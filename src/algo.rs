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

#[doc(hidden)]
pub mod minmax;
#[doc(inline)]
pub use minmax::*;

#[doc(hidden)]
pub mod partition;
#[doc(inline)]
pub use partition::{
    is_partitioned, partition, partition_point, stable_partition,
    stable_partition_no_alloc,
};

#[doc(hidden)]
pub mod sort;
#[doc(inline)]
pub use sort::{
    is_sorted, is_sorted_by, is_sorted_until, is_sorted_until_by, nth_element,
    nth_element_by, partial_sort, partial_sort_by, partial_sort_copy,
    partial_sort_copy_by, sort_range, sort_range_by, stable_sort,
    stable_sort_by, stable_sort_by_no_alloc, stable_sort_no_alloc,
};

#[doc(hidden)]
pub mod heap;
#[doc(inline)]
pub use heap::{
    is_heap, is_heap_by, is_heap_until, is_heap_until_by, make_heap,
    make_heap_by, pop_heap, pop_heap_by, push_heap, push_heap_by, sort_heap,
    sort_heap_by,
};

#[doc(hidden)]
pub mod merge;
#[doc(inline)]
pub use merge::*;

#[doc(hidden)]
pub mod swap_ranges;
#[doc(inline)]
pub use swap_ranges::*;

#[doc(hidden)]
pub mod binary_search;
#[doc(inline)]
pub use binary_search::*;

#[doc(hidden)]
pub mod numeric;
#[doc(inline)]
pub use numeric::*;

#[doc(hidden)]
pub mod for_each;
#[doc(inline)]
pub use for_each::*;
