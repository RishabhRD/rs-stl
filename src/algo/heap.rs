// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{RandomAccessRange, SemiOutputRange};

/// Finds largest range that from start represents a heap wrt comparator.
///
/// # Precondition
///   - `[start, end)` reperesents valid position in rng.
///   - is_less follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns last position `i` in `[start, end)` of rng such that
///     rng at `[start, i)` is a heap with respect to is_less.
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
    is_less: Compare,
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
        if is_less(&rng.at(&dad_pos), &rng.at(&son_pos)) {
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
///   - is_less follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Returns true if range at `[start, end)` of rng is a heap wrt is_less.
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
    is_less: Compare,
) -> bool
where
    Range: RandomAccessRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    is_heap_until_by(rng, start, end.clone(), is_less) == end
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

/// Inserts last element in specified range into a heap wrt is_less. After insertion, full range would
/// be heap wrt is_less.
///
/// # Precondition
///   - `[start, end)` represents valid range in rng.
///   - `start == end` || `[start, end - 1)` should be a heap wrt is_less.
///   - is_less follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Inserts element at `end - 1` to heap at `[start, end - 1)` in rng. After
///     operation `[start, end)` is a heap wrt is_less.
///   - Complexity: O(log n) comparisions.
///
/// Where n is number of elements in `[start, end)`.
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
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    if start == end {
        return;
    }

    let mut cur = rng.distance(start.clone(), end) - 1;
    while cur > 0 {
        let parent = (cur - 1) >> 1;
        let parent_pos = rng.after_n(start.clone(), parent);
        let cur_pos = rng.after_n(start.clone(), cur);
        if is_less(&rng.at(&parent_pos), &rng.at(&cur_pos)) {
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
/// Where n is number of elements in `[start, end)`.
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

/// Swaps element at `start` position with element before `end` position and ensures `[start, end - 1)` is a heap wrt is_less.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` is a heap wrt is_less.
///   - is_less should follow strict-weak-ordering relationship.
///
/// # Postcondition
///   - Swaps element at `start` position with element before `end` position
///     and then ensures `[start, end - 1)` is a heap wrt is_less.
///   - If rng at `[start, end)` is empty, then do nothing.
///   - Complexity: O(log n) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [9, 8, 7];
/// algo::pop_heap_by(&mut arr, 0, 3, |x, y| x < y);
/// assert!(&arr[0..2].is_heap_by(|x, y| x < y));
/// assert!(arr.equals(&[8, 7, 9]));
/// ```
pub fn pop_heap_by<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool,
{
    let n = rng.distance(start.clone(), end.clone());
    if n == 0 || n == 1 {
        return;
    }

    let prev = rng.before(end);
    rng.swap_at(&start, &prev);
    heap_details::heapify(rng, start, prev, is_less);
}

/// Swaps element at `start` position with element before `end` position and ensures `[start, end - 1)` is a heap.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` is a heap.
///
/// # Postcondition
///   - Swaps element at `start` position with element before `end` position
///     and then ensures `[start, end - 1)` is a heap.
///   - If rng at `[start, end)` is empty, then do nothing.
///   - Complexity: O(log n) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [9, 8, 7];
/// algo::pop_heap(&mut arr, 0, 3);
/// assert!(&arr[0..2].is_heap());
/// println!("array: {:?}", arr);
/// assert!(arr.equals(&[8, 7, 9]));
/// ```
pub fn pop_heap<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    pop_heap_by(rng, start, end, |x, y| x < y);
}

/// Converts given heap into sorted range wrt is_less.
///
/// # Precondition
///  - `[start, end)` represents valid positions in rng.
///  - rng at `[start, end)` is a heap wrt is_less.
///
/// # Postcondition
///  - Sorts the element in rng at `[start, end)` such that whole range at `[start, end)` is in non-decreasing order wrt is_less.
///  - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [8, 7, 5, 2];
/// let start = arr.start();
/// let end = arr.end();
/// algo::sort_heap_by(&mut arr, start, end, |x, y| x < y);
/// assert!(arr.equals(&[2, 5, 7, 8]));
/// ```
pub fn sort_heap_by<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    mut end: Range::Position,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    while start != end {
        pop_heap_by(rng, start.clone(), end.clone(), is_less.clone());
        end = rng.before(end);
    }
}

/// Converts given heap into sorted range.
///
/// # Precondition
///  - `[start, end)` represents valid positions in rng.
///  - rng at `[start, end)` is a heap.
///
/// # Postcondition
///  - Sorts the elements in rng at `[start, end)` such that the whole range `[start, end)` is in non-decending order.
///  - Complexity: O(n.log2(n)) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [8, 7, 5, 2];
/// let start = arr.start();
/// let end = arr.end();
/// algo::sort_heap(&mut arr, start, end);
/// assert!(arr.equals(&[2, 5, 7, 8]));
/// ```
pub fn sort_heap<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    sort_heap_by(rng, start, end, |x, y| x < y);
}

/// Reorders the range such that resulting range is heap wrt is_less.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///   - is_less follows strict-weak-ordering relationship.
///
/// # Postcondition
///   - Reorders rng at `[start, end)` such that resulting range at `[start, end)`
///     is a heap wrt is_less.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [2, 5, 1, 4];
/// let start = arr.start();
/// let end = arr.end();
/// algo::make_heap_by(&mut arr, start, end, |x, y| x < y);
/// assert!(arr.is_heap_by(|x, y| x < y));
/// ```
pub fn make_heap_by<Range, Compare>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    is_less: Compare,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
{
    let n = rng.distance(start.clone(), end.clone());
    if n == 0 || n == 1 {
        return;
    }
    let mut root = n >> 1;
    loop {
        let root_pos = rng.after_n(start.clone(), root);
        heap_details::heapify(
            rng,
            root_pos.clone(),
            end.clone(),
            is_less.clone(),
        );
        if root_pos == start {
            break;
        }
        root -= 1;
    }
}

/// Reorders the range such that resulting range is heap.
///
/// # Precondition
///   - `[start, end)` represents valid position in rng.
///
/// # Postcondition
///   - Reorders rng at `[start, end)` such that resulting range at `[start, end)`
///     is a heap.
///   - Complexity: O(n) comparisions.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [2, 5, 1, 4];
/// let start = arr.start();
/// let end = arr.end();
/// algo::make_heap(&mut arr, start, end);
/// assert!(arr.is_heap());
/// ```
pub fn make_heap<Range>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
) where
    Range: RandomAccessRange + SemiOutputRange + ?Sized,
    Range::Element: Ord,
{
    make_heap_by(rng, start, end, |x, y| x < y);
}

#[doc(hidden)]
pub mod heap_details {
    use crate::{InputRange, OutputRange, RandomAccessRange, SemiOutputRange};

    use super::make_heap_by;

    /// # Precondition
    ///   - `[start, end)` represents valid positions in rng.
    ///   - children of start element should be a heap wrt is_less.
    ///   - is_less should follow strict-weak-ordering relationship.
    ///
    /// # Postcondition
    ///   - Reorders element in rng such that whole range is a heap.
    ///   - Complexity: O(log n) comparisions.
    ///
    /// Where n is number of elements in `[start, end)`.
    pub fn heapify<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        is_less: Compare,
    ) where
        Range: RandomAccessRange + SemiOutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool,
    {
        let n = rng.distance(start.clone(), end.clone());

        let mut root = 0;

        loop {
            let left_child = root * 2 + 1;
            let right_child = root * 2 + 2;

            let root_pos = rng.after_n(start.clone(), root);
            let mut largest = root_pos.clone();

            if left_child < n {
                let left_pos = rng.after_n(start.clone(), left_child);
                if is_less(&rng.at(&largest), &rng.at(&left_pos)) {
                    largest = left_pos;
                }
            }

            if right_child < n {
                let right_pos = rng.after_n(start.clone(), right_child);
                if is_less(&rng.at(&largest), &rng.at(&right_pos)) {
                    largest = right_pos;
                }
            }

            if largest == root_pos {
                break;
            }

            rng.swap_at(&root_pos, &largest);
            root = rng.distance(start.clone(), largest);
        }
    }

    pub fn heap_select_by<Range, Compare>(
        rng: &mut Range,
        start: Range::Position,
        mid: Range::Position,
        end: Range::Position,
        is_less: Compare,
    ) where
        Range: RandomAccessRange + SemiOutputRange + ?Sized,
        Compare: Fn(&Range::Element, &Range::Element) -> bool + Clone,
    {
        make_heap_by(rng, start.clone(), mid.clone(), is_less.clone());
        let mut cur = mid.clone();
        while cur != end {
            if is_less(&rng.at(&cur), &rng.at(&start)) {
                rng.swap_at(&cur, &start);
                heapify(rng, start.clone(), mid.clone(), is_less.clone());
            }
            cur = rng.after(cur);
        }
    }

    pub fn heap_select_copy_by<SrcRange, DestRange, Compare, TillFn>(
        src: &SrcRange,
        mut src_start: SrcRange::Position,
        src_till: TillFn,
        dest: &mut DestRange,
        dest_start: DestRange::Position,
        dest_end: DestRange::Position,
        is_less: Compare,
    ) -> DestRange::Position
    where
        SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
        SrcRange::Element: Clone,
        DestRange: RandomAccessRange + OutputRange + ?Sized,
        Compare: Fn(&SrcRange::Element, &SrcRange::Element) -> bool + Clone,
        TillFn: Fn(&SrcRange::Position) -> bool,
    {
        let mut write = dest_start.clone();
        while !src_till(&src_start) && write != dest_end {
            *dest.at_mut(&write) = src.at(&src_start).clone();
            src_start = src.after(src_start);
            write = dest.after(write);
        }

        make_heap_by(dest, dest_start.clone(), write.clone(), is_less.clone());
        while !src_till(&src_start) {
            if is_less(&src.at(&src_start), &dest.at(&dest_start)) {
                *dest.at_mut(&dest_start) = src.at(&src_start).clone();
                heapify(
                    dest,
                    dest_start.clone(),
                    write.clone(),
                    is_less.clone(),
                );
            }
            src_start = src.after(src_start);
        }

        write
    }
}
