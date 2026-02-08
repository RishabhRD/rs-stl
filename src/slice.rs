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

    /// Panics if position is out of bounds of slice for reading element.
    ///
    /// # Complexity
    ///   - O(1).
    fn assert_bounds_check_read(&self, position: &Whole::Position) {
        if *position < self.from || *position >= self.to {
            panic!("Out of bounds read to slice.");
        }
    }

    /// Panics if position is out of bounds of slice for defining sub-slice.
    ///
    /// # Complexity
    ///   - O(1).
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
    /// # Complexity
    ///   - O(1).
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
    /// # Complexity
    ///   - O(1).
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

    /// Removes first `n` elements from `self` if `self` has atleast `n` elements; otherwise make
    /// `self` empty.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection;
    ///   - O(n) otherwise.
    pub fn drop(&mut self, n: usize) {
        let mut new_from = self.from.clone();
        self._whole
            .form_next_n_limited_by(&mut new_from, n, self.to.clone());
        self.from = new_from;
    }

    /// Removes the subsequence from start position of `self` up to, but not including `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn drop_prefix_upto(&mut self, position: Whole::Position) {
        self.assert_bounds_check_slice(&position);
        self.from = position;
    }

    /// Removes the subsequence from start position of `self` through (including) `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn drop_prefix_through(&mut self, position: Whole::Position) {
        self.assert_bounds_check_read(&position);
        self.from = self._whole.next(position);
    }

    /// Removes the longest prefix of `self` whose elements satisfy `p`.
    ///
    /// # Complexity
    ///   - Atmost `self.count()` applications of `p`.
    pub fn drop_while<Predicate>(&mut self, mut p: Predicate)
    where
        Predicate: FnMut(&Whole::Element) -> bool,
    {
        self.from = self.first_position_where(|e| !p(e)).unwrap_or(self.end());
    }

    /// Removes last `n` elements from `self`. If `self` has less than `n` elements, make it empty.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`;
    ///   - O(`self.count()`) otherwise.
    pub fn drop_suffix(&mut self, n: usize) {
        let c = self.count();
        if n > c {
            self.to = self.from.clone()
        } else {
            self.to = self.next_n(self.start(), c - n)
        }
    }

    /// Removes last elements from `self` starting from `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn drop_suffix_from(&mut self, p: Whole::Position) {
        self.assert_bounds_check_slice(&p);
        self.to = p;
    }
}

/// Pop algorithms.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Removes and yields the first element if non-empty; returns None otherwise.
    ///
    /// # Complexity:
    ///   - O(1).
    pub fn pop_first(&mut self) -> Option<Whole::ElementRef<'a>> {
        let f = self.from.clone();
        if self.drop_first() {
            Some(self._whole.at(&f))
        } else {
            None
        }
    }

    /// Removes and yields the last element if non-empty; returns None otherwise.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn pop_last(&mut self) -> Option<Whole::ElementRef<'a>>
    where
        Whole: BidirectionalCollection,
    {
        let t = self.to.clone();
        if self.drop_last() {
            Some(self._whole.at(&self.prior(t)))
        } else {
            None
        }
    }

    /// Removes and returns subsequence of first `n` elements in `self`; If `self` has less than `n`
    /// elements, make `self` empty and return all elements of `self`.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection;
    ///   - O(n) otherwise.
    pub fn pop(&mut self, n: usize) -> Self {
        let mut f = self.from.clone();
        self.form_next_n_limited_by(&mut f, n, self.to.clone());
        self.pop_prefix_upto(f)
    }

    /// Removes and returns the subsequence from start position of `self` up to, but not including `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn pop_prefix_upto(&mut self, p: Whole::Position) -> Self {
        self.assert_bounds_check_slice(&p);
        let prefix = Self {
            _whole: self._whole,
            from: self.from.clone(),
            to: p.clone(),
        };
        self.from = p;
        prefix
    }

    /// Removes and returns the subsequence from start position of `self` through (including) `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn pop_prefix_through(&mut self, p: Whole::Position) -> Self {
        self.pop_prefix_upto(self.next(p))
    }

    /// Removes and returns the longest prefix of `self` whose elements satisfy `p`.
    ///
    /// # Complexity
    ///   - Atmost `self.count()` applications of `p`.
    pub fn pop_while<Predicate>(&mut self, mut p: Predicate) -> Self
    where
        Predicate: FnMut(&Whole::Element) -> bool,
    {
        let p = self.first_position_where(|e| !p(e)).unwrap_or(self.end());
        self.pop_prefix_upto(p)
    }

    /// Removes and returns subsequence of last `n` elements from `self`.
    /// If `self` has less than `n` elements, make it empty and returns
    /// subsequence of all elements.
    ///
    /// # Complexity
    ///   - O(n).
    pub fn pop_end(&mut self, n: usize) -> Self {
        let c = self.count();
        let i = if n > c {
            self.from.clone()
        } else {
            self.next_n(self.start(), c - n)
        };
        self.pop_suffix_from(i)
    }

    /// Removes and returns subsequence of last elements from `self` starting from `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn pop_suffix_from(&mut self, p: Whole::Position) -> Self {
        let mut s = self.pop_prefix_upto(p);
        std::mem::swap(self, &mut s);
        s
    }
}

/// Splitting algorithms.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Splits `self` into two subsequences at position `p`:
    /// - the left part contains elements before `p`,
    /// - the right part contains elements starting at `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn split_at(mut self, p: Whole::Position) -> (Self, Self) {
        let r = self.pop_prefix_upto(p);
        (r, self)
    }

    /// Splits `self` into two subsequences at position `p`:
    /// - the left part contains elements before `p`,
    /// - the right part contains elements starting at `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn split_after(self, mut position: Whole::Position) -> (Self, Self) {
        self.form_next(&mut position);
        self.split_at(position)
    }

    /// Returns an iterator over subsequences of `self`, split at elements
    /// where `p` returns `true`.
    ///
    /// # Note
    ///   - Consecutive elements for which `p` returns `true` produce empty subsequences.
    ///
    /// # Complexity
    ///   - O(`self.count()`).
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 3, 5, 2, 2, 2, 3, 4, 5, 7];
    /// let v: Vec<_> =
    ///   arr.full_mut()
    ///      .split_where(|x| x % 2 == 0)
    ///      .map(|s| s.to_vec())
    ///      .collect();
    /// assert_eq!(v, vec![vec![1, 3, 5], vec![], vec![], vec![3], vec![5, 7]]);
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

    /// Returns an iterator over at most `n` subsequences of `self`, each of size
    /// at least `min_size`, splitting as evenly as possible.
    ///
    /// If the elements cannot be divided evenly, the earlier subsequences are
    /// one element larger than the later ones.
    ///
    /// # Precondition
    ///   - `n > 0`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`;
    ///   - O(`self.count()`) otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3, 4, 5, 6, 7];
    /// let splits: Vec<Vec<_>> = arr.full_mut()
    ///     .split_evenly_in_with_min_size(3, 2)
    ///     .map(|s| s.to_vec())
    ///     .collect();
    /// assert_eq!(splits, vec![vec![1, 2, 3], vec![4, 5], vec![6, 7]]);
    /// ```
    pub fn split_evenly_in_with_min_size(
        self,
        n: usize,
        min_size: usize,
    ) -> SplitEvenlyIterator<'a, Whole> {
        let c = self.count();
        if c == 0 {
            return SplitEvenlyIterator::new(self, 0, 0, 0);
        }
        let num_slices = if min_size == 0 {
            n
        } else {
            usize::min(usize::max(c / min_size, 1), n)
        };

        let slice_size = c / num_slices;
        let num_bigger_slices = c % num_slices;

        SplitEvenlyIterator::new(
            self,
            num_slices,
            slice_size,
            num_bigger_slices,
        )
    }

    /// Returns an iterator over `n` subsequences of `self`, split as evenly as possible.
    ///
    /// If the elements cannot be divided evenly, the earlier subsequences are
    /// one element larger than the later ones.
    ///
    /// # Precondition
    ///   - `n > 0`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`;
    ///   - O(`self.count()`) otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3, 4, 5, 6, 7];
    /// let splits: Vec<Vec<_>> = arr.full_mut()
    ///     .split_evenly_in(3)
    ///     .map(|s| s.to_vec())
    ///     .collect();
    /// assert_eq!(splits, vec![vec![1, 2, 3], vec![4, 5], vec![6, 7]]);
    pub fn split_evenly_in(self, n: usize) -> SplitEvenlyIterator<'a, Whole> {
        self.split_evenly_in_with_min_size(n, 0)
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
