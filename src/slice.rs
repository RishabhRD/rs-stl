// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    iterators::{SplitEvenlyIterator, SplitWhereIterator},
    BidirectionalCollection, Collection, CollectionExt, LazyCollection,
    RandomAccessCollection,
};

/// A contiguous sub-collection of a collection.
#[derive(PartialEq, Eq)]
pub struct Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Reference to the whole collection.
    _whole: &'a Whole,

    /// Start position of slice.
    from: Whole::Position,

    /// End position of slice.
    to: Whole::Position,
}

impl<'a, Whole> Clone for Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    fn clone(&self) -> Self {
        Self {
            _whole: self._whole,
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}

// Base accessor algorithms.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Creates a new instance of slice with given collection and position range.
    pub fn new(
        collection: &'a Whole,
        from: Whole::Position,
        to: Whole::Position,
    ) -> Self {
        Self {
            _whole: collection,
            from,
            to,
        }
    }

    /// Returns the reference to whole collection.
    pub fn whole(&self) -> &'a Whole {
        self._whole
    }

    /// Panics if position is out of bounds of slice for reading element.
    fn assert_bounds_check_read(&self, position: &Whole::Position) {
        if *position < self.from || *position >= self.to {
            panic!("Out of bounds read to slice.");
        }
    }

    /// Panics if position is out of bounds of slice for defining sub-slice.
    fn assert_bounds_check_slice(&self, position: &Whole::Position) {
        if *position < self.from || *position > self.to {
            panic!("Out of bounds slicing to slice.");
        }
    }
}

/// Dropping algorithms
impl<Whole> Slice<'_, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Removes the first element if non-empty and returns true; returns false otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let mut s = arr.full();
    /// assert!(s.drop_first());
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn drop_first(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            self._whole.form_next(&mut self.from);
            true
        }
    }

    /// Removes the last element if non-empty and returns true; returns false otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let mut s = arr.full();
    /// assert!(s.drop_last());
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn drop_last(&mut self) -> bool
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            false
        } else {
            self._whole.form_prior(&mut self.to);
            true
        }
    }

    /// Drops prefix upto specified maximum length.
    ///
    /// # Postcondition
    ///   - If `max_length > self.count()`, make `self` empty.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection;
    ///   - O(n) otherwise, where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// s.drop_prefix(2);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn drop_prefix(&mut self, max_length: usize) {
        let mut new_from = self.from.clone();
        self._whole.form_next_n_limited_by(
            &mut new_from,
            max_length,
            self.to.clone(),
        );
        self.from = new_from;
    }

    /// Drops the prefix of slice upto given `position`.
    ///
    /// # Precondition
    ///   - `position` is valid position in `self`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// s.drop_prefix_upto(2);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn drop_prefix_upto(&mut self, position: Whole::Position) {
        self.assert_bounds_check_slice(&position);
        self.from = position;
    }

    /// Drops the prefix of slice till and including given `position`.
    ///
    /// # Precondition
    ///   - `position` is valid position in `self`.
    ///   - `position != self.end()`
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// s.drop_prefix_through(2);
    /// assert!(s.equals(&[3]));
    /// ```
    pub fn drop_prefix_through(&mut self, position: Whole::Position) {
        self.assert_bounds_check_read(&position);
        self.from = self._whole.next(position);
    }

    /// Drops the element of `self` while the elements satisfy given `predicate`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// s.drop_while(|x| *x < 2);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn drop_while<Pred>(&mut self, mut predicate: Pred)
    where
        Pred: FnMut(&Whole::Element) -> bool,
    {
        self.from = self
            .first_position_where(|e| !predicate(e))
            .unwrap_or(self.end());
    }

    /// Drops suffix upto specified maximum length.
    ///
    /// # Postcondition
    ///   - If `max_length > self.count()`, make `self` empty.
    ///
    /// # Complexity
    ///   - O(n), where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// s.drop_suffix(2);
    /// assert!(s.equals(&[0, 1]));
    /// ```
    pub fn drop_suffix(&mut self, max_length: usize) {
        let n = self.count();
        if max_length > n {
            self.to = self.from.clone()
        } else {
            self.to = self.next_n(self.start(), n - max_length)
        }
    }

    /// Drops suffix from given `position`.
    ///
    /// # Precondition
    ///   - `position` is a valid position in self.
    ///
    /// # Complexity
    ///   - O(n), where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// s.drop_suffix_from(2);
    /// assert!(s.equals(&[0, 1]));
    /// ```
    pub fn drop_suffix_from(&mut self, position: Whole::Position) {
        self.assert_bounds_check_slice(&position);
        self.to = position;
    }
}

/// Pop algorithms.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Removes and returns the first element if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let mut s = arr.full();
    /// let first = s.pop_first().unwrap();
    /// assert_eq!(*first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_first(&mut self) -> Option<Whole::ElementRef<'a>> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self._whole.at(&self.from));
            self._whole.form_next(&mut self.from);
            e
        }
    }

    /// Removes and returns the first element and its position if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let mut s = arr.full();
    /// let (first_pos, first) = s.pop_first_with_pos().unwrap();
    /// assert_eq!(first_pos, 0);
    /// assert_eq!(*first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_first_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::ElementRef<'a>)> {
        if self.from == self.to {
            None
        } else {
            let e = self._whole.at(&self.from);
            let p = self.from.clone();
            self._whole.form_next(&mut self.from);
            Some((p, e))
        }
    }

    /// Removes and returns the last element if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let mut s = arr.full();
    /// let last = s.pop_last().unwrap();
    /// assert_eq!(*last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn pop_last(&mut self) -> Option<Whole::ElementRef<'a>>
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self._whole.prior(self.to.clone());
            let e = Some(self._whole.at(&ele_pos));
            self._whole.form_prior(&mut self.to);
            e
        }
    }

    /// Removes and returns the last element and its position if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3];
    /// let mut s = arr.full();
    /// let (last_pos, last) = s.pop_last_with_pos().unwrap();
    /// assert_eq!(last_pos, 2);
    /// assert_eq!(*last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn pop_last_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::ElementRef<'a>)>
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self._whole.prior(self.to.clone());
            let e = self._whole.at(&ele_pos);
            self._whole.form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }

    /// Removes and returns prefix upto specified maximum length.
    ///
    /// # Postcondition
    ///   - If `max_length > self.count()`, make `self` empty and return the full slice as result.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection;
    ///   - O(n) otherwise, where `n == self.count()`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// let prefix = s.pop_prefix(2);
    /// assert!(prefix.equals(&[0, 1]));
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_prefix(&mut self, max_length: usize) -> Self {
        let old_from = self.from.clone();
        let mut new_from = self.from.clone();
        self._whole.form_next_n_limited_by(
            &mut new_from,
            max_length,
            self.to.clone(),
        );
        self.from = new_from;
        Self {
            _whole: self._whole,
            from: old_from,
            to: self.from.clone(),
        }
    }

    /// Removes and returns the prefix slice upto given `position`.
    ///
    /// # Precondition
    ///   - `position` is valid position in `self`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// let prefix = s.pop_prefix_upto(2);
    /// assert!(prefix.equals(&[0, 1]));
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_prefix_upto(&mut self, position: Whole::Position) -> Self {
        self.assert_bounds_check_slice(&position);
        let prefix = Self {
            _whole: self._whole,
            from: self.from.clone(),
            to: position.clone(),
        };
        self.from = position;
        prefix
    }

    /// Removes and returns the prefix till and including given `position`.
    ///
    /// # Precondition
    ///   - `position` is valid position in `self`.
    ///   - `position != self.end()`
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// let prefix = s.pop_prefix_through(2);
    /// assert!(prefix.equals(&[0, 1, 2]));
    /// assert!(s.equals(&[3]));
    /// ```
    pub fn pop_prefix_through(&mut self, position: Whole::Position) -> Self {
        self.assert_bounds_check_read(&position);
        let old_from = self.from.clone();
        self.from = self._whole.next(position);
        Self {
            _whole: self._whole,
            from: old_from,
            to: self.from.clone(),
        }
    }

    /// Removes and returns  the element of `self` till the first element satisfies `predicate` as a slice.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// let prefix = s.pop_while(|x| *x < 2);
    /// assert!(prefix.equals(&[0, 1]));
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_while<Pred>(&mut self, mut predicate: Pred) -> Self
    where
        Pred: FnMut(&Whole::Element) -> bool,
    {
        let p = self
            .first_position_where(|e| !predicate(e))
            .unwrap_or(self.end());
        let res = Slice {
            _whole: self._whole,
            from: self.from.clone(),
            to: p.clone(),
        };
        self.from = p;
        res
    }

    /// Removes and returns suffix upto specified maximum length.
    ///
    /// # Postcondition
    ///   - If `max_length > self.count()`, make `self` empty and returns the
    ///     full slice as suffix.
    ///
    /// # Complexity
    ///   - O(n), where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// let suffix = s.pop_suffix(2);
    /// assert!(s.equals(&[0, 1]));
    /// assert!(suffix.equals(&[2, 3]));
    /// ```
    pub fn pop_suffix(&mut self, max_length: usize) -> Self {
        let n = self.count();
        let old_to = self.to.clone();
        if max_length > n {
            self.to = self.from.clone()
        } else {
            self.to = self.next_n(self.start(), n - max_length)
        }
        Self {
            _whole: self._whole,
            from: self.to.clone(),
            to: old_to,
        }
    }

    /// Removes and returns suffix from given `position`.
    ///
    /// # Precondition
    ///   - `position` is a valid position in self.
    ///
    /// # Complexity
    ///   - O(n), where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3];
    /// let mut s = arr.full();
    /// let suffix = s.pop_suffix_from(2);
    /// assert!(s.equals(&[0, 1]));
    /// assert!(suffix.equals(&[2, 3]));
    /// ```
    pub fn pop_suffix_from(&mut self, position: Whole::Position) -> Self {
        self.assert_bounds_check_slice(&position);
        let old_to = self.to.clone();
        self.to = position;
        Self {
            _whole: self._whole,
            from: self.to.clone(),
            to: old_to,
        }
    }
}

/// Pop algorithms for `LazyCollection`.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: LazyCollection<Whole = Whole>,
{
    /// Removes and returns the lazily computed first element if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = 1..=3;
    /// let mut s = arr.full();
    /// let first = s.lazy_pop_first().unwrap();
    /// assert_eq!(first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn lazy_pop_first(&mut self) -> Option<Whole::Element> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self._whole.compute_at(&self.from));
            self._whole.form_next(&mut self.from);
            e
        }
    }

    /// Removes and returns the lazily computed first element and its position if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = 1..=3;
    /// let mut s = arr.full();
    /// let (first_pos, first) = s.lazy_pop_first_with_pos().unwrap();
    /// assert_eq!(first_pos, 1);
    /// assert_eq!(first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn lazy_pop_first_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::Element)> {
        if self.from == self.to {
            None
        } else {
            let e = self._whole.compute_at(&self.from);
            let p = self.from.clone();
            self._whole.form_next(&mut self.from);
            Some((p, e))
        }
    }

    /// Removes and returns the lazily computed last element if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = 1..=3;
    /// let mut s = arr.full();
    /// let last = s.lazy_pop_last().unwrap();
    /// assert_eq!(last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn lazy_pop_last(&mut self) -> Option<Whole::Element>
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self._whole.prior(self.to.clone());
            let e = Some(self._whole.compute_at(&ele_pos));
            self._whole.form_prior(&mut self.to);
            e
        }
    }

    /// Removes and returns the lazily computed last element and its position if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = 1..=3;
    /// let mut s = arr.full();
    /// let (last_pos, last) = s.lazy_pop_last_with_pos().unwrap();
    /// assert_eq!(last_pos, 3);
    /// assert_eq!(last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn lazy_pop_last_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::Element)>
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self._whole.prior(self.to.clone());
            let e = self._whole.compute_at(&ele_pos);
            self._whole.form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }
}

/// Splitting algorithms.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Returns two disjoint slices of `self` split at the given `position`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3, 4];
    /// let (s1, s2) = arr.full().split_at(2);
    /// assert!(s1.equals(&[0, 1]));
    /// assert!(s2.equals(&[2, 3, 4]));
    /// ```
    pub fn split_at(self, position: Whole::Position) -> (Self, Self) {
        self.assert_bounds_check_slice(&position);
        let prefix = Self {
            _whole: self._whole,
            from: self.from.clone(),
            to: position.clone(),
        };
        let suffix = Self {
            _whole: self._whole,
            from: position,
            to: self.to.clone(),
        };
        (prefix, suffix)
    }

    /// Returns two disjoint slices of `self`, split immediately *after* the
    /// given `position`.
    ///
    /// # Precondition
    ///   - `position != self.end()`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [0, 1, 2, 3, 4];
    /// let (s1, s2) = arr.full().split_after(2);
    /// assert!(s1.equals(&[0, 1, 2]));
    /// assert!(s2.equals(&[3, 4]));
    /// ```
    pub fn split_after(self, mut position: Whole::Position) -> (Self, Self) {
        self.form_next(&mut position);
        self.split_at(position)
    }

    /// Splits `self` into slices separated by elements that satisfy `pred` by
    /// returning an Iterator of slices.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 3, 5, 2, 2, 3, 4, 5, 5];
    ///
    /// // Store sum of each split.
    /// let mut res = vec![];
    /// arr.full()
    ///    .split_where(|x| x % 2 == 0)
    ///    .for_each(|s| res.push(s.iter().sum::<i32>()));
    /// assert_eq!(res, vec![9, 0, 3, 10]);
    /// ```
    pub fn split_where<Pred>(
        self,
        pred: Pred,
    ) -> SplitWhereIterator<'a, Whole, Pred>
    where
        Pred: FnMut(&Whole::Element) -> bool,
        Self: Sized,
    {
        SplitWhereIterator::new(self, pred)
    }

    /// Splits `self` into at max `max_slices` slices with each slice being of
    /// at min size of `min_size` by returning an Iterator of slices.
    ///
    /// # Precondition
    ///   - `max_slices > 0`,
    ///
    /// # Postcondition
    ///   - If splitting exactly evenly is not possible, then slices on start
    ///     would have bigger size than slices at end, still maintaining as even
    ///     splitting as possible.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; otherwise O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4, 5, 6, 7];
    /// let splits: Vec<Vec<_>> = arr.full()
    ///     .split_evenly_in_with_min_size(3, 2)
    ///     .map(|s| s.iter().copied().collect())
    ///     .collect();
    /// assert_eq!(splits, vec![vec![1, 2, 3], vec![4, 5], vec![6, 7]]);
    /// ```
    pub fn split_evenly_in_with_min_size(
        self,
        max_slices: usize,
        min_size: usize,
    ) -> SplitEvenlyIterator<'a, Whole> {
        let n = self.count();
        let num_slices = if min_size == 0 {
            max_slices
        } else {
            usize::min(usize::max(n / min_size, 1), max_slices)
        };

        let slice_size = n / num_slices;
        let num_bigger_slices = n % num_slices;

        SplitEvenlyIterator::new(
            self,
            num_slices,
            slice_size,
            num_bigger_slices,
        )
    }

    /// Splits `self` into `num_slices` slices by returning an Iterator of slices.
    ///
    /// # Precondition
    ///   - `num_slices > 0`,
    ///
    /// # Postcondition
    ///   - If splitting exactly evenly is not possible, then slices on start
    ///     would have bigger size than slices at end, still maintaining as even
    ///     splitting as possible.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; otherwise O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let arr = [1, 2, 3, 4, 5, 6, 7];
    /// let splits: Vec<Vec<_>> = arr.full()
    ///     .split_evenly_in(3)
    ///     .map(|s| s.iter().copied().collect())
    ///     .collect();
    /// assert_eq!(splits, vec![vec![1, 2, 3], vec![4, 5], vec![6, 7]]);
    /// ```
    pub fn split_evenly_in(
        self,
        num_slices: usize,
    ) -> SplitEvenlyIterator<'a, Whole> {
        self.split_evenly_in_with_min_size(num_slices, 0)
    }
}

unsafe impl<'a, Whole> Send for Slice<'a, Whole> where
    Whole: Collection<Whole = Whole> + Send
{
}

impl<Whole> Collection for Slice<'_, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    type Position = Whole::Position;

    type Element = Whole::Element;

    type ElementRef<'a>
        = Whole::ElementRef<'a>
    where
        Self: 'a;

    type Whole = Whole;

    fn start(&self) -> Self::Position {
        self.from.clone()
    }

    fn end(&self) -> Self::Position {
        self.to.clone()
    }

    fn form_next(&self, i: &mut Self::Position) {
        self._whole.form_next(i);
    }

    fn form_next_n(&self, i: &mut Self::Position, n: usize) {
        self._whole.form_next_n(i, n);
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self._whole.form_next_n_limited_by(position, n, limit)
    }

    fn next(&self, i: Self::Position) -> Self::Position {
        self._whole.next(i)
    }

    fn next_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self._whole.next_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self._whole.distance(from, to)
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        self.assert_bounds_check_read(i);
        self._whole.at(i)
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole> {
        self.assert_bounds_check_slice(&from);
        self.assert_bounds_check_slice(&to);
        Slice::new(self._whole, from, to)
    }
}

impl<Whole> LazyCollection for Slice<'_, Whole>
where
    Whole: LazyCollection<Whole = Whole>,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.assert_bounds_check_read(i);
        self._whole.compute_at(i)
    }
}

impl<Whole> BidirectionalCollection for Slice<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole>,
{
    fn form_prior(&self, i: &mut Self::Position) {
        self._whole.form_prior(i);
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        self._whole.form_prior_n(i, n);
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self._whole.form_prior_n_limited_by(position, n, limit)
    }

    fn prior(&self, i: Self::Position) -> Self::Position {
        self._whole.prior(i)
    }

    fn prior_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self._whole.prior_n(i, n)
    }
}

impl<Whole> RandomAccessCollection for Slice<'_, Whole> where
    Whole: RandomAccessCollection<Whole = Whole>
{
}
