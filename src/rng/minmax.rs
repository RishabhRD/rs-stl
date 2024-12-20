// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, ForwardRange};

/// Returns position of mimimum element in range by comparator.
///
/// # Precondition
///   - cmp follows strict-weak-ordering.
///   - If a < b then cmp(a, b) == true otherwise false.
///
/// # Postcondition
///   - Returns minimum element in rng based on comparator cmp.
///   - If rng is empty then return end position.
///   - Complexity: O(n). Exactly max(n - 1, 0) comparisions.
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
/// let arr = [2, 1, 3];
///
/// let i = rng::min_element_by(&arr, |x, y| x < y);
/// assert_eq!(i, 1);
///
/// let i = arr.min_element_by(|x, y| x < y);
/// assert_eq!(i, 1);
/// ```
pub fn min_element_by<Range, Compare>(
    rng: &Range,
    cmp: Compare,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::min_element_by(rng, rng.start(), rng.end(), cmp)
}

/// Returns position of mimimum element in range.
///
/// # Precondition
///
/// # Postcondition
///   - Returns minimum element in rng.
///   - If rng is empty then return end position.
///   - Complexity: O(n). Exactly max(n - 1, 0) comparisions.
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
/// let arr = [2, 1, 3];
///
/// let i = rng::min_element(&arr);
/// assert_eq!(i, 1);
///
/// let i = arr.min_element();
/// assert_eq!(i, 1);
/// ```
pub fn min_element<Range>(rng: &Range) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    algo::min_element(rng, rng.start(), rng.end())
}

pub mod infix {
    use crate::{rng, ForwardRange};

    /// `min_element`, `min_element_by`, `max_element`, `max_element_by`, `minmax_element`,
    /// `minmax_element_by`.
    pub trait STLMinMaxExt: ForwardRange {
        fn min_element_by<Compare>(&self, cmp: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn min_element(&self) -> Self::Position
        where
            Self::Element: Ord;
    }

    impl<R> STLMinMaxExt for R
    where
        R: ForwardRange + ?Sized,
    {
        fn min_element_by<Compare>(&self, cmp: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::min_element_by(self, cmp)
        }

        fn min_element(&self) -> Self::Position
        where
            Self::Element: Ord,
        {
            rng::min_element(self)
        }
    }
}
