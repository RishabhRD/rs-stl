// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    algo::{self, heap_details::heap_select_copy_by, sort_heap_by},
    BoundedRange, ForwardRange, InputRange, OutputRange, RandomAccessRange,
    SemiOutputRange,
};

/// Unstable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts rng in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are NOT preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in rng.
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
pub fn sort_range_by<Range, Compare>(rng: &mut Range, is_less: Compare)
where
    Range: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::sort_range_by(rng, start, end, is_less)
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
/// Where n is number of elements in rng.
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
    Range: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::sort_range(rng, start, end)
}

/// Stable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts rng in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - The algorithm allocates O(n) buffer for achieving O(n.log2(n)) time complexity.
///   If memory allocation is a problem, consider using `stable_sort_by_no_alloc`.
///   `stable_sort_by_no_alloc` doesn't do any buffer allocation, however provides
///   O(n.log2^2(n)) time complexity.
/// - The algorithm requires `OutputRange` trait to read and write from/to
///   buffer. If only `SemiOutputRange` is available, consider using
///   `stable_sort_by_no_alloc` with given tradeoffs.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// rng::stable_sort_by(&mut arr, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// arr.stable_sort_by(|x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
/// ```
pub fn stable_sort_by<Range, Compare>(rng: &mut Range, is_less: Compare)
where
    Range: RandomAccessRange + OutputRange + BoundedRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_sort_by(rng, start, end, is_less);
}

/// Stable sort: sorts range in non-decreasing order.
///
/// # Precondition
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - The algorithm provides O(n.log2^2(n)) time complexity. If memory allocation
///   is not a problem, consider using `stable_sort_by` algorithm.
/// - The algorithm just requires `SemiOutputRange` for mutation instead of
///   `OutputRange`, that might be desirable in some situations.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// rng::stable_sort(&mut arr);
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// arr.stable_sort();
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
/// ```
pub fn stable_sort<Range>(rng: &mut Range)
where
    Range: RandomAccessRange + OutputRange + BoundedRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_sort(rng, start, end);
}

/// Stable sort: sorts range in non-decreasing order based on comparator.
///
/// # Precondition
///   - is_less follows strict weak ordering relationship, i.e., returns true for
///     is_less(a, b) if a comes before b otherwise false.
///   - Also if is_less(a, b) == false && is_less(b, a) == false, then a is equivalent
///     to b.
///
/// # Postcondition
///   - sorts rng in non-decreasing order by comparator is_less.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - The algorithm provides O(n.log2^2(n)) time complexity. If memory allocation
///   is not a problem, consider using `stable_sort` algorithm.
/// - The algorithm just requires `SemiOutputRange` for mutation instead of
///   `OutputRange`, that might be desirable in some situations.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// rng::stable_sort_by_no_alloc(&mut arr, |x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// arr.stable_sort_by_no_alloc(|x, y| x.0 < y.0);
/// assert!(arr.equals(&[(1, 2), (1, 1), (3, 3)]));
/// ```
pub fn stable_sort_by_no_alloc<Range, Compare>(
    rng: &mut Range,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_sort_by_no_alloc(rng, start, end, is_less);
}

/// Stable sort: sorts range in non-decreasing order.
///
/// # Precondition
///
/// # Postcondition
///   - sorts range at `[start, end)` in non-decreasing.
///   - Relative order of equivalent elements are preserved.
///   - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # NOTE
/// - The algorithm allocates O(n) buffer for achieving O(n.log2(n)) time complexity.
///   If memory allocation is a problem, consider using `stable_sort_no_alloc`.
///   `stable_sort_no_alloc` doesn't do any buffer allocation, however provides
///   O(n.log2^2(n)) time complexity.
/// - The algorithm requires `OutputRange` trait to read and write from/to
///   buffer. If only `SemiOutputRange` is available, consider using
///   `stable_sort_no_alloc` with given tradeoffs.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// rng::stable_sort_no_alloc(&mut arr);
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
///
/// let mut arr = [(3, 3), (1, 2), (1, 1)];
/// arr.stable_sort_no_alloc();
/// assert!(arr.equals(&[(1, 1), (1, 2), (3, 3)]));
/// ```
pub fn stable_sort_no_alloc<Range>(rng: &mut Range)
where
    Range: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_sort_no_alloc(rng, start, end);
}

/// Finds the largest prefix in range such that elements in the prefix are in non-decreasing order
/// wrt comparator.
///
/// # Precondition
///   - is_less should follow strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns last such position i, so that `[rng.start(), i)` is in non-decreasing order wrt is_less.
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
/// let arr = [1, 1, 2, 0];
///
/// let i = rng::is_sorted_until_by(&arr, |x, y| x < y);
/// assert_eq!(i, 3);
///
/// let i = arr.is_sorted_until_by(|x, y| x < y);
/// assert_eq!(i, 3);
/// ```
pub fn is_sorted_until_by<Range, Compare>(
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

    let mut prev = start.clone();
    start = rng.after(start);
    while !rng.is_end(&start) {
        if is_less(&rng.at(&start), &rng.at(&prev)) {
            return start;
        }
        prev = start.clone();
        start = rng.after(start);
    }
    start
}

/// Finds the largest prefix in range such that elements in the prefix are in non-decreasing order.
///
/// # Precondition
///
/// # Postcondition
///   - Returns last such position i, so that `[rng.start(), i)` is in non-decreasing order.
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
/// let arr = [1, 1, 2, 0];
///
/// let i = rng::is_sorted_until(&arr);
/// assert_eq!(i, 3);
///
/// let i = arr.is_sorted_until();
/// assert_eq!(i, 3);
/// ```
pub fn is_sorted_until<Range>(rng: &Range) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    is_sorted_until_by(rng, |x, y| x < y)
}

/// Returns true if given range is sorted in non-decreasing order wrt comparator.
///
/// # Precondition
///   - is_less should follow strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns true if given range is sorted in non-decreasing order wrt comparator.
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
/// let arr = [1, 1, 2];
/// assert!(rng::is_sorted_by(&arr, |x, y| x < y));
/// assert!(arr.is_sorted_by(|x, y| x < y));
/// ```
pub fn is_sorted_by<Range, Compare>(rng: &Range, is_less: Compare) -> bool
where
    Range: ForwardRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    rng.is_end(&is_sorted_until_by(rng, is_less))
}

/// Returns true if given range is sorted in non-decreasing order.
///
/// # Precondition
///
/// # Postcondition
///   - Returns true if given range is sorted in non-decreasing order.
///   - Complexity: O(n) comparisions.
///
/// #### Infix Supported
/// YES
///
/// Where n is number of elements in rng.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 1, 2];
/// assert!(rng::is_sorted(&arr));
/// assert!(arr.is_sorted());
/// ```
pub fn is_sorted<Range>(rng: &Range) -> bool
where
    Range: ForwardRange + ?Sized,
    Range::Element: Ord,
{
    is_sorted_by(rng, |x, y| x < y)
}

/// Reorders elements in range such that `[rng.start(), mid)` contains `distance(rng.start(), mid)` smallest
/// elements in range wrt comparator.
///
/// # Precondition
///   - mid is a valid position in rng.
///   - is_less follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Reorders elements in range such that `[rng.start(), mid)` contains
///     `distance(rng.start(), mid)` smallest elements in range wrt is_less.
///   - Relative order of equivalent elements are NOT preserved.
///   - Order of elements at [mid, rng.end()) is unspecified.
///   - Complexity: O(n.log(m)) comparisions.
///
/// Where n = distance(rng.start(), rng.end()), m = distance(rng.start(), mid).
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [4, 1, 5, 1, 2];
/// let mid = arr.after_n(arr.start(), 3);
/// rng::partial_sort_by(&mut arr, mid, |x, y| x < y);
/// assert!(&arr[0..3].equals(&[1, 1, 2]));
///
/// let mut arr = [4, 1, 5, 1, 2];
/// let mid = arr.after_n(arr.start(), 3);
/// arr.partial_sort_by(mid, |x, y| x < y);
/// assert!(&arr[0..3].equals(&[1, 1, 2]));
/// ```
pub fn partial_sort_by<Range, Compare>(
    rng: &mut Range,
    mid: Range::Position,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::partial_sort_by(rng, start, mid, end, is_less);
}

/// Reorders elements in range such that `[rng.start(), mid)` contains `distance(rng.start(), mid)` smallest
/// elements in range.
///
/// # Precondition
///   - `[rng.start(), mid)` represents valid positions in rng.
///   - `[mid, rng.end())` represents valid positions in rng.
///
/// # Postcondition
///   - Reorders elements in range such that `[rng.start(), mid)` contains
///     `distance(rng.start(), mid)` smallest elements in range.
///   - Relative order of equivalent elements are NOT preserved.
///   - Order of elements at [mid, rng.end()) is unspecified.
///   - Complexity: O(n.log(m)) comparisions.
///
/// Where n = distance(rng.start(), rng.end(0), m = distance(rng.start(), mid).
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [4, 1, 5, 1, 2];
/// let mid = arr.after_n(arr.start(), 3);
/// rng::partial_sort(&mut arr, mid);
/// assert!(&arr[0..3].equals(&[1, 1, 2]));
///
/// let mut arr = [4, 1, 5, 1, 2];
/// let mid = arr.after_n(arr.start(), 3);
/// arr.partial_sort(mid);
/// assert!(&arr[0..3].equals(&[1, 1, 2]));
/// ```
pub fn partial_sort<Range>(rng: &mut Range, mid: Range::Position)
where
    Range: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::partial_sort(rng, start, mid, end)
}

/// Stores d top minimum elements in non decreasing order wrt comparator of source range to destination range where d is
/// number of elements in destination range.
///
/// # Precondition
///   - is_less follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Stores d top minimum elements in non decreasing order wrt comparator of
///     src range to dest range where d is number of elements in
///     destination range.
///   - Relative order of equivalent elements are NOT preserved.
///   - Returns the position after last copied element in dest.
///   - Complexity: O(n.log2(min(n, d))) comparisions.
///
/// Where n = distance(src.start(), src.end()), d = distance(dest.start(), dest.end()).
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [4, 1, 5, 1, 2];
/// let mut dest = [0, 0, 0];
/// let i = rng::partial_sort_copy_by(&arr, &mut dest, |x, y| x < y);
/// assert_eq!(i, 3);
/// assert!(dest.equals(&[1, 1, 2]));
/// ```
pub fn partial_sort_copy_by<SrcRange, DestRange, Compare>(
    src: &SrcRange,
    dest: &mut DestRange,
    is_less: Compare,
) -> DestRange::Position
where
    DestRange: RandomAccessRange + OutputRange + BoundedRange + ?Sized,
    SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    Compare: Fn(&SrcRange::Element, &SrcRange::Element) -> bool + Clone,
{
    let src_start = src.start();
    let dest_start = dest.start();
    let dest_end = dest.end();
    let i = heap_select_copy_by(
        src,
        src_start,
        |i| src.is_end(i),
        dest,
        dest_start.clone(),
        dest_end,
        is_less.clone(),
    );

    sort_heap_by(dest, dest_start, i.clone(), is_less);

    i
}

/// Stores d top minimum elements in non decreasing order of source range to destination range where d is
/// number of elements in destination range.
///
/// # Precondition
///   - `[src_start, src_end)` represents valid positions in src.
///   - `[dest_start, dest_end)` represents valid positions in dest.
///
/// # Postcondition
///   - Stores d top minimum elements in non decreasing order of
///     src range to dest range where d is number of elements in
///     destination range.
///   - Relative order of equivalent elements are NOT preserved.
///   - Returns the position after last copied element in dest.
///   - Complexity: O(n.log2(min(n, d))) comparisions.
///
/// Where n = distance(src_start, src_end), d = distance(dest_start, dest_mid).
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [4, 1, 5, 1, 2];
/// let mut dest = [0, 0, 0, 0];
/// let i = rng::partial_sort_copy(&arr, &mut dest);
/// assert_eq!(i, 4);
/// assert!(dest.equals(&[1, 1, 2, 4]));
/// ```
pub fn partial_sort_copy<SrcRange, DestRange>(
    src: &SrcRange,
    dest: &mut DestRange,
) -> DestRange::Position
where
    DestRange: RandomAccessRange + OutputRange + BoundedRange + ?Sized,
    SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
    SrcRange::Element: Clone + Ord,
{
    partial_sort_copy_by(src, dest, |x, y| x < y)
}

/// Rearranges the elements in range st:
///   - element at nth position is changed to whatever element if range is sorted wrt comparator.
///   - resulting range would be patitioned on nth element wrt comparator.
///
/// # Precondition
///   - nth is a valid position in rng.
///   - is_less follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - element in rng is ordered as such, element at nth position would change
///     to whatever element if rng was sorted wrt is_less.
///   - Resulting range rng at would be partitioned by element
///     at nth_position, i.e., for any position j > nth,
///     `is_less(rng.at(&j), rng.at(&nth)) == false`.
///   - if nth == end, do nothing.
///   - Stability of equivalent is not guaranteed.
///   - Comlexity: O(n) comparisions on **average**.
///
/// Where n == `rng.distance(rng.start(), rng.end())`
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [4, 1, 5, 1, 2];
/// let nth = arr.after_n(arr.start(), 2);
/// rng::nth_element_by(&mut arr, nth,|x, y| x < y);
/// assert_eq!(arr.at(&nth), &2);
/// assert!(&arr[0..nth].is_partitioned(|x| *x < 2));
/// assert!(&arr[nth..].is_partitioned(|x| *x >= 2));
///
/// let mut arr = [4, 1, 5, 1, 2];
/// let nth = arr.after_n(arr.start(), 2);
/// arr.nth_element_by(nth,|x, y| x < y);
/// assert_eq!(arr.at(&nth), &2);
/// assert!(&arr[0..nth].is_partitioned(|x| *x < 2));
/// assert!(&arr[nth..].is_partitioned(|x| *x >= 2));
/// ```
pub fn nth_element_by<Range, Compare>(
    rng: &mut Range,
    nth: Range::Position,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let start = rng.start();
    let end = rng.end();
    algo::nth_element_by(rng, start, nth, end, is_less);
}

/// Rearranges the elements in range st:
///   - element at nth position is changed to whatever element if range is sorted.
///   - resulting range would be patitioned on nth element.
///
/// # Precondition
///   - nth represents valid position in rng.
///
/// # Postcondition
///   - elements in rng is ordered as such, element at nth position would
///     change to whatever element if rng was sorted.
///   - Resulting range rng would be partitioned by element at nth_position,
///     i.e., for any position j > nth, `rng.at(&j) >= rng.at(&nth)`.
///   - if nth == end, do nothing.
///   - Stability of equal elements is not guaranteed.
///   - Comlexity: O(n) comparisions on **average**.
///
/// Where n == `rng.distance(rng.start(), rng.end())`
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [4, 1, 5, 1, 2];
/// let nth = arr.after_n(arr.start(), 2);
/// rng::nth_element(&mut arr, nth);
/// assert_eq!(arr.at(&nth), &2);
/// assert!(&arr[0..nth].is_partitioned(|x| *x < 2));
/// assert!(&arr[nth..].is_partitioned(|x| *x >= 2));
///
/// let mut arr = [4, 1, 5, 1, 2];
/// let nth = arr.after_n(arr.start(), 2);
/// arr.nth_element(nth);
/// assert_eq!(arr.at(&nth), &2);
/// assert!(&arr[0..nth].is_partitioned(|x| *x < 2));
/// assert!(&arr[nth..].is_partitioned(|x| *x >= 2));
/// ```
pub fn nth_element<Range>(rng: &mut Range, nth: Range::Position)
where
    Range: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized,
    Range::Element: Ord,
{
    let start = rng.start();
    let end = rng.end();
    algo::nth_element(rng, start, nth, end);
}

pub mod infix {
    use crate::{
        rng, BoundedRange, ForwardRange, OutputRange, RandomAccessRange,
        SemiOutputRange,
    };

    /// `sort_range`, `sort_range_by`, `stable_sort_no_alloc`, `stable_sort_by_no_alloc`,
    /// `partial_sort`, `partial_sort_by`, `nth_element`, `nth_element_by`.
    pub trait STLSortExt:
        RandomAccessRange + SemiOutputRange + BoundedRange
    {
        fn sort_range_by<Compare>(&mut self, is_less: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::sort_range_by(self, is_less)
        }

        fn sort_range(&mut self)
        where
            Self::Element: Ord,
        {
            rng::sort_range(self)
        }

        fn stable_sort_by_no_alloc<Compare>(&mut self, is_less: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::stable_sort_by_no_alloc(self, is_less);
        }

        fn stable_sort_no_alloc(&mut self)
        where
            Self::Element: Ord,
        {
            rng::stable_sort_no_alloc(self);
        }

        fn partial_sort_by<Compare>(
            &mut self,
            mid: Self::Position,
            is_less: Compare,
        ) where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::partial_sort_by(self, mid, is_less)
        }

        fn partial_sort(&mut self, mid: Self::Position)
        where
            Self::Element: Ord,
        {
            rng::partial_sort(self, mid)
        }

        fn nth_element_by<Compare>(
            &mut self,
            nth: Self::Position,
            is_less: Compare,
        ) where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::nth_element_by(self, nth, is_less);
        }

        fn nth_element(&mut self, nth: Self::Position)
        where
            Self::Element: Ord,
        {
            rng::nth_element(self, nth);
        }
    }

    impl<R> STLSortExt for R where
        R: RandomAccessRange + SemiOutputRange + BoundedRange + ?Sized
    {
    }

    /// `stable_sort`, `stable_sort_by`.
    pub trait STLSortOutputExt:
        RandomAccessRange + OutputRange + BoundedRange
    {
        fn stable_sort_by<Compare>(&mut self, is_less: Compare)
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
        {
            rng::stable_sort_by(self, is_less);
        }

        fn stable_sort(&mut self)
        where
            Self::Element: Ord,
        {
            rng::stable_sort(self);
        }
    }

    impl<R> STLSortOutputExt for R where
        R: RandomAccessRange + OutputRange + BoundedRange + ?Sized
    {
    }

    /// `is_sorted_until`, `is_sorted_until_by`, `is_sorted`, `is_sorted_by`, `partial_sort`,
    /// `partial_sort_by`.
    pub trait STLSortForwardExt: ForwardRange {
        fn is_sorted_until_by<Compare>(
            &self,
            is_less: Compare,
        ) -> Self::Position
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::is_sorted_until_by(self, is_less)
        }

        fn is_sorted_until(&self) -> Self::Position
        where
            Self::Element: Ord,
        {
            rng::is_sorted_until(self)
        }

        fn is_sorted_by<Compare>(&self, is_less: Compare) -> bool
        where
            Compare: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::is_sorted_by(self, is_less)
        }

        fn is_sorted(&self) -> bool
        where
            Self::Element: Ord,
        {
            rng::is_sorted(self)
        }
    }

    impl<R> STLSortForwardExt for R where R: ForwardRange + ?Sized {}
}
