// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, LazyCollection, LazyCollectionIterator};

/// Algorithms for `LazyCollection`.
pub trait LazyCollectionExt: LazyCollection
where
    Self::Whole: LazyCollection,
{
    /// Returns the "lazily computed" first element, or nil if `self` is empty.
    fn lazy_first(&self) -> Option<<Self as Collection>::Element> {
        if self.start() == self.end() {
            None
        } else {
            Some(self.compute_at(&self.start()))
        }
    }

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

    /// Returns a non-consuming iterator that iterates over `Self::Element`.
    /// The iterator lazily computes the value on each `next` call.
    fn lazy_iter(&self) -> LazyCollectionIterator<Self::Whole> {
        LazyCollectionIterator::new(self.slice(self.start(), self.end()))
    }
}

impl<R> LazyCollectionExt for R
where
    R: LazyCollection + ?Sized,
    R::Whole: LazyCollection,
{
}
