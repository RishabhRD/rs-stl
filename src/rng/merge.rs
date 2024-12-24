// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange, OutputRange};

/// Merges 2 sorted ranges into one sorted range wrt Comparator.
///
/// # Precondition
///   - dest can accomodate n1 + n2 elements starting from out.
///   - cmp follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 sorted range rng1 and rng2 into one sorted prefix at dest. Sorting is wrt cmp.
///   - Returns position immediately after last copied element in dest.
///   - Relative order of equivalent elements by cmp is preserved.
///   - Complexity: O(n1 + n2). At most n1 + n2 - 1 comparisions.
///
/// Where n1 is number of elements in rng1 and n2 is number of elements in rng2.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr1 = [(1, 1), (1, 3), (2, 3)];
/// let arr2 = [(1, 2), (2, 2), (2, 4)];
/// let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
/// let i = rng::merge_by(&arr1, &arr2, &mut dest, |x, y| x.0 < y.0);
/// assert_eq!(i, 6);
/// assert!(&dest[0..i].equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));
/// ```
pub fn merge_by<R1, R2, DestRange, Compare>(
    rng1: &R1,
    rng2: &R2,
    dest: &mut DestRange,
    cmp: Compare,
) -> DestRange::Position
where
    R1: InputRange,
    R2: InputRange<Element = R1::Element>,
    DestRange: OutputRange<Element = R1::Element>,
    R1::Element: Clone,
    Compare: Fn(&R1::Element, &R1::Element) -> bool,
{
    let out = dest.start();
    algo::merge_by(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        rng2.start(),
        rng2.end(),
        dest,
        out,
        cmp,
    )
}

/// Merges 2 sorted ranges into one sorted range.
///
/// # Precondition
///   - dest can accomodate n1 + n2 elements.
///
/// # Postcondition
///   - Merges 2 sorted range rng1 and rng2 into one sorted prefix dest.
///   - Returns position immediately after last copied element in dest.
///   - Relative order of equal elements is preserved.
///   - Complexity: O(n1 + n2). At most n1 + n2 - 1 comparisions.
///
/// Where n1 is number of elements in rng1 and n2 is number of elements in rng2.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr1 = [(1, 1), (1, 3), (2, 3)];
/// let arr2 = [(1, 2), (2, 2), (2, 4)];
/// let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
/// let i = rng::merge(&arr1, &arr2, &mut dest);
/// assert_eq!(i, 6);
/// assert!(&dest[0..i].equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));
/// ```
pub fn merge<R1, R2, DestRange>(
    rng1: &R1,
    rng2: &R2,
    dest: &mut DestRange,
) -> DestRange::Position
where
    R1: InputRange,
    R2: InputRange<Element = R1::Element>,
    DestRange: OutputRange<Element = R1::Element>,
    R1::Element: Clone,
    R1::Element: Ord,
{
    let out = dest.start();
    algo::merge(
        rng1,
        rng1.start(),
        rng1.end(),
        rng2,
        rng2.start(),
        rng2.end(),
        dest,
        out,
    )
}
