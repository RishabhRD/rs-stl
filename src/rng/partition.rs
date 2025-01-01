// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    algo, BoundedRange, ForwardRange, InputRange, OutputRange, SemiOutputRange,
};

/// Returns true if range is partitioned wrt pred, otherwise false.
///
/// # Precondition
///
/// # Postcondition
///   - Returns true if rng is partitioned wrt pred. i.e.,
///     There should be NO position `i` and `j` in rng such that
///     i comes before j and
///     `pred(rng.at(&i)) == false && pred(rng.at(&j)) == true`.
///   - Otherwise, returns false.
///   - Complexity: O(n). At most n applications of pred.
///
/// Where n is number of elements in rng.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = [1, 3, 5, 2, 4];
/// assert!(rng::is_partitioned(&arr, |x| x % 2 == 1));
/// ```
pub fn is_partitioned<Range, Predicate>(rng: &Range, pred: Predicate) -> bool
where
    Range: InputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        if !pred(&rng.at(&start)) {
            break;
        }
        start = rng.after(start);
    }

    while !rng.is_end(&start) {
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
///
/// # Postcondition
///   - Reorders elements in rng such that all elements
///     satisfying pred precede elements not satisfying pred.
///   - Relative order of the elements is NOT preserved.
///   - Returns position to first element in modified range that doesn't satisfy pred.
///   - Complexity: O(n). Exactly n applications of pred. Atmost n swaps.
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
/// let mut arr = [1, 3, 2, 5, 4];
/// let i = rng::partition(&mut arr, |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].all_of(|x| x % 2 == 1));
/// assert!(arr[i..].all_of(|x| x % 2 == 0));
///
/// let mut arr = [1, 3, 2, 5, 4];
/// let i = arr.partition(|x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].all_of(|x| x % 2 == 1));
/// assert!(arr[i..].all_of(|x| x % 2 == 0));
/// ```
pub fn partition<Range, Predicate>(
    rng: &mut Range,
    pred: Predicate,
) -> Range::Position
where
    Range: SemiOutputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        if !pred(&rng.at(&start)) {
            break;
        }
        start = rng.after(start);
    }

    if rng.is_end(&start) {
        return start;
    }

    let mut i = rng.after(start.clone());
    while !rng.is_end(&i) {
        if pred(&rng.at(&i)) {
            rng.swap_at(&i, &start);
            start = rng.after(start);
        }
        i = rng.after(i);
    }

    start
}

/// Partitions range based on given predicate with preserving relative order of elements.
///
/// # Precondition
///
/// # Postcondition
///   - Reorders elements in rng such that all elements
///     satisfying pred precede elements not satisfying pred.
///   - Relative order of the elements is preserved.
///   - Returns position to first element in modified range that doesn't satisfy pred.
///   - Complexity: O(n). Exactly n applications of pred. O(n) swaps.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
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
/// let i = rng::stable_partition(&mut arr, |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].equals(&[1, 3, 5]));
/// assert!(arr[i..].equals(&[2, 4]));
///
/// let mut arr = [1, 3, 2, 5, 4];
/// let i = arr.stable_partition(|x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].equals(&[1, 3, 5]));
/// assert!(arr[i..].equals(&[2, 4]));
/// ```
pub fn stable_partition<Range, Predicate>(
    rng: &mut Range,
    pred: Predicate,
) -> Range::Position
where
    Range: OutputRange + BoundedRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_partition(rng, start, end, pred)
}

/// Partitions range based on given predicate with preserving relative order of elements.
///
/// # Precondition
///
/// # Postcondition
///   - Reorders elements in rng such that all elements
///     satisfying pred precede elements not satisfying pred.
///   - Relative order of the elements is preserved.
///   - Returns position to first element in modified range that doesn't satisfy pred.
///   - Complexity: O(n.log2(n)). Exactly n applications of pred. Atmost n.log2(n) swaps.
///
/// Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
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
/// let i = rng::stable_partition_no_alloc(&mut arr, |x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].equals(&[1, 3, 5]));
/// assert!(arr[i..].equals(&[2, 4]));
///
/// let mut arr = [1, 3, 2, 5, 4];
/// let i = arr.stable_partition_no_alloc(|x| x % 2 == 1);
/// assert_eq!(i, 3);
/// assert!(arr[0..i].equals(&[1, 3, 5]));
/// assert!(arr[i..].equals(&[2, 4]));
/// ```
pub fn stable_partition_no_alloc<Range, Predicate>(
    rng: &mut Range,
    pred: Predicate,
) -> Range::Position
where
    Range: SemiOutputRange + BoundedRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool + Clone,
{
    let start = rng.start();
    let end = rng.end();
    algo::stable_partition_no_alloc(rng, start, end, pred)
}

/// Returns the position of first such element in partitioned range such that predicate is not
/// satisfied.
///
/// # Precondition
///   - rng is partitioned based on pred.
///
/// # Postcondition
///   - Returns position of first element in rng such that element at
///     that position does not satisfy pred.
///   - If no such element exist, then returns end position.
///   - Complexity: O(log2(n)). log2(n) applications of pred. For traversal,
///     if range is RandomAccessRange then O(log2(n)) traversal otherwise
///     O(n.log2(n)) traversal.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [1, 3, 5, 2, 4];
///
/// let i = rng::partition_point(&arr, |x| x % 2 == 1);
/// assert_eq!(i, 3);
///
/// let i = arr.partition_point(|x| x % 2 == 1);
/// assert_eq!(i, 3);
/// ```
pub fn partition_point<Range, Predicate>(
    rng: &Range,
    pred: Predicate,
) -> Range::Position
where
    Range: ForwardRange + BoundedRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    algo::partition_point(rng, rng.start(), rng.end(), pred)
}

pub mod infix {
    use crate::{
        rng, BoundedRange, ForwardRange, InputRange, OutputRange,
        SemiOutputRange,
    };

    /// `is_partitioned`.
    pub trait STLInputPartitionExt: InputRange {
        fn is_partitioned<Predicate>(&self, pred: Predicate) -> bool
        where
            Predicate: Fn(&Self::Element) -> bool,
        {
            rng::is_partitioned(self, pred)
        }
    }

    impl<R> STLInputPartitionExt for R where R: InputRange + ?Sized {}

    /// `partition`.
    pub trait STLSemiOutputPartitionExt: SemiOutputRange {
        fn partition<Predicate>(&mut self, pred: Predicate) -> Self::Position
        where
            Predicate: Fn(&Self::Element) -> bool,
        {
            rng::partition(self, pred)
        }
    }

    impl<R> STLSemiOutputPartitionExt for R where R: SemiOutputRange + ?Sized {}

    /// `stable_partition_no_alloc`.
    pub trait STLSemiOutputBoundedPartitionExt:
        SemiOutputRange + BoundedRange
    {
        fn stable_partition_no_alloc<Predicate>(
            &mut self,
            pred: Predicate,
        ) -> Self::Position
        where
            Predicate: Fn(&Self::Element) -> bool + Clone,
        {
            rng::stable_partition_no_alloc(self, pred)
        }
    }

    impl<R> STLSemiOutputBoundedPartitionExt for R where
        R: SemiOutputRange + BoundedRange + ?Sized
    {
    }

    /// `stable_partition`.
    pub trait STLOutputPartitionExt: BoundedRange + OutputRange {
        fn stable_partition<Predicate>(
            &mut self,
            pred: Predicate,
        ) -> Self::Position
        where
            Predicate: Fn(&Self::Element) -> bool + Clone,
        {
            rng::stable_partition(self, pred)
        }
    }

    impl<R> STLOutputPartitionExt for R where R: BoundedRange + OutputRange + ?Sized {}

    /// `partition_point`.
    pub trait STLForwardPartitonExt: ForwardRange + BoundedRange {
        fn partition_point<Predicate>(&self, pred: Predicate) -> Self::Position
        where
            Predicate: Fn(&Self::Element) -> bool,
        {
            rng::partition_point(self, pred)
        }
    }

    impl<R> STLForwardPartitonExt for R where R: ForwardRange + BoundedRange + ?Sized
    {}
}
