// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::algo::collection_ext::CollectionExt;
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
    /// let mut s = arr.full_mut();
    /// let start = s.start();
    /// *s.at_mut(&start) = 0;
    /// assert!(s.equals(&[0, 2, 3, 4, 5]));
    /// assert!(arr.equals(&[0, 2, 3, 4, 5]));
    /// ```
    fn full_mut(&mut self) -> SliceMut<Self::Whole> {
        self.slice_mut(self.start(), self.end())
    }

    /// Returns a slice, upto specified maximum length, containing the initial elements of
    /// collection.
    ///
    /// # Postcondition
    ///   - If the maximum length exceeds the number of elements in the
    ///     collection, the result contains all elements in the collection.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; otherwise O(k) where k is the
    ///     number of elements in resultant slice.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3, 4, 5];
    /// let s = arr.prefix_mut(3);
    /// assert!(s.equals(&[1, 2, 3]));
    /// ```
    fn prefix_mut(&mut self, max_length: usize) -> SliceMut<Self::Whole> {
        let mut end = self.start();
        self.form_next_n_limited_by(&mut end, max_length, self.end());
        self.prefix_upto_mut(end)
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
    /// let mut p = arr.prefix_upto_mut(3);
    /// let start = p.start();
    /// *p.at_mut(&start) = 0;
    /// assert!(p.equals(&[0, 2, 3]));
    /// assert!(arr.equals(&[0, 2, 3, 4, 5]));
    /// ```
    fn prefix_upto_mut(&mut self, to: Self::Position) -> SliceMut<Self::Whole> {
        self.slice_mut(self.start(), to)
    }

    /// Returns mutable prefix slice of the collection ending at `to` inclusive.
    ///
    /// # Precondition
    ///   - `to` is a valid position in the collection and `to != self.end()`.
    ///
    /// # Complexity
    ///   - O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3, 4, 5];
    /// let p = arr.prefix_through_mut(3);
    /// assert!(p.equals(&[1, 2, 3, 4]));
    /// ```
    fn prefix_through_mut(
        &mut self,
        pos: Self::Position,
    ) -> SliceMut<Self::Whole> {
        let next = self.next(pos);
        self.prefix_upto_mut(next)
    }

    /// Returns mutable prefix slice containing the initial elements until `predicate` returns
    /// false skipping the remaining elements.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 3, 5, 2, 7];
    /// let p = arr.prefix_while_mut(|x| x % 2 == 1);
    /// assert!(p.equals(&[1, 3, 5]));
    /// ```
    fn prefix_while_mut<F: FnMut(&Self::Element) -> bool>(
        &mut self,
        mut predicate: F,
    ) -> SliceMut<Self::Whole> {
        let p = self.first_position_where(|x| !predicate(x));
        self.prefix_upto_mut(p)
    }

    /// Returns a slice by skipping initial elements while `predicate` returns true,
    /// returning the remaining elements.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 3, 5, 2, 4, 7];
    /// let s = arr.drop_while_mut(|x| x % 2 == 1);
    /// assert!(s.equals(&[2, 4, 7]));
    /// ```
    fn drop_while_mut<F>(&mut self, mut predicate: F) -> SliceMut<Self::Whole>
    where
        F: FnMut(&Self::Element) -> bool,
    {
        self.suffix_from_mut(self.first_position_where(|x| !predicate(x)))
    }

    /// Returns a slice containing all but the given number of initial elements.
    ///
    /// # Postcondition
    ///   - If `n > self.count()`, returns an empty slice.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection;
    ///   - O(k) otherwise, where k is number of elements to drop from beginning of collection.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let s = arr.drop(3);
    /// assert!(s.equals(&[4, 5]));
    /// ```
    fn drop_mut(&mut self, n: usize) -> SliceMut<Self::Whole> {
        let mut start = self.start();
        self.form_next_n_limited_by(&mut start, n, self.end());
        self.suffix_from_mut(start)
    }

    /// Returns a slice, upto specified maximum length, containing the final elements of the
    /// collection.
    ///
    /// # Postcondition
    ///   - If the maximum length exceeds the number of elements in the
    ///     collection, the result contains all elements in the collection.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; otherwise O(n) where
    ///     `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3, 4, 5];
    /// let s = arr.suffix_mut(3);
    /// assert!(s.equals(&[3, 4, 5]));
    /// ```
    fn suffix_mut(&mut self, max_length: usize) -> SliceMut<Self::Whole> {
        let n = self.count();
        if max_length > n {
            self.full_mut()
        } else {
            self.suffix_from_mut(self.next_n(self.start(), n - max_length))
        }
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
    /// let mut s = arr.suffix_from_mut(3);
    /// let start = s.start();
    /// *s.at_mut(&start) = 0;
    /// assert!(s.equals(&[0, 5]));
    /// assert!(arr.equals(&[1, 2, 3, 0, 5]));
    /// ```
    fn suffix_from_mut(
        &mut self,
        from: Self::Position,
    ) -> SliceMut<Self::Whole> {
        self.slice_mut(from, self.end())
    }
}

impl<R> ReorderableCollectionExt for R
where
    R: ReorderableCollection + ?Sized,
    R::Whole: ReorderableCollection,
{
}
