// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{RandomAccessCollection, ReorderableCollection};
mod sort;

/// Algorithms for `RandomAccessCollection`.
pub trait RandomAccessCollectionExt: RandomAccessCollection
where
    Self::Whole: RandomAccessCollection,
{
    /*-----------------Sorting Algorithms-----------------*/

    /// Sorts the collection in place, using the given predicate as comparision between elements.
    ///
    /// # Precondition:
    ///   - `are_in_increasing_order` should follow strict weak ordering.
    ///
    /// # Postcondition:
    ///   - Relative ordering of equivalent elements are NOT guaranteed to be presevered.
    ///
    /// # Complexity:
    ///   - O(n * log(n)) worst case where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [3, 4, 1, 2, 5];
    /// arr.sort_unstable_by(|x, y| x < y);
    /// assert_eq!(arr, [1, 2, 3, 4, 5]);
    /// ```
    fn sort_unstable_by<Compare>(&mut self, are_in_increasing_order: Compare)
    where
        Self: ReorderableCollection,
        Self::Whole: ReorderableCollection,
        Compare: Fn(&Self::Element, &Self::Element) -> bool + Clone,
    {
        sort::sort_unstable_by(self, are_in_increasing_order);
    }

    /// Sorts the collection in place.
    ///
    /// # Postcondition:
    ///   - Relative ordering of equivalent elements are NOT guaranteed to be presevered.
    ///
    /// # Complexity:
    ///   - O(n * log(n)) worst case where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [3, 4, 1, 2, 5];
    /// arr.sort_unstable();
    /// assert_eq!(arr, [1, 2, 3, 4, 5]);
    /// ```
    fn sort_unstable(&mut self)
    where
        Self: ReorderableCollection,
        Self::Whole: ReorderableCollection,
        Self::Element: Ord,
    {
        self.sort_unstable_by(|x, y| x < y)
    }
}

impl<R> RandomAccessCollectionExt for R
where
    R: RandomAccessCollection + ?Sized,
    R::Whole: RandomAccessCollection,
{
}
