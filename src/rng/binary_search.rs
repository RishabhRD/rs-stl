// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, ForwardRange};

/// Returns the position of first element in partitioned range which is not ordered before value
/// wrt comparator.
///
/// # Precondition
///   - rng should be partitioned wrt expression `is_less(rng.at(&i), ele)`.
///
/// # Postcondition
///   - Returns position `i` of first element in rng st `is_less(rng.at(&i), ele) == false`.
///   - If no such element exists, returns rng.end().
///   - Complexity: O(log2(n)) comparisions. If rng is not a RandomAccessRange,
///     then number of position increment is O(n).
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
/// let arr = [2, 1, 4, 8, 7];
///
/// let i = rng::lower_bound_by(&arr, &4, |x, y| x < y);
/// assert_eq!(i, 2);
///
/// let i = arr.lower_bound_by(&4, |x, y| x < y);
/// assert_eq!(i, 2);
/// ```
pub fn lower_bound_by<Range, Compare>(
    rng: &Range,
    ele: &Range::Element,
    is_less: Compare,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::lower_bound_by(rng, rng.start(), rng.end(), ele, is_less)
}

/// Returns the position of first element in partitioned range which is not ordered before value.
///
/// # Precondition
///   - rng should be partitioned wrt expression `rng.at(&i) < ele`.
///
/// # Postcondition
///   - Returns position `i` of first element in rng st `rng.at(&i) >= ele`.
///   - If no such element exists, returns rng.end().
///   - Complexity: O(log2(n)) comparisions. If rng is not a RandomAccessRange,
///     then number of position increment is O(n).
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
/// let arr = [2, 1, 4, 8, 7];
///
/// let i = rng::lower_bound(&arr, &4);
/// assert_eq!(i, 2);
///
/// let i = arr.lower_bound(&4);
/// assert_eq!(i, 2);
/// ```
pub fn lower_bound<Range>(rng: &Range, ele: &Range::Element) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    algo::lower_bound(rng, rng.start(), rng.end(), ele)
}

/// Returns the position of first element in partitioned range which is not ordered after value
/// wrt comparator.
///
/// # Precondition
///   - rng should be partitioned wrt expression `is_less(rng.at(&i), ele)`.
///
/// # Postcondition
///   - Returns position `i` of first element in rng st `is_less(ele, rng.at(&i)) == true`.
///   - If no such element exists, returns rng.end().
///   - Complexity: O(log2(n)) comparisions. If rng is not a RandomAccessRange,
///     then number of position increment is O(n).
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
/// let arr = [2, 1, 4, 8, 7];
///
/// let i = rng::upper_bound_by(&arr, &4, |x, y| x < y);
/// assert_eq!(i, 3);
///
/// let i = arr.upper_bound_by(&4, |x, y| x < y);
/// assert_eq!(i, 3);
/// ```
pub fn upper_bound_by<Range, Compare>(
    rng: &Range,
    ele: &Range::Element,
    is_less: Compare,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::upper_bound_by(rng, rng.start(), rng.end(), ele, is_less)
}

/// Returns the position of first element in partitioned range which is not ordered after value.
///
/// # Precondition
///   - rng should be partitioned wrt expression `rng.at(&i) < ele`.
///
/// # Postcondition
///   - Returns position `i` of first element in rng st `rng.at(&i) > ele`.
///   - If no such element exists, returns end.
///   - Complexity: O(log2(n)) comparisions. If rng is not a RandomAccessRange,
///     then number of position increment is O(n).
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
/// let arr = [2, 1, 4, 8, 7];
///
/// let i = rng::upper_bound(&arr, &4);
/// assert_eq!(i, 3);
///
/// let i = arr.upper_bound(&4);
/// assert_eq!(i, 3);
/// ```
pub fn upper_bound<Range>(rng: &Range, ele: &Range::Element) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    algo::upper_bound(rng, rng.start(), rng.end(), ele)
}

pub mod infix {
    use crate::{rng, ForwardRange};

    /// `lower_bound`, `lower_bound_by`, `upper_bound`, `upper_bound_by`.
    pub trait STLBinarySearchExt: ForwardRange {
        fn lower_bound_by<Compare>(
            &self,
            ele: &Self::Element,
            is_less: Compare,
        ) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn lower_bound(&self, ele: &Self::Element) -> Self::Position
        where
            Self::Element: Ord;

        fn upper_bound_by<Compare>(
            &self,
            ele: &Self::Element,
            is_less: Compare,
        ) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn upper_bound(&self, ele: &Self::Element) -> Self::Position
        where
            Self::Element: Ord;
    }

    impl<R> STLBinarySearchExt for R
    where
        R: ForwardRange + ?Sized,
    {
        fn lower_bound_by<Compare>(
            &self,
            ele: &Self::Element,
            is_less: Compare,
        ) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::lower_bound_by(self, ele, is_less)
        }

        fn lower_bound(&self, ele: &Self::Element) -> Self::Position
        where
            Self::Element: Ord,
        {
            rng::lower_bound(self, ele)
        }

        fn upper_bound_by<Compare>(
            &self,
            ele: &Self::Element,
            is_less: Compare,
        ) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::upper_bound_by(self, ele, is_less)
        }

        fn upper_bound(&self, ele: &Self::Element) -> Self::Position
        where
            Self::Element: Ord,
        {
            rng::upper_bound(self, ele)
        }
    }
}
