// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::algo::reorderable_collection_ext::ReorderableCollectionExt;
use crate::{MutableCollection, MutableCollectionIter};

/// Algorithms for `MutableCollection`.
pub trait MutableCollectionExt: MutableCollection
where
    Self::Whole: MutableCollection,
{
    /*-----------------Iteration Algorithms-----------------*/

    /// Applies `f` to each element of collection.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// arr.for_each_mut(|e| *e = *e + 1);
    /// assert_eq!(arr, [2, 3, 4]);
    /// ```
    fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self::Element),
    {
        let mut start = self.start();
        let end = self.end();
        while start != end {
            f(self.at_mut(&start));
            start = self.next(start);
        }
    }

    /// Returns an iterator to iterate over mutable element refs in collection.
    fn iter_mut(&mut self) -> MutableCollectionIter<'_, Self::Whole> {
        MutableCollectionIter::new(self.full_mut())
    }
}

impl<R> MutableCollectionExt for R
where
    R: MutableCollection + ?Sized,
    R::Whole: MutableCollection,
{
}
