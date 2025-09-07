// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::algo::collection_ext::CollectionExt;
use crate::iterators::LazyCollectionIter;
use crate::{BidirectionalCollection, Collection, LazyCollection};

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

    /*-----------------Iteration Algorithms-----------------*/

    /// Returns an iterator to iterate over lazyily computed elements in collection.
    fn lazy_iter(&self) -> LazyCollectionIter<'_, Self::Whole> {
        LazyCollectionIter::new(self.full())
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

    /*-----------------Partition Algorithms-----------------*/

    /// Returns two Vec containing the elements of the collection that
    /// don’t and do satisfy the given predicate, respectively.
    ///
    /// # Postcondition
    ///   - Returns `(falseVec, trueVec)` where `falseVec` contains all elements
    ///     that don't satisfy predicate and `trueVec` contains all elements
    ///     that do satisfy predicate.
    ///   - Relative ordering of elements is preserved in both Vec.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = 1..=5;
    /// let (evens, odds) = arr.lazy_partitioned(|x| x % 2 == 1);
    /// assert_eq!(evens, [2, 4]);
    /// assert_eq!(odds, [1, 3, 5]);
    /// ```
    fn lazy_partitioned<F>(
        &self,
        mut belongs_in_second_half: F,
    ) -> (Vec<Self::Element>, Vec<Self::Element>)
    where
        F: FnMut(&Self::Element) -> bool,
    {
        let n = self.count();
        let mut left = Vec::with_capacity(n);
        let arr = left.spare_capacity_mut();
        let mut left_idx = 0;
        let mut right_idx = n;

        for e in self.lazy_iter() {
            if belongs_in_second_half(&e) {
                right_idx -= 1;
                arr[right_idx].write(e);
            } else {
                arr[left_idx].write(e);
                left_idx += 1;
            }
        }

        unsafe {
            left.set_len(n);
        }

        let mut right = left.split_off(right_idx);
        right.reverse();

        (left, right)
    }

    /*-----------------Numeric Algorithms-----------------*/

    /// Returns the result of combining elements of given collection using given
    /// accumulation operation from left to right.
    ///
    /// # Postcondition
    ///   - Result is `(((init + e1) + e2) + ... + en)`.
    ///     where e1, e2, ..., en are the lazily computed values of collection,
    ///     where (a + b) represents op(a, b).
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

    /// Returns the result of combining elements of given collection using given
    /// accumulation operation from right to left.
    ///
    /// # Postcondition
    ///   - Result is `(e0 + ... + (e(n-1) + (e(n) + init)))`.
    ///     where e1, e2, ..., en are the references to collection elements,
    ///     where (a + b) represents op(a, b).
    ///
    /// # Complexity:
    ///   - O(`count`)
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = 1..=3;
    /// assert_eq!(arr.lazy_fold_right(0, |x, y| x + y), 6);
    /// ```
    fn lazy_fold_right<R, F>(&self, init: R, mut op: F) -> R
    where
        F: FnMut(Self::Element, R) -> R,
        Self: BidirectionalCollection,
        Self::Whole: BidirectionalCollection,
    {
        let mut res = init;
        let mut rest = self.full();
        while let Some(e) = rest.lazy_pop_last() {
            res = op(e, res)
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
