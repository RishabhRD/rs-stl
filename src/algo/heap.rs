// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{RandomAccessRange, SemiOutputRange};

/// Finds largest range that from start represents a heap wrt comparator.
///
/// # Precondition
///   - `[start, end)` reperesents valid position in rng.
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns last position `i` in `[start, end)` of rng such that
///     rng at `[start, i)` is a heap with respect to cmp.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in `[start, end)`.
/// ```rust
/// use stl::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2, 6];
/// let i = algo::is_heap_until_by(&arr, arr.start(), arr.end(), |x, y| x < y);
/// assert_eq!(i, 7);
/// ```
pub fn is_heap_until_by<Range, Compare>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    cmp: Compare,
) -> Range::Position
where
    Range: RandomAccessRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let n = rng.distance(start.clone(), end.clone());
    let mut dad: usize = 0;
    for son in 1..n {
        let son_pos = rng.after_n(start.clone(), son);
        let dad_pos = rng.after_n(start.clone(), dad);
        if cmp(rng.at(&dad_pos), rng.at(&son_pos)) {
            return son_pos;
        } else if son % 2 == 0 {
            dad += 1;
        }
    }
    end
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
/// Where n is number of elements in `[start, end)`.
/// ```rust
/// use stl::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2, 6];
/// let i = algo::is_heap_until(&arr, arr.start(), arr.end());
/// assert_eq!(i, 7);
/// ```
pub fn is_heap_until<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
) -> Range::Position
where
    Range: RandomAccessRange + ?Sized,
    Range::Element: Ord,
{
    is_heap_until_by(rng, start, end, |x, y| x < y)
}

/// Returns true if given range is a heap wrt comparator.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns true if range at `[start, end)` of rng is a heap wrt cmp.
///   - Otherwise, returns false.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2];
/// assert!(algo::is_heap_by(&arr, arr.start(), arr.end(), |x, y| x < y));
/// ```
pub fn is_heap_by<Range, Compare>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
    cmp: Compare,
) -> bool
where
    Range: RandomAccessRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    is_heap_until_by(rng, start, end.clone(), cmp) == end
}

/// Returns true if given range is a heap.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///
/// # Postcondition
///   - Returns true if range at `[start, end)` of rng is a heap.
///   - Otherwise, returns false.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [9, 5, 4, 1, 1, 3, 2];
/// assert!(algo::is_heap(&arr, arr.start(), arr.end()));
/// ```
pub fn is_heap<Range>(
    rng: &Range,
    start: Range::Position,
    end: Range::Position,
) -> bool
where
    Range: RandomAccessRange + ?Sized,
    Range::Element: Ord,
{
    is_heap_by(rng, start, end, |x, y| x < y)
}

/// Inserts last element in specified range into a heap wrt cmp. After insertion, full range would
/// be heap wrt cmp.
///
/// # Precondition
///   - `[start, end)` represents valid range in rng.
///   - `start == end` || `[start, end - 1)` should be a heap wrt cmp.
///   - cmp follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Inserts element at `end - 1` to heap at `[start, end - 1)` in rng. After
///     operation `[start, end)` is a heap wrt cmp.
///   - Complexity: O(log n) comparisions.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [8, 2, 9];
/// let start = arr.start();
/// let end = arr.end();
/// algo::push_heap_by(&mut arr, start, end, |x, y| x < y);
/// assert!(arr.is_heap_by(|x, y| x < y));
/// ```
pub fn push_heap_by<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    cmp: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    if start == end {
        return;
    }

    let mut cur = rng.distance(start.clone(), end) - 1;
    while cur > 0 {
        let parent = (cur - 1) / 2;
        let parent_pos = rng.after_n(start.clone(), parent);
        let cur_pos = rng.after_n(start.clone(), cur);
        if cmp(rng.at(&parent_pos), rng.at(&cur_pos)) {
            rng.swap_at(&parent_pos, &cur_pos);
            cur = parent;
        } else {
            break;
        }
    }
}

/// Inserts last element in specified range into a heap. After insertion, full range would be a
/// heap.
///
/// # Precondition
///   - `[start, end)` represents valid range in rng.
///   - `start == end` || `[start, end - 1)` should be a heap.
///
/// # Postcondition
///   - Inserts element at `end - 1` to heap at `[start, end - 1)` in rng. After
///     operation `[start, end)` is a heap.
///   - Complexity: O(log n) comparisions.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [8, 2, 9];
/// let start = arr.start();
/// let end = arr.end();
/// algo::push_heap(&mut arr, start, end);
/// assert!(arr.is_heap());
/// ```
pub fn push_heap<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    push_heap_by(rng, start, end, |x, y| x < y)
}
