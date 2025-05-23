// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::LazyCollection;

pub trait LazyCollectionExt: LazyCollection
where
    Self::Whole: LazyCollection,
{
    /// Applies f to each "lazily computed" element of collection.
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
    /// let arr = 1..4;
    /// let mut sum = 0;
    /// arr.lazy_for_each(|x| sum = sum + x);
    /// assert_eq!(sum, 6);
    /// ```
    fn lazy_for_each<F>(&self, mut f: F)
    where
        F: FnMut(Self::Element),
    {
        let mut start = self.start();
        let end = self.end();
        while start != end {
            f(self.compute_at(&start));
            start = self.next(start);
        }
    }
}

impl<R> LazyCollectionExt for R
where
    R: LazyCollection + ?Sized,
    R::Whole: LazyCollection,
{
}
