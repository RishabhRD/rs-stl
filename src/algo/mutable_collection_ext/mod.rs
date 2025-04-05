// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::MutableCollection;

pub trait MutableCollectionExt: MutableCollection
where
    Self::Whole: MutableCollection,
{
    /// Applies f to each element of collection.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Applies f to each element of collection.
    ///   - Complexity: O(n). Exactly n applications of f.
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
}

impl<R> MutableCollectionExt for R
where
    R: MutableCollection + ?Sized,
    R::Whole: MutableCollection,
{
}
