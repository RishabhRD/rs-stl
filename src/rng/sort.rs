// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, OutputRange, RandomAccessRange, SemiOutputRange};

/// Unstable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts rng in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are NOT preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - Unconventional name `sort_range_by` is used to avoid name clash with `sort_by`
///   of rust stdlib.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 5, 2, 1, 4];
/// rng::sort_range_by(&mut arr, |x, y| x < y);
/// assert!(arr.equals(&[1, 1, 2, 4, 5]));
///
/// let mut arr = [1, 5, 2, 1, 4];
/// arr.sort_range_by(|x, y| x < y);
/// assert!(arr.equals(&[1, 1, 2, 4, 5]));
/// ```
pub fn sort_range_by<Range, Compare>(rng: &mut Range, is_less: Compare)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::sort_range_by(rng, start, end, is_less)
}

/// Unstable sort: sorts range in non-decreasing order.
///
/// # Precondition
///
/// # Postcondition
///   - sorts range in non-decreasing order.
///   - Relative order of equivalent elements are NOT preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - Unconventional name `sort_range` is used to avoid name clash with `sort`
///   of rust stdlib.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 5, 2, 1, 4];
/// rng::sort_range(&mut arr);
/// assert!(arr.equals(&[1, 1, 2, 4, 5]));
///
/// let mut arr = [1, 5, 2, 1, 4];
/// arr.sort_range();
/// assert!(arr.equals(&[1, 1, 2, 4, 5]));
/// ```
pub fn sort_range<Range>(rng: &mut Range)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::sort_range(rng, start, end)
}

/// Stable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts rng in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - The algorithm allocates O(n) buffer for achieving O(n.log2(n)) time complexity.
///   If memory allocation is a problem, consider using `stable_sort_by_no_alloc`.
///   `stable_sort_by_no_alloc` doesn't do any buffer allocation, however provides
///   O(n.log2^2(n)) time complexity.
/// - The algorithm requires `OutputRange` trait to read and write from/to
///   buffer. If only `SemiOutputRange` is available, consider using
///   `stable_sort_by_no_alloc` with given tradeoffs.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// rng::stable_sort_by(&mut arr, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// arr.stable_sort_by(|x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
/// ```
pub fn stable_sort_by<Range, Compare>(rng: &mut Range, is_less: Compare)
where
    Range: RandomAccessRange + OutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_sort_by(rng, start, end, is_less);
}

/// Stable sort: sorts range in non-decreasing order.
///
/// # Precondition
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - The algorithm provides O(n.log2^2(n)) time complexity. If memory allocation
///   is not a problem, consider using `stable_sort_by` algorithm.
/// - The algorithm just requires `SemiOutputRange` for mutation instead of
///   `OutputRange`, that might be desirable in some situations.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// rng::stable_sort(&mut arr);
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// arr.stable_sort();
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
/// ```
pub fn stable_sort<Range>(rng: &mut Range)
where
    Range: RandomAccessRange + OutputRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_sort(rng, start, end);
}

/// Stable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts rng in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - The algorithm provides O(n.log2^2(n)) time complexity. If memory allocation
///   is not a problem, consider using `stable_sort` algorithm.
/// - The algorithm just requires `SemiOutputRange` for mutation instead of
///   `OutputRange`, that might be desirable in some situations.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// rng::stable_sort_by_no_alloc(&mut arr, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// arr.stable_sort_by_no_alloc(|x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
/// ```
pub fn stable_sort_by_no_alloc<Range, Compare>(
    rng: &mut Range,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_sort_by_no_alloc(rng, start, end, is_less);
}

/// Stable sort: sorts range in non-decreasing order.
///
/// # Precondition
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - The algorithm allocates O(n) buffer for achieving O(n.log2(n)) time complexity.
///   If memory allocation is a problem, consider using `stable_sort_no_alloc`.
///   `stable_sort_no_alloc` doesn't do any buffer allocation, however provides
///   O(n.log2^2(n)) time complexity.
/// - The algorithm requires `OutputRange` trait to read and write from/to
///   buffer. If only `SemiOutputRange` is available, consider using
///   `stable_sort_no_alloc` with given tradeoffs.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// rng::stable_sort_no_alloc(&mut arr);
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// arr.stable_sort_no_alloc();
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
/// ```
pub fn stable_sort_no_alloc<Range>(rng: &mut Range)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_sort_no_alloc(rng, start, end);
}

pub mod infix {
    use crate::{rng, OutputRange, RandomAccessRange, SemiOutputRange};

    /// `sort_range`, `sort_range_by`, `stable_sort_no_alloc`, `stable_sort_by_no_alloc`.
    pub trait STLSortExt: RandomAccessRange + SemiOutputRange {
        fn sort_range_by<Compare>(&mut self, is_less: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::sort_range_by(self, is_less)
        }

        fn sort_range(&mut self)
        where
            Self::Element: Ord,
        {
            rng::sort_range(self)
        }

        fn stable_sort_by_no_alloc<Compare>(&mut self, is_less: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::stable_sort_by_no_alloc(self, is_less);
        }

        fn stable_sort_no_alloc(&mut self)
        where
            Self::Element: Ord,
        {
            rng::stable_sort_no_alloc(self);
        }
    }

    impl<R> STLSortExt for R where R: RandomAccessRange + SemiOutputRange + ?Sized {}

    /// `stable_sort`, `stable_sort_by`.
    pub trait STLSortOutputExt: RandomAccessRange + OutputRange {
        fn stable_sort_by<Compare>(&mut self, is_less: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::stable_sort_by(self, is_less);
        }

        fn stable_sort(&mut self)
        where
            Self::Element: Ord,
        {
            rng::stable_sort(self);
        }
    }

    impl<R> STLSortOutputExt for R where R: RandomAccessRange + OutputRange + ?Sized {}
}
