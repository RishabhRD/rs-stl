// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::algo::collection_ext::CollectionExt;
use crate::collections::ReversedCollection;
use crate::BidirectionalCollection;
use crate::Collection;
use crate::ReorderableCollection;
use crate::ReorderableCollectionExt;

/// Algorithms for `BidirectionalCollection`.
pub trait BidirectionalCollectionExt: BidirectionalCollection
where
    Self::Whole: BidirectionalCollection,
{
    /// Reverses the order of elements in `self`.
    ///
    /// # Complexity:
    ///   O(n)
    ///
    /// # Example
    /// ```
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// arr.reverse();
    /// assert_eq!(arr, [3, 2, 1]);
    /// ```
    fn reverse(&mut self)
    where
        Self: ReorderableCollection,
        Self::Whole: ReorderableCollection,
    {
        let mut s = self.full_mut();
        while s.count() > 1 {
            s.swap_at(&s.start(), &s.prior(s.end()));
            s.drop_first();
            s.drop_last();
        }
    }

    /// Returns a collection whose elements are in reversed order of given collection.
    ///
    /// # Postcondition:
    ///   - No allocations are done for forming collection.
    ///
    /// # Complexity:
    ///   - O(1).
    fn reversed(self) -> ReversedCollection<Self>
    where
        Self: Sized,
    {
        ReversedCollection::new(self)
    }

    /*-----------------Numeric Algorithms-----------------*/

    /// Returns the result of combining elements of given collection using given
    /// accumulation operation from right to left.
    ///
    /// # Postcondition
    ///   - Result is `(e0 + ... + (e(n-1) + (e(n) + init)))`.
    ///     where e1, e2, ..., en are the references to collection elements,
    ///     where (a + b) represents op(a, b).
    ///
    /// # Complexity:
    ///   - O(`count`)
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// assert_eq!(arr.fold_right(0, |x, y| x + y), 6);
    /// ```
    fn fold_right<R, F>(&self, init: R, mut op: F) -> R
    where
        F: FnMut(&Self::Element, R) -> R,
    {
        let mut res = init;
        let mut rest = self.full();
        while let Some(e) = rest.pop_last() {
            res = op(&e, res)
        }
        res
    }
}

impl<R> BidirectionalCollectionExt for R
where
    R: BidirectionalCollection + ?Sized,
    R::Whole: BidirectionalCollection,
{
}
