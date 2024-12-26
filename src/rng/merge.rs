// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    algo, BidirectionalRange, InputRange, OutputRange, SemiOutputRange,
};

/// Merges 2 sorted ranges into one sorted range wrt Comparator.
///
/// # Precondition
///   - dest can accomodate n1 + n2 elements starting from out.
///   - is_less follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 sorted range rng1 and rng2 into one sorted prefix at dest. Sorting is wrt is_less.
///   - Returns position immediately after last copied element in dest.
///   - Relative order of equivalent elements by is_less is preserved.
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
    is_less: Compare,
) -> DestRange::Position
where
    R1: InputRange + ?Sized,
    R2: InputRange<Element = R1::Element> + ?Sized,
    DestRange: OutputRange<Element = R1::Element> + ?Sized,
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
        is_less,
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
    R1: InputRange + ?Sized,
    R2: InputRange<Element = R1::Element> + ?Sized,
    DestRange: OutputRange<Element = R1::Element> + ?Sized,
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

/// Merges 2 consecutive sorted range into one range wrt comparator.
///
/// # Precondition
///   - mid is valid position in rng.
///   - is_less follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 consecutive sorted range in rng at `[rng.start(), mid)` and `[mid, rng.end())`
///     into one sorted range rng wrt is_less.
///   - Relative order of equivalent elements by is_less is preserved.
///   - Complexity: O(n). Exactly n - 1 comparisions.
///
/// Where n in number of elements in rng.
///
/// # NOTE
///   - Algorithm uses O(n) buffer space to acheive O(n) time complexity.
///     If allocation is a problem, use `merge_inplace_by_no_alloc` algorithm
///     instead doesn't do any allocation but provided O(n.log2(n)) time complexity.
///   - The algorithm requires `OutputRange` trait. If due to somereason,
///     only `SemiOutputRange` is available use `merge_inplace_by_no_alloc`
///     with same tradeoff.
///
///   Where n1 is number of elements in `[rng.start(), mid)`, and n2 is number of elements in `[mid, rng.end())`.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// rng::merge_inplace_by(&mut arr, 3, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));
/// ```
pub fn merge_inplace_by<Range, Compare>(
    rng: &mut Range,
    mid: Range::Position,
    is_less: Compare,
) where
    Range: OutputRange + BidirectionalRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let start = rng.start();
    let end = rng.end();
    algo::merge_inplace_by(rng, start, mid, end, is_less);
}

/// Merges 2 consecutive sorted range into one range.
///
/// # Precondition
///   - mid is valid position in rng.
///
/// # Postcondition
///   - Merges 2 consecutive sorted range in rng at `[rng.start(), mid)` and `[mid, rng.end())`
///     into one sorted range rng.
///   - Relative order of equal elements is preserved.
///   - Complexity: O(n). Exactly n - 1 comparisions.
///
/// Where n in number of elements in rng.
///
/// # NOTE
///   - Algorithm uses min(n1, n2) additional buffer space to acheive O(n) time complexity.
///     If memory allocation is a concern, consider using `merge_inplace_by_no_alloc` algorithm.
///     `merge_inplace_by_no_alloc` doesn't do any memory allocation but provides O(n.log2(n)) time complexity.
///   - Algorithm requires `OutputRange` trait to handle reading and writing from additional buffer.
///     If due to some reason, only `SemiOutputRange` is available use `merge_inplace_by_no_alloc`
///     instead, with the associated trade-offs.
///
///   Where n1 is number of elements in `[rng.start(), mid)`, and n2 is number of elements in `[mid, rng.end())`.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// rng::merge_inplace(&mut arr, 3);
/// assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));
/// ```
pub fn merge_inplace<Range>(rng: &mut Range, mid: Range::Position)
where
    Range: OutputRange + BidirectionalRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::merge_inplace(rng, start, mid, end);
}

/// Merges 2 consecutive sorted range into one range wrt comparator.
///
/// # Precondition
///   - mid is valid position in rng.
///   - is_less follows strict-weak-ordering.
///
/// # Postcondition
///   - Merges 2 consecutive sorted range in rng at `[rng.start(), mid)` and `[mid, rng.end())`
///     into one sorted range rng wrt is_less.
///   - Relative order of equivalent elements by is_less is preserved.
///   - Complexity: O(n). Exactly n - 1 comparisions.
///
/// Where n in number of elements in rng.
///
/// # NOTE
///   - Algorithm provides O(n.log2(n)) time complexity, but O(1) additional space.
///     If memory allocation is not a concern, consider using `merge_inplace_by`
///     algorithm instead, that provides O(n) time complexity but O(min(n1, n2)) allocations.
///
///   Where n1 is number of elements in `[rng.start(), mid)`, and n2 is number of elements in `[mid, rng.end())`.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// rng::merge_inplace_by_no_alloc(&mut arr, 3, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));
/// ```
pub fn merge_inplace_by_no_alloc<Range, Compare>(
    rng: &mut Range,
    mid: Range::Position,
    is_less: Compare,
) where
    Range: SemiOutputRange + BidirectionalRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::merge_inplace_by_no_alloc(rng, start, mid, end, is_less);
}

/// Merges 2 consecutive sorted range into one range.
///
/// # Precondition
///   - mid is valid position in rng.
///
/// # Postcondition
///   - Merges 2 consecutive sorted range in rng at `[rng.start(), mid)` and `[mid, rng.end())`
///     into one sorted range rng.
///   - Relative order of equal elements is preserved.
///   - Complexity: O(n). Exactly n - 1 comparisions.
///
/// Where n in number of elements in rng.
///
/// # NOTE
///   - Algorithm provides O(n.log2(n)) time complexity, but O(1) additional space.
///     If memory allocation is not a concern, consider using `merge_inplace`
///     algorithm instead, that provides O(n) time complexity but O(min(n1, n2)) allocations.
///
///   Where n1 is number of elements in `[rng.start(), mid)`, and n2 is number of elements in `[mid, rng.end())`.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
/// rng::merge_inplace(&mut arr, 3);
/// assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));
/// ```
pub fn merge_inplace_no_alloc<Range>(rng: &mut Range, mid: Range::Position)
where
    Range: SemiOutputRange + BidirectionalRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::merge_inplace_no_alloc(rng, start, mid, end);
}
