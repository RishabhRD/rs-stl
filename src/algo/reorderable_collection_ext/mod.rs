// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ReorderableCollection, SliceMut};

/// Algorithms for `ReorderableCollection`.
pub trait ReorderableCollectionExt: ReorderableCollection
where
    Self::Whole: ReorderableCollection,
{
    /*-----------------Slice Algorithms-----------------*/

    /// Returns mutable slice of the collection covering full collection.
    ///
    /// # Complexity
    ///   - O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3, 4, 5];
    /// let mut s = arr.all_mut();
    /// let start = s.start();
    /// *s.at_mut(&start) = 0;
    /// assert!(s.equals(&[0, 2, 3, 4, 5]));
    /// assert!(arr.equals(&[0, 2, 3, 4, 5]));
    /// ```
    fn all_mut(&mut self) -> SliceMut<Self::Whole> {
        self.slice_mut(self.start(), self.end())
    }

    /// Returns mutable prefix slice of the collection ending at `to` exclusive.
    ///
    /// # Precondition
    ///   - `to` is a valid position in the collection.
    ///
    /// # Complexity
    ///   - O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3, 4, 5];
    /// let mut p = arr.prefix_mut(3);
    /// let start = p.start();
    /// *p.at_mut(&start) = 0;
    /// assert!(p.equals(&[0, 2, 3]));
    /// assert!(arr.equals(&[0, 2, 3, 4, 5]));
    /// ```
    fn prefix_mut(&mut self, to: Self::Position) -> SliceMut<Self::Whole> {
        self.slice_mut(self.start(), to)
    }

    /// Returns mutable suffix slice of the collection starting from `from` inclusive.
    ///
    /// # Precondition
    ///   - `from` is a valid position in the collection.
    ///
    /// # Complexity
    ///   - O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3, 4, 5];
    /// let mut s = arr.suffix_mut(3);
    /// let start = s.start();
    /// *s.at_mut(&start) = 0;
    /// assert!(s.equals(&[0, 5]));
    /// assert!(arr.equals(&[1, 2, 3, 0, 5]));
    /// ```
    fn suffix_mut(&mut self, from: Self::Position) -> SliceMut<Self::Whole> {
        self.slice_mut(from, self.end())
    }
}

impl<R> ReorderableCollectionExt for R
where
    R: ReorderableCollection + ?Sized,
    R::Whole: ReorderableCollection,
{
}
