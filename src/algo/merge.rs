// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, OutputRange};

use super::copy;

/// Merges 2 sorted ranges into one sorted range wrt Comparator.
///
/// # Precondition
///   - `[start1, end1)` represents valid positions in rng1.
///   - `[start2, end2)` represents valid positions in rng2.
///   - dest can accomodate n1 + n2 elements starting from out.
///   - cmp follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 sorted range at `[start1, end1)` of rng1 and `[start2, end2)`
///     at rng2 into one sorted range dest starting at out. Sorting is wrt cmp.
///   - Returns position immediately after last copied element in dest.
///   - Relative order of equivalent elements by cmp is preserved.
///   - Complexity: O(n1 + n2). At most n1 + n2 - 1 comparisions.
///
/// Where n1 is number of elements in `[start1, end1)` and n2 is number of
/// elements in `[start2, end2)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr1 = [(1, 1), (1, 3), (2, 3)];
/// let arr2 = [(1, 2), (2, 2), (2, 4)];
/// let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
/// let out = dest.start();
/// let i = algo::merge_by(
///     &arr1, arr1.start(), arr1.end(),
///     &arr2, arr2.start(), arr2.end(),
///     &mut dest, out,
///     |x, y| x.0 < y.0
/// );
/// assert_eq!(i, 6);
/// assert!(&dest[0..i].equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));
/// ```
pub fn merge_by<R1, R2, DestRange, Compare>(
    rng1: &R1,
    mut start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    mut start2: R2::Position,
    end2: R2::Position,
    dest: &mut DestRange,
    mut out: DestRange::Position,
    cmp: Compare,
) -> DestRange::Position
where
    R1: InputRange + ?Sized,
    R2: InputRange<Element = R1::Element> + ?Sized,
    DestRange: OutputRange<Element = R1::Element> + ?Sized,
    R1::Element: Clone,
    Compare: Fn(&R1::Element, &R1::Element) -> bool,
{
    while start1 != end1 {
        if start2 == end2 {
            return copy(rng1, start1, end1, dest, out);
        }
        if cmp(rng2.at(&start2), rng1.at(&start1)) {
            *dest.at_mut(&out) = rng2.at(&start2).clone();
            start2 = rng2.after(start2);
        } else {
            *dest.at_mut(&out) = rng1.at(&start1).clone();
            start1 = rng1.after(start1);
        }
        out = dest.after(out);
    }
    copy(rng2, start2, end2, dest, out)
}

/// Merges 2 sorted ranges into one sorted range.
///
/// # Precondition
///   - `[start1, end1)` represents valid positions in rng1.
///   - `[start2, end2)` represents valid positions in rng2.
///   - dest can accomodate n1 + n2 elements starting from out.
///
/// # Postcondition
///   - Merges 2 sorted range at `[start1, end1)` of rng1 and `[start2, end2)`
///     at rng2 into one sorted range dest starting at out.
///   - Returns position immediately after last copied element in dest.
///   - Relative order of equal elements is preserved.
///   - Complexity: O(n1 + n2). At most n1 + n2 - 1 comparisions.
///
/// Where n1 is number of elements in `[start1, end1)` and n2 is number of
/// elements in `[start2, end2)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr1 = [(1, 1), (1, 3), (2, 3)];
/// let arr2 = [(1, 2), (2, 2), (2, 4)];
/// let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
/// let out = dest.start();
/// let i = algo::merge(
///     &arr1, arr1.start(), arr1.end(),
///     &arr2, arr2.start(), arr2.end(),
///     &mut dest, out,
/// );
/// assert_eq!(i, 6);
/// assert!(&dest[0..i].equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));
/// ```
pub fn merge<R1, R2, DestRange>(
    rng1: &R1,
    start1: R1::Position,
    end1: R1::Position,
    rng2: &R2,
    start2: R2::Position,
    end2: R2::Position,
    dest: &mut DestRange,
    out: DestRange::Position,
) -> DestRange::Position
where
    R1: InputRange + ?Sized,
    R2: InputRange<Element = R1::Element> + ?Sized,
    DestRange: OutputRange<Element = R1::Element> + ?Sized,
    R1::Element: Clone,
    R1::Element: Ord,
{
    merge_by(rng1, start1, end1, rng2, start2, end2, dest, out, |x, y| {
        x < y
    })
}
