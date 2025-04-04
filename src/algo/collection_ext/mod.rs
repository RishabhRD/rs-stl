// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, Slice};

pub trait CollectionExt: Collection {
    /// Returns the first element, or nil if `self` is empty.
    fn first(&self) -> Option<&<Self as Collection>::Element> {
        if self.start() == self.end() {
            None
        } else {
            Some(self.at(&self.start()))
        }
    }

    /// Returns slice of the collection covering full collection.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Returns slice of the collection covering full collection.
    ///   - Complexity: O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let s = arr.all();
    /// assert!(s.equals(&[1, 2, 3, 4, 5]));
    /// ```
    fn all(&self) -> Slice<Self::Whole> {
        self.slice(self.start(), self.end())
    }

    /// Returns prefix slice of the collection ending at `to` exclusive.
    ///
    /// # Precondition
    ///   - `to` is a valid position in the collection.
    ///
    /// # Postcondition
    ///   - Returns prefix slice of the collection ending at `to` exclusive.
    ///   - Complexity: O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let p = arr.prefix(3);
    /// assert!(p.equals(&[1, 2, 3]));
    /// ```
    fn prefix(&self, to: Self::Position) -> Slice<Self::Whole> {
        self.slice(self.start(), to)
    }

    /// Returns suffix slice of the collection starting from `from` inclusive.
    ///
    /// # Precondition
    ///   - `from` is a valid position in the collection.
    ///
    /// # Postcondition
    ///   - Returns suffix slice of the collection starting from `from` inclusive.
    ///   - Complexity: O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let s = arr.suffix(3);
    /// assert!(s.equals(&[4, 5]));
    /// ```
    fn suffix(&self, from: Self::Position) -> Slice<Self::Whole> {
        self.slice(from, self.end())
    }

    /// Returns true if elements of self is equivalent to elements of other by given relation bi_pred.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Returns true if elements of self is equivalent to other by given relation bi_pred.
    ///   - If self and other have different number of elements, then return false.
    ///   - Complexity: `O(min(m, n))`
    ///     where
    ///     - `m == self.size()`
    ///     - `n == other.size()`
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr1 = [1, 2, 3];
    /// let arr2 = [2, 3, 4];
    /// assert!(arr1.equals_by(&arr2, |x, y| *y == x + 1));
    /// ```
    fn equals_by<OtherCollection, F>(
        &self,
        other: &OtherCollection,
        bi_pred: F,
    ) -> bool
    where
        OtherCollection: Collection,
        F: Fn(&Self::Element, &OtherCollection::Element) -> bool,
    {
        let mut self1 = self.all();
        let mut other1 = other.all();
        loop {
            match (self1.pop_first(), other1.pop_first()) {
                (Some(x), Some(y)) if bi_pred(x, y) => {}
                (None, None) => return true,
                _ => return false,
            }
        }
    }

    /// Returns true if elements of self is equal to elements of other.
    ///
    /// # Precondition
    ///
    /// # Postcondition
    ///   - Returns true if elements of self is equal to elements of other.
    ///   - If self and other have different number of elements, then return false.
    ///   - Complexity: `O(min(m, n))`
    ///     where
    ///     - `m == self.size()`
    ///     - `n == other.size()`
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr1 = [1, 2, 3];
    /// let arr2 = [1, 2, 3];
    /// assert!(arr1.equals(&arr2));
    /// ```
    fn equals<OtherCollection>(&self, other: &OtherCollection) -> bool
    where
        OtherCollection: Collection<Element = Self::Element>,
        Self::Element: Eq,
    {
        self.equals_by(other, |x, y| x == y)
    }

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
        let mut rest = self.all();
        while let Some(x) = rest.first() {
            if pred(x) {
                break;
            }
            rest.drop_first();
        }
        rest.start()
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
