// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange};

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

pub mod infix {
    use crate::{rng, InputRange};

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
}
