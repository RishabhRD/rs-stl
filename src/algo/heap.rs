// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::RandomAccessRange;

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
