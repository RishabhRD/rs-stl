// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::marker::PhantomData;

use crate::{
    iterators::{SplitEvenlyIteratorMut, SplitWhereIteratorMut},
    BidirectionalCollection, Collection, CollectionExt, LazyCollection,
    MutableCollection, RandomAccessCollection, ReorderableCollection, Slice,
};

/// A contiguous mutable sub-collection of a mutable collection.
#[derive(PartialEq, Eq)]
pub struct SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Pointer to the whole collection.
    _whole: *mut Whole,

    /// Phantom data to bind the lifetime to struct.
    _phantom: PhantomData<&'a mut Whole>,

    /// Start position of slice.
    from: Whole::Position,

    /// End position of slice.
    to: Whole::Position,
}

/// Base accessor algorithms.
impl<'a, Whole> SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Creates a new instance of slice with given collection and position range.
    pub fn new(
        collection: &'a mut Whole,
        from: Whole::Position,
        to: Whole::Position,
    ) -> Self {
        Self {
            _whole: collection as *mut Whole,
            _phantom: PhantomData,
            from,
            to,
        }
    }

    /// Yields whole collection wrapped by `self`.
    ///
    /// # Complexity
    ///   - O(1).
    fn whole(&self) -> &'a Whole {
        unsafe { &mut *self._whole }
    }

    /// Yields whole collection wrapped by `self`.
    ///
    /// # Complexity
    ///   - O(1).
    fn whole_mut(&mut self) -> &'a mut Whole {
        unsafe { &mut *self._whole }
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
impl<Whole> SliceMut<'_, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Removes the first element if non-empty and returns true; returns false otherwise.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn drop_first(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            let f = self.next(self.from.clone());
            self.from = f;
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
            let t = self.prior(self.to.clone());
            self.to = t;
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
        let mut f = self.from.clone();
        self.form_next_n_limited_by(&mut f, n, self.to.clone());
        self.from = f;
    }

    /// Removes the subsequence from start position of `self` up to, but not including `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn drop_prefix_upto(&mut self, p: Whole::Position) {
        self.assert_bounds_check_slice(&p);
        self.from = p;
    }

    /// Removes the subsequence from start position of `self` through (including) `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn drop_prefix_through(&mut self, p: Whole::Position) {
        self.drop_prefix_upto(self.next(p))
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
    pub fn drop_end(&mut self, n: usize) {
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
impl<'a, Whole> SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Removes and yields the first element if non-empty; returns None otherwise.
    ///
    /// # Complexity:
    ///   - O(1).
    pub fn pop_first(&mut self) -> Option<Whole::ElementRef<'a>> {
        let f = self.from.clone();
        if self.drop_first() {
            Some(unsafe { &mut *self._whole }.at(&f))
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
            Some(unsafe { &mut *self._whole }.at(&self.prior(t)))
        } else {
            None
        }
    }

    /// Removes and yields the mutable first element if non-empty; returns None otherwise.
    ///
    /// # Complexity:
    ///   - O(1).
    pub fn pop_first_mut(&mut self) -> Option<&'a mut Whole::Element>
    where
        Whole: MutableCollection,
    {
        let f = self.from.clone();
        if self.drop_first() {
            Some(unsafe { &mut *self._whole }.at_mut(&f))
        } else {
            None
        }
    }

    /// Removes and yields the mutable last element if non-empty; returns None otherwise.
    pub fn pop_last_mut(&mut self) -> Option<&'a mut Whole::Element>
    where
        Whole: BidirectionalCollection + MutableCollection,
    {
        let t = self.prior(self.to.clone());
        if self.drop_last() {
            Some(unsafe { &mut *self._whole }.at_mut(&t))
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
            _phantom: PhantomData,
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
impl<'a, Whole> SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
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
    pub fn split_after(self, mut p: Whole::Position) -> (Self, Self) {
        self.form_next(&mut p);
        self.split_at(p)
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
    pub fn split_where<Predicate>(
        self,
        p: Predicate,
    ) -> SplitWhereIteratorMut<'a, Whole, Predicate>
    where
        Predicate: FnMut(&Whole::Element) -> bool,
        Self: Sized,
    {
        SplitWhereIteratorMut::new(self, p)
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
    ) -> SplitEvenlyIteratorMut<'a, Whole> {
        let c = self.count();
        if c == 0 {
            return SplitEvenlyIteratorMut::new(self, 0, 0, 0);
        }
        let num_slices = if min_size == 0 {
            n
        } else {
            usize::min(usize::max(c / min_size, 1), n)
        };

        let slice_size = c / num_slices;
        let num_bigger_slices = c % num_slices;

        SplitEvenlyIteratorMut::new(
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
    pub fn split_evenly_in(
        self,
        n: usize,
    ) -> SplitEvenlyIteratorMut<'a, Whole> {
        self.split_evenly_in_with_min_size(n, 0)
    }
}

unsafe impl<'a, Whole> Send for SliceMut<'a, Whole> where
    Whole: ReorderableCollection<Whole = Whole> + Send
{
}

impl<Whole> Collection for SliceMut<'_, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
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
        self.whole().form_next(i);
    }

    fn form_next_n(&self, i: &mut Self::Position, n: usize) {
        self.whole().form_next_n(i, n);
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.whole().form_next_n_limited_by(position, n, limit)
    }

    fn next(&self, i: Self::Position) -> Self::Position {
        self.whole().next(i)
    }

    fn next_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.whole().next_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.whole().distance(from, to)
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        self.assert_bounds_check_read(i);
        self.whole().at(i)
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole> {
        self.assert_bounds_check_slice(&from);
        self.assert_bounds_check_slice(&to);
        Slice::new(self.whole(), from, to)
    }
}

impl<Whole> LazyCollection for SliceMut<'_, Whole>
where
    Whole: LazyCollection<Whole = Whole> + ReorderableCollection,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.assert_bounds_check_read(i);
        self.whole().compute_at(i)
    }
}

impl<Whole> BidirectionalCollection for SliceMut<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole> + ReorderableCollection,
{
    fn form_prior(&self, i: &mut Self::Position) {
        self.whole().form_prior(i);
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        self.whole().form_prior_n(i, n);
    }

    fn prior(&self, i: Self::Position) -> Self::Position {
        self.whole().prior(i)
    }

    fn prior_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.whole().prior_n(i, n)
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.whole().form_prior_n_limited_by(position, n, limit)
    }
}

impl<Whole> RandomAccessCollection for SliceMut<'_, Whole> where
    Whole: RandomAccessCollection<Whole = Whole> + ReorderableCollection
{
}

impl<Whole> ReorderableCollection for SliceMut<'_, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.assert_bounds_check_read(i);
        self.assert_bounds_check_read(j);
        self.whole_mut().swap_at(i, j);
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::Whole> {
        self.assert_bounds_check_slice(&from);
        self.assert_bounds_check_slice(&to);
        SliceMut::new(self.whole_mut(), from, to)
    }
}

impl<Whole> MutableCollection for SliceMut<'_, Whole>
where
    Whole: MutableCollection<Whole = Whole>,
{
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        self.assert_bounds_check_read(i);
        self.whole_mut().at_mut(i)
    }
}
