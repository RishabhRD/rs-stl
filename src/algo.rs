// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, CollectionLifetime, MutableCollection};

mod find;
mod for_each;

pub trait CollectionExtension: Collection {
    /// Finds position of first element satisfying predicate.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Returns position of first element in self satisfying pred.
    ///   - Returns end position if no such element exists.
    ///   - Complexity: O(n). Maximum `n` applications of `pred`.
    ///
    ///     where n is number of elements in self.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    /// use algo::*;
    ///
    /// let arr = [1, 2, 3];
    /// let i = arr.find_if(|x| *x == 3);
    /// assert_eq!(i, 2);
    /// ```
    fn find_if<Pred>(&self, pred: Pred) -> Self::Position
    where
        Pred: Fn(<Self as CollectionLifetime<'_>>::Element) -> bool,
    {
        find::find_if(self, pred)
    }

    /// Finds position of first element not satisfying predicate.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Returns position of first element in self not satisfying pred.
    ///   - Returns end position if no such element exists.
    ///   - Complexity: O(n). Maximum `n` applications of `pred`.
    ///
    ///     where n is number of elements in self.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    /// use algo::*;
    ///
    /// let arr = [1, 2, 3];
    ///
    /// let i = arr.find_if_not(|x| *x == 3);
    /// assert_eq!(i, 0);
    /// ```
    fn find_if_not<Pred>(&self, pred: Pred) -> Self::Position
    where
        Pred: Fn(<Self as CollectionLifetime<'_>>::Element) -> bool,
    {
        find::find_if_not(self, pred)
    }

    /// Applies op on each element of collection.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Applies op on each element of collection.
    ///   - Complexity: O(n). Exactly n applications of op.
    ///
    ///     where n is number of elements in self.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    /// use algo::*;
    ///
    /// let arr = [1, 2, 3];
    /// let mut sum = 0;
    /// arr.for_each(|x| sum += x);
    /// assert_eq!(sum, 6);
    /// ```
    fn for_each<Op>(&self, op: Op)
    where
        Op: FnMut(<Self as CollectionLifetime<'_>>::Element),
    {
        for_each::for_each(self, op)
    }
}

impl<R> CollectionExtension for R where R: Collection + ?Sized {}

pub trait MutableCollectionExtension: MutableCollection
where
    for<'a> <Self as CollectionLifetime<'a>>::MutableSlice: MutableCollection,
{
    /// Applies op on each element of collection.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Applies op on each element of collection.
    ///   - Complexity: O(n). Exactly n applications of op.
    ///
    ///     where n is number of elements in self.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    /// use algo::*;
    ///
    /// let arr = [1, 2, 3];
    /// arr.for_each_mut(|x| *x += 1);
    /// assert_eq!(arr, [2, 3, 4]);
    /// ```
    fn for_each_mut<Op>(&mut self, op: Op)
    where
        Op: FnMut(<Self as CollectionLifetime<'_>>::MutableElement),
    {
        for_each::for_each_mut(self, op)
    }
}

impl<R> MutableCollectionExtension for R
where
    R: MutableCollection + ?Sized,
    for<'a> <Self as CollectionLifetime<'a>>::MutableSlice: MutableCollection,
{
}
