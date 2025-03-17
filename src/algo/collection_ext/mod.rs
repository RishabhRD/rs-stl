// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::Collection;

pub trait CollectionExt: Collection {
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
    ///
    /// let arr = [1, 2, 3];
    /// let i = arr.find_if(|x| *x == 3);
    /// assert_eq!(i, 2);
    /// ```
    fn find_if<Pred>(&self, pred: Pred) -> Self::Position
    where
        Pred: Fn(&Self::Element) -> bool,
    {
        let mut start = self.start();
        let end = self.end();
        while start != end {
            if pred(self.at(&start)) {
                return start;
            }
            start = self.after(start);
        }
        start
    }

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
    /// let arr = [1, 2, 3];
    /// let mut sum = 0;
    /// arr.for_each(|x| sum = sum + x);
    /// assert_eq!(sum, 6);
    /// ```
    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&Self::Element),
    {
        let mut start = self.start();
        let end = self.end();
        while start != end {
            f(self.at(&start));
            start = self.after(start);
        }
    }
}

impl<R> CollectionExt for R where R: Collection + ?Sized {}
