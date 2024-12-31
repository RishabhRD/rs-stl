// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::ForwardRange;

/// Returns position of mimimum element in range by comparator.
///
/// # Precondition
///   - is_less follows strict-weak-ordering.
///   - If a comes before b then is_less(a, b) == true otherwise false.
///
/// # Postcondition
///   - Returns position minimum element in rng based on comparator is_less. If
///     there are multiple equivalent minimum elements, returns position of
///     first one of them.
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
/// let arr = [2, 1, 3, 1];
///
/// let i = rng::min_element_by(&arr, |x, y| x < y);
/// assert_eq!(i, 1);
///
/// let i = arr.min_element_by(|x, y| x < y);
/// assert_eq!(i, 1);
/// ```
pub fn min_element_by<Range, Compare>(
    rng: &Range,
    is_less: Compare,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let mut start = rng.start();
    if rng.is_end(&start) {
        return start;
    }

    let mut smallest = start.clone();
    start = rng.after(start);

    while !rng.is_end(&start) {
        if is_less(rng.at(&start), rng.at(&smallest)) {
            smallest = start.clone();
        }
        start = rng.after(start);
    }

    smallest
}

/// Returns position of mimimum element in range.
///
/// # Precondition
///
/// # Postcondition
///   - Returns position minimum element in rng. If there are multiple equal
///     minimum elements, returns the position of first one of them.
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
/// let arr = [2, 1, 3, 1];
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
    min_element_by(rng, |x, y| x < y)
}

/// Returns position of maximum element in the range by comparator.
///
/// # Precondition
///   - is_less should follow strict-weak-ordering.
///   - If a comes before b, then is_less(a, b) == true otherwise false.
///
/// # Postcondition
///   - Returns position of maximum element in rng. If there
///     are multiple equivalent maximum element, then returns the position of
///     last one of them.
///   - Returns end if rng is empty.
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
/// let arr = [1, 4, 3, 4, 2];
///
/// let i = rng::max_element_by(&arr, |x, y| x < y);
/// assert_eq!(i, 3);
///
/// let i = arr.max_element_by(|x, y| x < y);
/// assert_eq!(i, 3);
/// ```
pub fn max_element_by<Range, Compare>(
    rng: &Range,
    is_less: Compare,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let mut start = rng.start();
    if rng.is_end(&start) {
        return start;
    }

    let mut max = start.clone();
    start = rng.after(start);

    while !rng.is_end(&start) {
        if !is_less(rng.at(&start), rng.at(&max)) {
            max = start.clone();
        }
        start = rng.after(start);
    }

    max
}

/// Returns position of maximum element in the range.
///
/// # Precondition
///
/// # Postcondition
///   - Returns position of maximum element in rng. If there
///     are multiple equivalent maximum element, then returns the position of
///     last one of them.
///   - Returns end if rng is empty.
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
/// let arr = [1, 4, 3, 4, 2];
///
/// let i = rng::max_element(&arr);
/// assert_eq!(i, 3);
///
/// let i = arr.max_element();
/// assert_eq!(i, 3);
/// ```
pub fn max_element<Range>(rng: &Range) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    max_element_by(rng, |x, y| x < y)
}

/// Returns position of minimum element and maximum element in range by comparator.
///
/// # Precondition
///   - is_less follows strict-weak-ordering relationship.
///   - If a comes before b then is_less(a, b) == true otherwise false.
///
/// # Postcondition
///   - Returns a pair (i, j) where i denotes the position of minimum element
///     and j denotes the position of maximum element in rng by is_less.
///   - If there are multiple equivalent minimum elements, then i denotes
///     position of first one of them.
///   - If there are multiple equivalent maximum elements, then j denotes
///     position of last one of them.
///   - Complexity: O(n). At most `max(floor(3/2(n - 1)), 0)` comparisions.
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
/// let arr = [2, 1, 1, 2, 3, 3, 2];
///
/// let (i, j) = rng::minmax_element_by(&arr, |x, y| x < y);
/// assert_eq!(i, 1);
/// assert_eq!(j, 5);
///
/// let (i, j) = arr.minmax_element_by(|x, y| x < y);
/// assert_eq!(i, 1);
/// assert_eq!(j, 5);
/// ```
pub fn minmax_element_by<Range, Compare>(
    rng: &Range,
    is_less: Compare,
) -> (Range::Position, Range::Position)
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let mut start = rng.start();

    let mut min = start.clone();
    let mut max = start.clone();

    if rng.is_end(&start) {
        return (min, max);
    }

    start = rng.after(start);

    if rng.is_end(&start) {
        return (min, max);
    }

    if is_less(rng.at(&start), rng.at(&min)) {
        min = start.clone();
    } else {
        max = start.clone();
    }

    start = rng.after(start);

    while !rng.is_end(&start) {
        let i = start.clone();
        start = rng.after(start);
        if rng.is_end(&start) {
            if is_less(rng.at(&i), rng.at(&min)) {
                min = i;
            } else if !is_less(rng.at(&i), rng.at(&max)) {
                max = i;
            }
            break;
        } else if is_less(rng.at(&start), rng.at(&i)) {
            if is_less(rng.at(&start), rng.at(&min)) {
                min = start.clone();
            }
            if !is_less(rng.at(&i), rng.at(&max)) {
                max = i;
            }
        } else {
            if is_less(rng.at(&i), rng.at(&min)) {
                min = i;
            }
            if !is_less(rng.at(&start), rng.at(&max)) {
                max = start.clone();
            }
        }
        start = rng.after(start);
    }

    (min, max)
}

/// Returns position of minimum element and maximum element in range.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a pair (i, j) where i denotes the position of minimum element
///     and j denotes the position of maximum element in rng.
///   - If there are multiple equal minimum elements, then i denotes
///     position of first one of them.
///   - If there are multiple equal maximum elements, then j denotes
///     position of last one of them.
///   - Complexity: O(n). At most `max(floor(3/2(n - 1)), 0)` comparisions.
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
/// let arr = [2, 1, 1, 2, 3, 3, 2];
///
/// let (i, j) = rng::minmax_element(&arr);
/// assert_eq!(i, 1);
/// assert_eq!(j, 5);
///
/// let (i, j) = arr.minmax_element();
/// assert_eq!(i, 1);
/// assert_eq!(j, 5);
/// ```
pub fn minmax_element<Range>(rng: &Range) -> (Range::Position, Range::Position)
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    minmax_element_by(rng, |x, y| x < y)
}

pub mod infix {
    use crate::{rng, ForwardRange};

    /// `min_element`, `min_element_by`, `max_element`, `max_element_by`, `minmax_element`,
    /// `minmax_element_by`.
    pub trait STLMinMaxExt: ForwardRange {
        fn min_element_by<Compare>(&self, is_less: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn min_element(&self) -> Self::Position
        where
            Self::Element: Ord;

        fn max_element_by<Compare>(&self, is_less: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn max_element(&self) -> Self::Position
        where
            Self::Element: Ord;

        fn minmax_element_by<Compare>(
            &self,
            is_less: Compare,
        ) -> (Self::Position, Self::Position)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool;

        fn minmax_element(&self) -> (Self::Position, Self::Position)
        where
            Self::Element: Ord;
    }

    impl<R> STLMinMaxExt for R
    where
        R: ForwardRange + ?Sized,
    {
        fn min_element_by<Compare>(&self, is_less: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::min_element_by(self, is_less)
        }

        fn min_element(&self) -> Self::Position
        where
            Self::Element: Ord,
        {
            rng::min_element(self)
        }

        fn max_element_by<Compare>(&self, is_less: Compare) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::max_element_by(self, is_less)
        }

        fn max_element(&self) -> Self::Position
        where
            Self::Element: Ord,
        {
            rng::max_element(self)
        }

        fn minmax_element_by<Compare>(
            &self,
            is_less: Compare,
        ) -> (Self::Position, Self::Position)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::minmax_element_by(self, is_less)
        }

        fn minmax_element(&self) -> (Self::Position, Self::Position)
        where
            Self::Element: Ord,
        {
            rng::minmax_element(self)
        }
    }
}
