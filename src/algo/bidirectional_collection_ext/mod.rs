// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

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
}

impl<R> BidirectionalCollectionExt for R
where
    R: BidirectionalCollection + ?Sized,
    R::Whole: BidirectionalCollection,
{
}
