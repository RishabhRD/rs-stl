// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, Slice};

/// Algorithms for `Collection`.
pub trait CollectionExt: Collection {
    /*-----------------Iteration Algorithms-----------------*/

    /// Applies f to each element of collection.
    ///
    /// # Complexity:
    ///   - O(n) where `n == self.count()`.
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
            f(&self.at(&start));
            start = self.next(start);
        }
    }

    /*-----------------Element Access Algorithms-----------------*/

    /// Returns the first element, or nil if `self` is empty.
    fn first(&self) -> Option<<Self as Collection>::ElementRef<'_>> {
        if self.start() == self.end() {
            None
        } else {
            Some(self.at(&self.start()))
        }
    }

    /*-----------------Slice Algorithms-----------------*/

    /// Returns slice of the collection covering full collection.
    ///
    /// # Complexity
    ///   - O(1).
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let s = arr.full();
    /// assert!(s.equals(&[1, 2, 3, 4, 5]));
    /// ```
    fn full(&self) -> Slice<Self::Whole> {
        self.slice(self.start(), self.end())
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
    /// let arr = [1, 2, 3, 4, 5];
    /// let s = arr.prefix(3);
    /// assert!(s.equals(&[1, 2, 3]));
    /// ```
    fn prefix(&self, max_length: usize) -> Slice<Self::Whole> {
        let mut end = self.start();
        self.form_next_n_limited_by(&mut end, max_length, self.end());
        self.prefix_upto(end)
    }

    /// Returns prefix slice of the collection ending at `to` exclusive.
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
    /// let arr = [1, 2, 3, 4, 5];
    /// let p = arr.prefix_upto(3);
    /// assert!(p.equals(&[1, 2, 3]));
    /// ```
    fn prefix_upto(&self, pos: Self::Position) -> Slice<Self::Whole> {
        self.slice(self.start(), pos)
    }

    /// Returns prefix slice of the collection ending at `to` inclusive.
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
    /// let arr = [1, 2, 3, 4, 5];
    /// let p = arr.prefix_through(3);
    /// assert!(p.equals(&[1, 2, 3, 4]));
    /// ```
    fn prefix_through(&self, pos: Self::Position) -> Slice<Self::Whole> {
        self.prefix_upto(self.next(pos))
    }

    /// Returns prefix slice containing the initial elements until `predicate` returns false and
    /// skipping the remaining elements.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 3, 5, 2, 7];
    /// let p = arr.prefix_while(|x| x % 2 == 1);
    /// assert!(p.equals(&[1, 3, 5]));
    /// ```
    fn prefix_while<F: FnMut(&Self::Element) -> bool>(
        &self,
        mut predicate: F,
    ) -> Slice<Self::Whole> {
        let p = self.first_position_where(|x| !predicate(x));
        self.prefix_upto(p)
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
    /// let arr = [1, 3, 5, 2, 4, 7];
    /// let s = arr.drop_while(|x| x % 2 == 1);
    /// assert!(s.equals(&[2, 4, 7]));
    /// ```
    fn drop_while<F>(&self, mut predicate: F) -> Slice<Self::Whole>
    where
        F: FnMut(&Self::Element) -> bool,
    {
        self.suffix_from(self.first_position_where(|x| !predicate(x)))
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
    fn drop(&self, n: usize) -> Slice<Self::Whole> {
        let mut start = self.start();
        self.form_next_n_limited_by(&mut start, n, self.end());
        self.suffix_from(start)
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
    /// let arr = [1, 2, 3, 4, 5];
    /// let s = arr.suffix(3);
    /// assert!(s.equals(&[3, 4, 5]));
    /// ```
    fn suffix(&self, max_length: usize) -> Slice<Self::Whole> {
        let n = self.count();
        if max_length > n {
            self.full()
        } else {
            self.suffix_from(self.next_n(self.start(), n - max_length))
        }
    }

    /// Returns suffix slice of the collection starting from `from` inclusive.
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
    /// let arr = [1, 2, 3, 4, 5];
    /// let s = arr.suffix_from(3);
    /// assert!(s.equals(&[4, 5]));
    /// ```
    fn suffix_from(&self, from: Self::Position) -> Slice<Self::Whole> {
        self.slice(from, self.end())
    }

    /*-----------------Equality algorithms-----------------*/

    /// Returns true if elements of self is equivalent to elements of other by given relation bi_pred.
    ///
    /// # Postcondition
    ///   - Returns true if elements of self is equivalent to other by given relation bi_pred.
    ///   - If self and other have different number of elements, then return false.
    ///
    /// # Complexity
    ///   - `O(min(m, n))`
    ///     where
    ///     - `m == self.count()`
    ///     - `n == other.count()`
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
        mut bi_pred: F,
    ) -> bool
    where
        OtherCollection: Collection,
        F: FnMut(&Self::Element, &OtherCollection::Element) -> bool,
    {
        let mut self1 = self.full();
        let mut other1 = other.full();
        loop {
            match (self1.pop_first(), other1.pop_first()) {
                (Some(x), Some(y)) if bi_pred(&x, &y) => {}
                (None, None) => return true,
                _ => return false,
            }
        }
    }

    /// Returns true if elements of self is equal to elements of other.
    ///
    /// # Postcondition
    ///   - Returns true if elements of self is equal to elements of other.
    ///   - If self and other have different number of elements, then return false.
    ///
    /// # Complexity
    ///   - `O(min(m, n))`
    ///     where
    ///     - `m == self.count()`
    ///     - `n == other.count()`
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

    /*-----------------Find Algorithms-----------------*/

    /// Finds position of first element in `self` satisfying `pred`. If no such
    /// element exists, returns `self.end()`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let i = arr.first_position_where(|x| *x == 3);
    /// assert_eq!(i, 2);
    /// ```
    fn first_position_where<Pred>(&self, mut pred: Pred) -> Self::Position
    where
        Pred: FnMut(&Self::Element) -> bool,
    {
        let mut rest = self.full();
        while let Some((p, e)) = rest.pop_first_with_pos() {
            if pred(&e) {
                return p;
            }
        }
        self.end()
    }

    /// Finds position of first element in `self` equals `e`. If no such element
    /// exists, returns `self.end()`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 3];
    /// let i = arr.first_position_of(&3);
    /// assert_eq!(i, 2);
    /// ```
    fn first_position_of(&self, e: &Self::Element) -> Self::Position
    where
        Self::Element: Eq,
    {
        self.first_position_where(|x| x == e)
    }

    /// Finds position of last element in `self` satisfying `pred`. If no such
    /// element exists, returns `self.end()`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4];
    /// let i = arr.last_position_where(|x| x % 2 == 1);
    /// assert_eq!(i, 2);
    /// ```
    fn last_position_where<Pred>(&self, mut pred: Pred) -> Self::Position
    where
        Pred: FnMut(&Self::Element) -> bool,
    {
        let mut rest = self.full();
        let mut res = self.end();
        while let Some((p, e)) = rest.pop_first_with_pos() {
            if pred(&e) {
                res = p;
            }
        }
        res
    }

    /// Finds position of `last` element equals `e`. If no such element exist,
    /// return `self.end()`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 3, 3];
    /// let i = arr.last_position_of(&3);
    /// assert_eq!(i, 2);
    /// ```
    fn last_position_of(&self, e: &Self::Element) -> Self::Position
    where
        Self::Element: Eq,
    {
        self.last_position_where(|x| x == e)
    }

    /*-----------------Predicate Satisfication Algorithms-----------------*/

    /// Returns true if all element in `self` satisfies `pred`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 3, 5];
    /// assert!(arr.all_satisfy(|x| x % 2 == 1));
    /// ```
    fn all_satisfy<Pred>(&self, mut pred: Pred) -> bool
    where
        Pred: FnMut(&Self::Element) -> bool,
    {
        let mut cur = self.full();
        while let Some(e) = cur.pop_first() {
            if !pred(&e) {
                return false;
            }
        }
        true
    }

    /// Returns true if atleast one element in `self` satisfies `pred`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 5];
    /// assert!(arr.any_satisfy(|x| x % 2 == 1));
    /// ```
    fn any_satisfy<Pred>(&self, mut pred: Pred) -> bool
    where
        Pred: FnMut(&Self::Element) -> bool,
    {
        let mut cur = self.full();
        while let Some(e) = cur.pop_first() {
            if pred(&e) {
                return true;
            }
        }
        false
    }

    /// Returns true if none of elements in `self` satisfy `pred`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [2, 4, 6];
    /// assert!(arr.none_satisfy(|x| x % 2 == 1));
    /// ```
    fn none_satisfy<Pred>(&self, mut pred: Pred) -> bool
    where
        Pred: FnMut(&Self::Element) -> bool,
    {
        let mut cur = self.full();
        while let Some(e) = cur.pop_first() {
            if pred(&e) {
                return false;
            }
        }
        true
    }

    /*-----------------Count Algorithms-----------------*/

    /// Returns number of elements in `self` satisfying `pred`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let n = arr.count_where(|x| x % 2 == 1);
    /// assert_eq!(n, 2);
    /// ```
    fn count_where<Pred>(&self, mut pred: Pred) -> usize
    where
        Pred: FnMut(&Self::Element) -> bool,
    {
        let mut cur = self.full();
        let mut count = 0;
        while let Some(e) = cur.pop_first() {
            if pred(&e) {
                count += 1;
            }
        }
        count
    }

    /// Returns number of elements in `self` equals `e`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [3, 0, 3];
    /// let n = arr.count_of(&3);
    /// assert_eq!(n, 2);
    /// ```
    fn count_of(&self, e: &Self::Element) -> usize
    where
        Self::Element: Eq,
    {
        self.count_where(|x| x == e)
    }

    /*-----------------Partition Find Algorithms-----------------*/

    /// Returns position of first element of collection for which predicate returns false.
    ///
    /// # Precondition
    ///   - The collection should be already partitioned wrt predicate i.e,
    ///     there exist a position `i` such that predicate is true for every
    ///     element of `self.prefix_upto(i)` and predicate is false for every
    ///     element of `self.suffix_from(i)`.
    ///
    /// # Complexity
    ///   - O(log n) for RandomAccessCollection, O(n) otherwise; where `n == self.count()`.
    ///   - O(log n) application of `belongs_in_second_half`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 3, 5, 2, 4];
    /// let i = arr.partition_point(|x| x % 2 == 0);
    /// assert_eq!(i, 3);
    /// ```
    fn partition_point<F>(
        &self,
        mut belongs_in_second_half: F,
    ) -> Self::Position
    where
        F: FnMut(&Self::Element) -> bool,
    {
        let mut f = self.start();
        let mut n = self.count();
        while n > 0 {
            let half = n / 2;
            let m = self.next_n(f.clone(), half);
            if belongs_in_second_half(&self.at(&m)) {
                n = half;
            } else {
                f = self.next(m);
                n -= half + 1;
            }
        }
        f
    }
}

impl<R> CollectionExt for R where R: Collection + ?Sized {}
