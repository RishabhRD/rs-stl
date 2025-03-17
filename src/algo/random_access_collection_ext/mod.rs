// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::RandomAccessCollection;

pub trait RandomAccessCollectionExt: RandomAccessCollection
where
    Self::SliceCore: RandomAccessCollection,
{
    /// Returns number of elements in the collection.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Returns number of elements in the collection.
    ///   - Complexity: O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// assert_eq!(arr.size(), 3);
    /// ```
    fn size(&self) -> usize {
        self.distance(self.start(), self.end())
    }
}

impl<R> RandomAccessCollectionExt for R
where
    R: RandomAccessCollection + ?Sized,
    R::SliceCore: RandomAccessCollection,
{
}
