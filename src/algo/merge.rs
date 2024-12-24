// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{utils::create_raw_buffer, InputRange, OutputRange};

use super::{copy, swap_ranges};

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
#[allow(clippy::too_many_arguments)]
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
#[allow(clippy::too_many_arguments)]
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

/// Merges 2 consecutive sorted range into one range wrt comparator.
///
/// # Precondition
///   - `[start, mid)` represents valid positions in rng.
///   - `[mid, end)` represents valid positions in rng.
///   - cmp follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 consecutive sorted range in rng at `[start, mid)` and `[mid, end)`
///     into one sorted range `[start, end)` wrt cmp.
///   - Relative order of equivalent elements by cmp is preserved.
///   - Complexity: O(n). Exactly n - 1 comparisions.
///
/// Where n in number of elements in `[start, end)`.
///
/// # NOTE
///   - Algorithm uses O(n) buffer space to acheive O(n) time complexity.
///     If allocation is a problem, use `inplace_merge_by_no_alloc` algorithm
///     instead doesn't do any allocation but provided O(n.log2(n)) time complexity.
///   - The algorithm requires `OutputRange` trait. If due to somereason,
///     only `SemiOutputRange` is available use `inplace_merge_by_no_alloc`
///     with same tradeoff.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// algo::inplace_merge_by(&mut arr, 0, 3, 6, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));
/// ```
#[allow(clippy::clone_on_copy)]
pub fn inplace_merge_by<Range, Compare>(
    rng: &mut Range,
    mut start: Range::Position,
    mid: Range::Position,
    end: Range::Position,
    cmp: Compare,
) where
    Range: OutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    if start == end {
        return;
    }

    let n = rng.distance(start.clone(), end.clone());
    let mut buf = create_raw_buffer::<Range::Element>(n);
    {
        let buf_start = buf.start();
        let buf_end = buf.end();
        // TODO: is it even safe?
        swap_ranges(
            rng,
            start.clone(),
            end.clone(),
            &mut buf,
            buf_start,
            buf_end,
        );
    }

    let mut start1 = buf.start();
    let end1 = buf.after_n(start1.clone(), rng.distance(start.clone(), mid));
    let mut start2 = end1;
    let end2 = buf.end();

    while start1 != end1 {
        if start2 == end2 {
            swap_ranges(
                &mut buf,
                start1.clone(),
                end1.clone(),
                rng,
                start.clone(),
                end.clone(),
            );
            return;
        }
        if cmp(buf.at(&start2), buf.at(&start1)) {
            std::mem::swap(rng.at_mut(&start), buf.at_mut(&start2));
            start2 = buf.after(start2);
        } else {
            std::mem::swap(rng.at_mut(&start), buf.at_mut(&start1));
            start1 = buf.after(start1);
        }
        start = rng.after(start);
    }
    swap_ranges(&mut buf, start2, end2, rng, start, end);
}

/// Merges 2 consecutive sorted range into one range.
///
/// # Precondition
///   - `[start, mid)` represents valid positions in rng.
///   - `[mid, end)` represents valid positions in rng.
///
/// # Postcondition
///   - Merges 2 consecutive sorted range in rng at `[start, mid)` and `[mid, end)`
///     into one sorted range `[start, end)`.
///   - Relative order of equal elements is preserved.
///   - Complexity: O(n). Exactly n - 1 comparisions.
///
/// Where n in number of elements in `[start, end)`.
///
/// # NOTE
///   - Algorithm uses O(n) buffer space to acheive O(n) time complexity.
///     If allocation is a problem, use `inplace_merge_no_alloc` algorithm
///     instead doesn't do any allocation but provided O(n.log2(n)) time complexity.
///   - The algorithm requires `OutputRange` trait. If due to somereason,
///     only `SemiOutputRange` is available use `inplace_merge_no_alloc`
///     with same tradeoff.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// algo::inplace_merge(&mut arr, 0, 3, 6);
/// assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));
/// ```
pub fn inplace_merge<Range>(
    rng: &mut Range,
    start: Range::Position,
    mid: Range::Position,
    end: Range::Position,
) where
    Range: OutputRange + ?Sized,
    Range::Element: Ord,
{
    inplace_merge_by(rng, start, mid, end, |x, y| x < y);
}
