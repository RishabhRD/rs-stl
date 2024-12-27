// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, RandomAccessRange, SemiOutputRange};

/// Unstable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - cmp follows strict weak ordering relationship, i.e., returns true for
///     cmp(a, b) if a comes before b otherwise false.
///   - Also if cmp(a, b) == false && cmp(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts rng in non-decreasing order by comparator cmp.
///   - Relative order of equivalent elements are NOT preserved.
///   - Complexity: O(n.log2(n)) comparisions.
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
pub fn sort_range_by<Range, Compare>(rng: &mut Range, cmp: Compare)
where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::sort_range_by(rng, start, end, cmp)
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

pub mod infix {
    use crate::{rng, RandomAccessRange, SemiOutputRange};

    /// `sort_range`, `sort_range_by`.
    pub trait STLSortExt: RandomAccessRange + SemiOutputRange {
        fn sort_range_by<Compare>(&mut self, cmp: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone;

        fn sort_range(&mut self)
        where
            Self::Element: Ord;
    }

    impl<R> STLSortExt for R
    where
        R: RandomAccessRange + SemiOutputRange + ?Sized,
    {
        fn sort_range_by<Compare>(&mut self, cmp: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::sort_range_by(self, cmp)
        }

        fn sort_range(&mut self)
        where
            Self::Element: Ord,
        {
            rng::sort_range(self)
        }
    }
}
