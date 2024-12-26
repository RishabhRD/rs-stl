// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::mem::MaybeUninit;

use crate::{BidirectionalRange, InputRange, OutputRange, SemiOutputRange};

use super::{copy, lower_bound_by, rotate, upper_bound_by};

/// Merges 2 sorted ranges into one sorted range wrt Comparator.
///
/// # Precondition
///   - `[start1, end1)` represents valid positions in rng1.
///   - `[start2, end2)` represents valid positions in rng2.
///   - dest can accomodate n1 + n2 elements starting from out.
///   - is_less follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 sorted range at `[start1, end1)` of rng1 and `[start2, end2)`
///     at rng2 into one sorted range dest starting at out. Sorting is wrt is_less.
///   - Returns position immediately after last copied element in dest.
///   - Relative order of equivalent elements by is_less is preserved.
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
    is_less: Compare,
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
        if is_less(rng2.at(&start2), rng1.at(&start1)) {
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

/// # Precondition
///   - buf should have atleast `[start, mid)` capacity.
fn merge_inplace_by_left_buffer<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    mid: Range::Position,
    end: Range::Position,
    is_less: Compare,
    mut buf: Vec<MaybeUninit<Range::Element>>,
) where
    Range: OutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    {
        let mut i = start.clone();
        while i != mid {
            unsafe {
                buf.push(MaybeUninit::new(std::ptr::read(rng.at(&i))));
            }
            i = rng.after(i);
        }
    }

    let mut left_pos = buf.start();
    let left_end = buf.end();
    let mut right_pos = mid.clone();
    let right_end = end.clone();
    let mut merge = start.clone();

    while left_pos != left_end && right_pos != right_end {
        unsafe {
            let left_elem = buf.at(&left_pos).assume_init_ref();
            let right_elem = rng.at(&right_pos);

            if is_less(right_elem, left_elem) {
                *rng.at_mut(&merge) = std::ptr::read(right_elem);
                right_pos = rng.after(right_pos);
            } else {
                *rng.at_mut(&merge) = std::ptr::read(left_elem);
                left_pos = buf.after(left_pos);
            }
        }
        merge = rng.after(merge);
    }

    while left_pos != left_end {
        unsafe {
            *rng.at_mut(&merge) =
                std::ptr::read(buf.at(&left_pos).assume_init_ref());
        }
        left_pos = buf.after(left_pos);
        merge = rng.after(merge);
    }
}

/// # Precondition
///   - buf should have atleast `[mid, end)` capacity.
#[allow(clippy::clone_on_copy)]
fn merge_inplace_by_right_buffer<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    mid: Range::Position,
    end: Range::Position,
    is_less: Compare,
    mut buf: Vec<MaybeUninit<Range::Element>>,
) where
    Range: OutputRange + BidirectionalRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    {
        let mut i = mid.clone();
        while i != end {
            unsafe {
                buf.push(MaybeUninit::new(std::ptr::read(rng.at(&i))));
            }
            i = rng.after(i);
        }
    }

    let mut left_pos = mid;
    let left_start = start;
    let mut right_pos = buf.end();
    let right_start = buf.start();
    let mut merge = end;

    while left_pos != left_start && right_pos != right_start {
        unsafe {
            let left_elem = rng.at(&rng.before(left_pos.clone()));
            let right_elem =
                buf.at(&buf.before(right_pos.clone())).assume_init_ref();

            if !is_less(right_elem, left_elem) {
                merge = rng.before(merge);
                *rng.at_mut(&merge) = std::ptr::read(right_elem);
                right_pos = buf.before(right_pos);
            } else {
                merge = rng.before(merge);
                *rng.at_mut(&merge) = std::ptr::read(left_elem);
                left_pos = rng.before(left_pos);
            }
        }
    }

    while right_pos != right_start {
        right_pos = buf.before(right_pos);
        merge = rng.before(merge);
        unsafe {
            *rng.at_mut(&merge) =
                std::ptr::read(buf.at(&right_pos).assume_init_ref());
        }
    }
}

/// Merges 2 consecutive sorted range into one range wrt comparator.
///
/// # Precondition
///   - `[start, mid)` represents valid positions in rng.
///   - `[mid, end)` represents valid positions in rng.
///   - is_less follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 consecutive sorted range in rng at `[start, mid)` and `[mid, end)`
///     into one sorted range `[start, end)` wrt is_less.
///   - Relative order of equivalent elements by is_less is preserved.
///   - Complexity: O(n). Exactly n - 1 comparisions.
///
/// Where n in number of elements in `[start, end)`.
///
/// # NOTE
///   - Algorithm uses min(n1, n2) additional buffer space to acheive O(n) time complexity.
///     If memory allocation is a concern, consider using `merge_inplace_by_no_alloc` algorithm.
///     `merge_inplace_by_no_alloc` doesn't do any memory allocation but provides O(n.log2(n)) time complexity.
///   - Algorithm requires `OutputRange` and `BidirectionalRange` trait to handle reading and writing from additional buffer.
///     If due to some reason, only `SemiOutputRange` is available use `merge_inplace_by_no_alloc`
///     instead, with the associated trade-offs.
///
///   Where n1 is number of elements in `[start, mid)` and n2 is number of elements in `[mid, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// algo::merge_inplace_by(&mut arr, 0, 3, 6, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));
/// ```
pub fn merge_inplace_by<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    mid: Range::Position,
    end: Range::Position,
    is_less: Compare,
) where
    Range: OutputRange + BidirectionalRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    if start == end {
        return;
    }

    let left_n = rng.distance(start.clone(), mid.clone());
    let right_n = rng.distance(mid.clone(), end.clone());

    if left_n <= right_n {
        merge_inplace_by_left_buffer(
            rng,
            start,
            mid,
            end,
            is_less,
            Vec::with_capacity(left_n),
        );
    } else {
        merge_inplace_by_right_buffer(
            rng,
            start,
            mid,
            end,
            is_less,
            Vec::with_capacity(right_n),
        );
    }
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
///   - Algorithm uses min(n1, n2) additional buffer space to acheive O(n) time complexity.
///     If memory allocation is a concern, consider using `merge_inplace_no_alloc` algorithm.
///     `merge_inplace_no_alloc` doesn't do any memory allocation but provides O(n.log2(n)) time complexity.
///   - Algorithm requires `OutputRange` and `BidirectionalRange` trait to handle reading and writing from additional buffer.
///     If due to some reason, only `SemiOutputRange` is available use `merge_inplace_no_alloc`
///     instead, with the associated trade-offs.
///
///   Where n1 is number of elements in `[start, mid)`, and n2 is number of elements in `[mid, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// algo::merge_inplace(&mut arr, 0, 3, 6);
/// assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));
/// ```
pub fn merge_inplace<Range>(
    rng: &mut Range,
    start: Range::Position,
    mid: Range::Position,
    end: Range::Position,
) where
    Range: OutputRange + BidirectionalRange + ?Sized,
    Range::Element: Ord,
{
    merge_inplace_by(rng, start, mid, end, |x, y| x < y);
}

/// Merges 2 consecutive sorted range into one range wrt comparator.
///
/// # Precondition
///   - `[start, mid)` represents valid positions in rng.
///   - `[mid, end)` represents valid positions in rng.
///   - is_less follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 consecutive sorted range in rng at `[start, mid)` and `[mid, end)`
///     into one sorted range `[start, end)` wrt is_less.
///   - Relative order of equivalent elements by is_less is preserved.
///   - Complexity: O(n.log2(n - 1)) comparisions.
///
/// Where n in number of elements in `[start, end)`.
///
/// # NOTE
///   - Algorithm provides O(n.log2(n)) time complexity, but O(1) additional space.
///     If memory allocation is not a concern, consider using `merge_inplace_by`
///     algorithm instead, that provides O(n) time complexity but O(min(n1, n2)) allocations.
///
///   Where n1 is number of elements in `[start, mid)` and n2 is number of elements in `[mid, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// algo::merge_inplace_by_no_alloc(&mut arr, 0, 3, 6, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));
/// ```
pub fn merge_inplace_by_no_alloc<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    mid: Range::Position,
    end: Range::Position,
    is_less: Compare,
) where
    Range: SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    if start == end {
        return;
    }
    let n0 = rng.distance(start.clone(), mid.clone());
    let n1 = rng.distance(mid.clone(), end.clone());

    if n0 == 0 || n1 == 0 {
        return;
    }

    let (left_half, left_end, right_start, right_half) = if n0 < n1 {
        let half = n0 / 2;
        let left_half = rng.after_n(start.clone(), half);
        let right_half = lower_bound_by(
            rng,
            mid.clone(),
            end.clone(),
            rng.at(&left_half),
            is_less.clone(),
        );
        let mut right_start =
            rotate(rng, left_half.clone(), mid, right_half.clone());
        let left_end = right_start.clone();
        right_start = rng.after(right_start);
        (left_half, left_end, right_start, right_half)
    } else {
        let half = n1 >> 1;
        let mut right_half = rng.after_n(mid.clone(), half);
        let left_half = upper_bound_by(
            rng,
            start.clone(),
            mid.clone(),
            rng.at(&right_half),
            is_less.clone(),
        );
        let left_end = rng.after_n(left_half.clone(), half);
        right_half = rng.after(right_half);
        let right_start =
            rotate(rng, left_half.clone(), mid, right_half.clone());
        (left_half, left_end, right_start, right_half)
    };

    merge_inplace_by_no_alloc(rng, start, left_half, left_end, is_less.clone());
    merge_inplace_by_no_alloc(rng, right_start, right_half, end, is_less);
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
///   - Algorithm provides O(n.log2(n)) time complexity, but O(1) additional space.
///     If memory allocation is not a concern, consider using `merge_inplace`
///     algorithm instead, that provides O(n) time complexity but O(min(n1, n2)) allocations.
///
///   Where n1 is number of elements in `[start, mid)`, and n2 is number of elements in `[mid, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// algo::merge_inplace_no_alloc(&mut arr, 0, 3, 6);
/// assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));
/// ```
pub fn merge_inplace_no_alloc<Range>(
    rng: &mut Range,
    start: Range::Position,
    mid: Range::Position,
    end: Range::Position,
) where
    Range: SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    merge_inplace_by_no_alloc(rng, start, mid, end, |x, y| x < y);
}
