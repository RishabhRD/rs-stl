// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ForwardRange, InputRange, OutputRange, SemiOutputRange};

/// Returns true if range is partitioned wrt pred, otherwise false.
///
/// # Precondition
///   - `[start, end)` reperesents valid position in rng.
///
/// # Postcondition
///   - Returns true if rng at `[start, end)` is partitioned wrt pred. i.e.,
///     There should be NO position `i` and `j` in `[start, end)` such that
///     i comes before j and
///     `pred(rng.at(&i)) == false && pred(rng.at(&j)) == true`.
///   - Otherwise, returns false.
///   - Complexity: O(n). At most n applications of pred.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 3, 5, 2, 4];
/// assert!(algo::is_partitioned(&arr, arr.start(), arr.end(), |x| x % 2 == 1));
/// ```
pub fn is_partitioned<Range, Predicate>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> bool
where
    Range: InputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    while start != end {
        if !pred(&rng.at(&start)) {
            break;
        }
        start = rng.after(start);
    }

    while start != end {
        if pred(&rng.at(&start)) {
            return false;
        }
        start = rng.after(start);
    }

    true
}

/// Partitions range based on given predicate.
///
/// # Precondition
///   - `[start, end)` represents valid range in rng.
///
/// # Postcondition
///   - Reorders elements in rng at `[start, end)` such that all elements
///     satisfying pred precede elements not satisfying pred.
///   - Relative order of the elements is NOT preserved.
///   - Returns position to first element in modified range that doesn't satisfy pred.
///   - Complexity: O(n). Exactly n applications of pred. Atmost n swaps.
///
/// Where n is number of elements in `[start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 3, 2, 5, 4];
/// let start = arr.start();
/// let end = arr.end();
/// let i = algo::partition(&mut arr, start, end, |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].all_of(|x| x % 2 == 1));
/// assert!(arr[i..].all_of(|x| x % 2 == 0));
/// ```
pub fn partition<Range, Predicate>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> Range::Position
where
    Range: SemiOutputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    partition_details::partition_pos(rng, start, end, |r, i| pred(&r.at(i)))
}

/// Partitions range based on given predicate with preserving relative order of elements.
///
/// # Precondition
///   - `[start, end)` represents valid range in rng.
///
/// # Postcondition
///   - Reorders elements in rng at `[start, end)` such that all elements
///     satisfying pred precede elements not satisfying pred.
///   - Relative order of the elements is preserved.
///   - Returns position to first element in modified range that doesn't satisfy pred.
///   - Complexity: O(n). Exactly n applications of pred. O(n) swaps.
///
/// Where n is number of elements in `[start, end)`.
///
/// # NOTE
///   - The algorithm provides O(n) time complexity with O(n) additional memory allocation.
///     If memory allocation is a concern, consider using `stable_partition_no_alloc` algorithm, which
///     provides O(n.log2(n)) time complexity with no memory allocation.
///   - The algorithm requires `OutputRange` trait for reading and writing to buffer.
///     If only `SemiOutputRange` is available, consider using `stable_partition_no_alloc`
///     algorithm with given tradeoffs.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 3, 2, 5, 4];
/// let start = arr.start();
/// let end = arr.end();
/// let i = algo::stable_partition(&mut arr, start, end, |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].equals(&[1, 3, 5]));
/// assert!(arr[i..].equals(&[2, 4]));
/// ```
pub fn stable_partition<Range, Predicate>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> Range::Position
where
    Range: OutputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool + Clone,
{
    partition_details::stable_partition_pos(rng, start, end, |r, i| {
        pred(&r.at(i))
    })
}

/// Partitions range based on given predicate with preserving relative order of elements.
///
/// # Precondition
///   - `[start, end)` represents valid range in rng.
///
/// # Postcondition
///   - Reorders elements in rng at `[start, end)` such that all elements
///     satisfying pred precede elements not satisfying pred.
///   - Relative order of the elements is preserved.
///   - Returns position to first element in modified range that doesn't satisfy pred.
///   - Complexity: O(n.log2(n)). Exactly n applications of pred. Atmost n.log2(n) swaps.
///
/// Where n is number of elements in `[start, end)`.
///
/// # NOTE
///   - The algorithm provides O(n.log2(n)) time complexity with no additional memory allocation.
///     If memory allocation is not a concern, consider using `stable_partition` algorithm, which
///     provides O(n) time complexity with O(n) additional space.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 3, 2, 5, 4];
/// let start = arr.start();
/// let end = arr.end();
/// let i = algo::stable_partition_no_alloc(&mut arr, start, end, |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].equals(&[1, 3, 5]));
/// assert!(arr[i..].equals(&[2, 4]));
/// ```
pub fn stable_partition_no_alloc<Range, Predicate>(
    rng: &mut Range,
    start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> Range::Position
where
    Range: SemiOutputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool + Clone,
{
    partition_details::stable_partition_no_alloc_pos(rng, start, end, |r, i| {
        pred(&r.at(i))
    })
}

/// Returns the position of first such element in partitioned range such that predicate is not
/// satisfied.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - rng at `[start, end)` is partitioned based on pred.
///
/// # Postcondition
///   - Returns position of first element in rng at `[start, end)` such that
///     element at that position does not satisfy pred.
///   - If no such element exist, then returns end position.
///   - Complexity: O(log2(n)). log2(n) applications of pred. For traversal,
///     if range is RandomAccessRange then O(log2(n)) traversal otherwise
///     O(n.log2(n)) traversal.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 3, 5, 2, 4];
/// let i = algo::partition_point(&arr, arr.start(), arr.end(), |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// ```
pub fn partition_point<Range, Predicate>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    pred: Predicate,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    let mut n = rng.distance(start.clone(), end.clone());

    while n > 0 {
        let half = n / 2;
        let mid = rng.after_n(start.clone(), half);
        if pred(&rng.at(&mid)) {
            start = rng.after(mid);
            n -= half + 1;
        } else {
            n = half;
        }
    }

    start
}

#[doc(hidden)]
pub mod partition_details {
    use std::io::Write;
    use std::slice;
    use std::{io, mem::MaybeUninit};

    use crate::{algo::rotate, InputRange, OutputRange, SemiOutputRange};

    pub fn partition_pos<Range, Predicate>(
        rng: &mut Range,
        mut start: Range::Position,
        end: Range::Position,
        pred: Predicate,
    ) -> Range::Position
    where
        Range: SemiOutputRange + ?Sized,
        Predicate: Fn(&Range, &Range::Position) -> bool,
    {
        while start != end {
            if !pred(rng, &start) {
                break;
            }
            start = rng.after(start);
        }

        if start == end {
            return start;
        }

        let mut i = rng.after(start.clone());
        while i != end {
            if pred(rng, &i) {
                rng.swap_at(&i, &start);
                start = rng.after(start);
            }
            i = rng.after(i);
        }

        start
    }

    pub fn stable_partition_pos<Range, Predicate>(
        rng: &mut Range,
        mut start: Range::Position,
        end: Range::Position,
        pred: Predicate,
    ) -> Range::Position
    where
        Range: OutputRange + ?Sized,
        Predicate: Fn(&Range, &Range::Position) -> bool + Clone,
    {
        while start != end {
            if !pred(rng, &start) {
                break;
            }
            start = rng.after(start);
        }

        let n = rng.distance(start.clone(), end.clone());

        if n == 0 {
            return start;
        }

        if n == 1 {
            return if pred(rng, &start) {
                rng.after(start)
            } else {
                start
            };
        }

        let mut scratch: Vec<MaybeUninit<Range::Element>> =
            Vec::with_capacity(n);
        let buf = unsafe {
            slice::from_raw_parts_mut(scratch.as_mut_ptr(), scratch.capacity())
        };

        let mut rng_write = start.clone();
        let mut buf_write = buf.start();

        while start != end {
            if pred(rng, &start) {
                if start != rng_write {
                    rng.swap_at(&rng_write, &start);
                    rng_write = rng.after(rng_write);
                }
            } else {
                unsafe {
                    *buf.at_mut(&buf_write) = MaybeUninit::new(std::ptr::read(
                        &rng.at(&start) as &Range::Element,
                    ));
                }
                buf_write = buf.after(buf_write);
            }
            start = rng.after(start)
        }

        let res = rng_write.clone();

        let mut buf_cur = buf.start();
        while buf_cur != buf_write {
            unsafe {
                std::mem::swap(
                    (&mut rng.at_mut(&rng_write)) as &mut Range::Element,
                    buf.at_mut(&buf_cur).assume_init_mut(),
                );
            }
            rng_write = rng.after(rng_write);
            buf_cur = buf.after(buf_cur);
        }
        res
    }

    pub fn stable_partition_no_alloc_pos<Range, Predicate>(
        rng: &mut Range,
        start: Range::Position,
        end: Range::Position,
        pred: Predicate,
    ) -> Range::Position
    where
        Range: SemiOutputRange + ?Sized,
        Predicate: Fn(&Range, &Range::Position) -> bool + Clone,
    {
        io::stdout().flush().unwrap();
        let n = rng.distance(start.clone(), end.clone());

        if n == 0 {
            return start;
        }
        if n == 1 {
            return if pred(rng, &start) {
                rng.after(start)
            } else {
                start
            };
        }

        let mid = rng.after_n(start.clone(), n / 2);

        let left_start = stable_partition_no_alloc_pos(
            rng,
            start,
            mid.clone(),
            pred.clone(),
        );
        let right_end =
            stable_partition_no_alloc_pos(rng, mid.clone(), end, pred);
        rotate(rng, left_start, mid, right_end)
    }
}
