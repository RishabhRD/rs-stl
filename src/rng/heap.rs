// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, RandomAccessRange};

/// Finds largest range that from start represents a heap wrt comparator.
///
/// # Precondition
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns last position `i` in rng such that
///     rng at `[rng.start(), i)` is a heap with respect to cmp.
///   - Complexity: O(n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// Where n is number of elements in rng.
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2, 6];
///
/// let i = rng::is_heap_until_by(&arr, |x, y| x < y);
/// assert_eq!(i, 7);
///
/// let i = arr.is_heap_until_by(|x, y| x < y);
/// assert_eq!(i, 7);
/// ```
pub fn is_heap_until_by<Range, Compare>(
    rng: &Range,
    cmp: Compare,
) -> Range::Position
where
    Range: RandomAccessRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::is_heap_until_by(rng, rng.start(), rng.end(), cmp)
}

/// Finds largest range that from start represents a heap.
///
/// # Precondition
///   - `[start, end)` reperesents valid position in rng.
///
/// # Postcondition
///   - Returns last position `i` in `[start, end)` of rng such that
///     rng at `[start, i)` is a heap.
///   - Complexity: O(n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// Where n is number of elements in `[start, end)`.
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2, 6];
///
/// let i = rng::is_heap_until(&arr);
/// assert_eq!(i, 7);
///
/// let i = arr.is_heap_until();
/// assert_eq!(i, 7);
/// ```
pub fn is_heap_until<Range>(rng: &Range) -> Range::Position
where
    Range: RandomAccessRange + ?Sized,
    Range::Element: Ord,
{
    algo::is_heap_until(rng, rng.start(), rng.end())
}

/// Returns true if given range is a heap wrt comparator.
///
/// # Precondition
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns true if rng is a heap wrt cmp. Otherwise, returns false.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2];
/// assert!(rng::is_heap_by(&arr, |x, y| x < y));
/// assert!(arr.is_heap_by(|x, y| x < y));
/// ```
pub fn is_heap_by<Range, Compare>(rng: &Range, cmp: Compare) -> bool
where
    Range: RandomAccessRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::is_heap_by(rng, rng.start(), rng.end(), cmp)
}

/// Returns true if given range is a heap.
///
/// # Precondition
///
/// # Postcondition
///   - Returns true if rng is a heap. Otherwise, returns false.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2];
///
/// assert!(rng::is_heap(&arr));
/// assert!(arr.is_heap());
/// ```
pub fn is_heap<Range>(rng: &Range) -> bool
where
    Range: RandomAccessRange + ?Sized,
    Range::Element: Ord,
{
    algo::is_heap(rng, rng.start(), rng.end())
}

pub mod infix {
    use crate::{rng, RandomAccessRange};

    /// `is_heap_until`, `is_heap_until_by`.
    pub trait STLHeapExt: RandomAccessRange {
        fn is_heap_until_by<Compare>(&self, cmp: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn is_heap_until(&self) -> Self::Position
        where
            Self::Element: Ord;

        fn is_heap_by<Compare>(&self, cmp: Compare) -> bool
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn is_heap(&self) -> bool
        where
            Self::Element: Ord;
    }

    impl<R> STLHeapExt for R
    where
        R: RandomAccessRange + ?Sized,
    {
        fn is_heap_until_by<Compare>(&self, cmp: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::is_heap_until_by(self, cmp)
        }

        fn is_heap_until(&self) -> Self::Position
        where
            Self::Element: Ord,
        {
            rng::is_heap_until(self)
        }

        fn is_heap_by<Compare>(&self, cmp: Compare) -> bool
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::is_heap_by(self, cmp)
        }

        fn is_heap(&self) -> bool
        where
            Self::Element: Ord,
        {
            rng::is_heap(self)
        }
    }
}
