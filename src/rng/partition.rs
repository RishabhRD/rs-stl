// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange, OutputRange};

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
    algo::is_partitioned(rng, rng.start(), rng.end(), pred)
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
    Range: OutputRange + ?Sized,
    Predicate: Fn(&Range::Element) -> bool,
{
    let start = rng.start();
    let end = rng.end();
    algo::partition(rng, start, end, pred)
}

pub mod infix {
    use crate::{rng, InputRange, OutputRange};

    /// `is_partitioned`.
    pub trait STLInputPartitionExt: InputRange {
        fn is_partitioned<Predicate>(&self, pred: Predicate) -> bool
        where
            Predicate: Fn(&Self::Element) -> bool;
    }

    impl<R> STLInputPartitionExt for R
    where
        R: InputRange + ?Sized,
    {
        fn is_partitioned<Predicate>(&self, pred: Predicate) -> bool
        where
            Predicate: Fn(&Self::Element) -> bool,
        {
            rng::is_partitioned(self, pred)
        }
    }

    /// `partition`.
    pub trait STLOutputPartitonExt: OutputRange {
        fn partition<Predicate>(&mut self, pred: Predicate) -> Self::Position
        where
            Predicate: Fn(&Self::Element) -> bool;
    }

    impl<R> STLOutputPartitonExt for R
    where
        R: OutputRange + ?Sized,
    {
        fn partition<Predicate>(&mut self, pred: Predicate) -> Self::Position
        where
            Predicate: Fn(&Self::Element) -> bool,
        {
            rng::partition(self, pred)
        }
    }
}
