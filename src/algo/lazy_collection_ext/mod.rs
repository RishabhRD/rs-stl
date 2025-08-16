// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, LazyCollection};

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
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
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

    /*-----------------Numeric Algorithms-----------------*/

    /// Returns the result of combining elements of given collection using given
    /// accumulation operation from left to right.
    ///
    /// # Postcondition
    ///   - Result is `(((init + e1) + e2) + ... + en)`.
    ///     where e1, e2, ..., en are the lazily computed values of collection.
    ///
    /// # Complexity:
    ///   - O(`count`)
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = 1..=3;
    /// assert_eq!(arr.lazy_fold_left(0, |x, y| x + y), 6);
    /// ```
    fn lazy_fold_left<R, F>(&self, init: R, mut op: F) -> R
    where
        F: FnMut(R, Self::Element) -> R,
    {
        let mut res = init;
        for e in self.lazy_iter() {
            res = op(res, e)
        }
        res
    }
}

impl<R> LazyCollectionExt for R
where
    R: LazyCollection + ?Sized,
    R::Whole: LazyCollection,
{
}
