// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, ForwardRange};

/// Returns first position in range such that element at that position and element after
/// that position satisfies binary predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns first position in rng such that element at that
///     position and element after that position satisfies bi_pred.
///   - Returns end position if no such element is found.
///   - Complexity: O(n), maximum `n - 1` calls to `bi_pred`.
///     Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use rng::infix::*;
/// use stl::*;
///
/// let arr = [1, 1, 3];
///
/// let i = rng::adjacent_find_if(&arr, |x, y| x == y);
/// assert_eq!(i, 0);
/// let i = arr.adjacent_find_if(|x, y| x == y);
/// assert_eq!(i, 0);
/// ```
pub fn adjacent_find_if<Range, BinaryPred>(
    rng: &Range,
    bi_pred: BinaryPred,
) -> Range::Position
where
    Range: ForwardRange + ?Sized,
    BinaryPred: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::adjacent_find_if(rng, rng.start(), rng.end(), bi_pred)
}

pub mod infix {
    use crate::{rng, ForwardRange};

    /// `adjacent_find`.
    pub trait STLAdjacentFindExt: ForwardRange {
        fn adjacent_find_if<BinaryPred>(
            &self,
            bi_pred: BinaryPred,
        ) -> Self::Position
        where
            BinaryPred: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::adjacent_find_if(self, bi_pred)
        }
    }

    impl<R> STLAdjacentFindExt for R where R: ForwardRange + ?Sized {}
}
