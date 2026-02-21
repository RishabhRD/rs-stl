// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::marker::PhantomData;

use crate::{iterators::*, *};

/// A safe view into unsafe subsequence of a collection.
pub struct SliceMut<'a, C>
where
    C: UnsafeSubSequence + ReorderableCollection<SubSequence = C> + 'a,
{
    /// The unsafe subsequence.
    subsequence: C,

    /// A marker data managing lifetime of `self`.
    _marker: PhantomData<&'a ()>,
}

impl<'a, C> SliceMut<'a, C>
where
    C: UnsafeSubSequence + ReorderableCollection<SubSequence = C> + 'a,
{
    /// Returns a slice for unsafe subsequence `s`.
    pub unsafe fn new(s: C::SubSequence) -> Self {
        SliceMut {
            subsequence: s,
            _marker: PhantomData,
        }
    }

    /// Returns the wrapped unsafe subsequence
    pub unsafe fn subsequence(&self) -> C::SubSequence {
        let s = self.subsequence.start();
        let e = self.subsequence.end();
        self.subsequence.unsafe_slice(s, e)
    }

    /// Removes and returns first element.
    ///
    /// The removed element becomes independent of `self` and can therefore be used
    /// in parallel with `self`.
    pub fn pop_first(
        &mut self,
    ) -> <C::SubSequence as Collection>::ElementRef<'a> {
        precondition!(!self.is_empty());
        let mut i = self.start();
        let r = unsafe { self.subsequence.unsafe_at(&i) };
        self.subsequence.form_next(&mut i);
        unsafe { self.subsequence.set_start(i) };
        r
    }

    /// Removes and returns last element.
    ///
    /// The removed element becomes independent of `self` and can therefore be used
    /// in parallel with `self`.
    pub fn pop_last(&mut self) -> <C::SubSequence as Collection>::ElementRef<'a>
    where
        C: BidirectionalCollection,
        C::SubSequence: BidirectionalCollection,
        C::SubSequence: BidirectionalCollection,
    {
        precondition!(!self.subsequence.is_empty());
        let i = self.subsequence.prior(self.subsequence.end());
        let r = unsafe { self.subsequence.unsafe_at(&i) };
        unsafe { self.subsequence.set_end(i) };
        r
    }

    /// Removes and returns mutable reference to first element.
    ///
    /// The removed element becomes independent of `self` and can therefore be used
    /// in parallel with `self`.
    pub fn pop_first_mut(&mut self) -> &'a mut C::Element
    where
        C: MutableCollection,
        C::SubSequence: MutableCollection,
    {
        precondition!(!self.is_empty());
        let mut i = self.start();
        let r = unsafe { self.subsequence.unsafe_at_mut(&i) };
        self.subsequence.form_next(&mut i);
        unsafe { self.subsequence.set_start(i) };
        r
    }

    /// Removes and returns mutable reference last element.
    ///
    /// The removed element becomes independent of `self` and can therefore be used
    /// in parallel with `self`.
    pub fn pop_last_mut(&mut self) -> &'a mut C::Element
    where
        C: MutableCollection + BidirectionalCollection,
        C::SubSequence: MutableCollection + BidirectionalCollection,
    {
        precondition!(!self.subsequence.is_empty());
        let i = self.subsequence.prior(self.subsequence.end());
        let r = unsafe { self.subsequence.unsafe_at_mut(&i) };
        unsafe { self.subsequence.set_end(i) };
        r
    }

    /// Removes and returns a subsequence of elements starting from `self.start()` upto but not
    /// including `p`.
    pub fn pop_prefix_upto(&mut self, p: C::Position) -> Self {
        let s = unsafe {
            self.subsequence
                .unsafe_slice(self.subsequence.start(), p.clone())
        };
        unsafe { self.subsequence.set_start(p) };
        unsafe { Self::new(s) }
    }
}

/// Drop algorithms
impl<'a, C> SliceMut<'a, C>
where
    C: UnsafeSubSequence + ReorderableCollection<SubSequence = C> + 'a,
{
    /// Removes the first element.
    ///
    /// - Precondition: `self` is not empty.
    pub fn drop_first(&mut self) {
        _ = self.pop_first();
    }

    /// Removes the last element.
    ///
    /// - Precondition: `self` is not empty.
    pub fn drop_last(&mut self)
    where
        C: BidirectionalCollection,
        C::SubSequence: BidirectionalCollection,
        C::SubSequence: BidirectionalCollection,
    {
        _ = self.pop_last();
    }

    /// Removes subsequence of elements starting from `self.start()` upto but not
    /// including `p`.
    pub fn drop_prefix_upto(&mut self, p: C::Position) {
        _ = self.pop_prefix_upto(p)
    }

    /// Removes subsequence of first `n` elements in `self`;
    /// If `self` has less than `n` elements, make `self` empty and return all elements of `self`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`;
    ///   - O(`n`) otherwise.
    pub fn drop(&mut self, n: usize) {
        _ = self.pop(n);
    }

    /// Removes the subsequence from start position of `self` through (including) `p`.
    ///
    /// - Precondition: Next position of `p` should be well defined.
    pub fn drop_prefix_through(&mut self, p: C::Position) {
        _ = self.pop_prefix_through(p);
    }

    /// Removes the longest prefix of `self` whose elements satisfy `p`.
    ///
    /// # Complexity
    ///   - Atmost `self.count()` applications of `p`.
    pub fn drop_while<Predicate>(&mut self, p: Predicate)
    where
        Predicate: FnMut(&C::Element) -> bool,
    {
        _ = self.pop_while(p);
    }

    /// Removes subsequence of last `n` elements from `self`.
    /// If `self` has less than `n` elements, make it empty and returns
    /// subsequence of all elements.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`;
    ///   - O(`n`) otherwise.
    pub fn drop_end(&mut self, n: usize)
    where
        C: BidirectionalCollection,
        C::SubSequence: BidirectionalCollection,
        C::SubSequence: BidirectionalCollection,
    {
        _ = self.pop_end(n);
    }

    /// Removes subsequence of last elements from `self` starting from `p`.
    pub fn drop_suffix_from(&mut self, p: C::Position) {
        _ = self.pop_suffix_from(p);
    }
}

/// Pop algorithms
impl<'a, C> SliceMut<'a, C>
where
    C: UnsafeSubSequence + ReorderableCollection<SubSequence = C> + 'a,
{
    /// Removes and returns subsequence of first `n` elements in `self`;
    /// If `self` has less than `n` elements, make `self` empty and return all elements of `self`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`;
    ///   - O(`n`) otherwise.
    pub fn pop(&mut self, n: usize) -> Self {
        let mut f = self.subsequence.start();
        self.form_next_n_limited_by(&mut f, n, self.end());
        self.pop_prefix_upto(f)
    }

    /// Removes and returns the subsequence from start position of `self` through (including) `p`.
    ///
    /// - Precondition: Next position of `p` should be well defined.
    pub fn pop_prefix_through(&mut self, p: C::Position) -> Self {
        self.pop_prefix_upto(self.next(p))
    }

    /// Removes and returns the longest prefix of `self` whose elements satisfy `p`.
    ///
    /// # Complexity
    ///   - Atmost `self.count()` applications of `p`.
    pub fn pop_while<Predicate>(&mut self, mut p: Predicate) -> Self
    where
        Predicate: FnMut(&C::Element) -> bool,
    {
        let p = self.first_position_where(|e| !p(e)).unwrap_or(self.end());
        self.pop_prefix_upto(p)
    }

    /// Removes and returns subsequence of last `n` elements from `self`.
    /// If `self` has less than `n` elements, make it empty and returns
    /// subsequence of all elements.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`;
    ///   - O(`n`) otherwise.
    pub fn pop_end(&mut self, n: usize) -> Self
    where
        C: BidirectionalCollection,
        C::SubSequence: BidirectionalCollection,
        C::SubSequence: BidirectionalCollection,
    {
        let mut f = self.end();
        self.form_prior_n_limited_by(&mut f, n, self.start());
        self.pop_suffix_from(f)
    }

    /// Removes and returns subsequence of last elements from `self` starting from `p`.
    pub fn pop_suffix_from(&mut self, p: C::Position) -> Self {
        let mut s = self.pop_prefix_upto(p);
        std::mem::swap(self, &mut s);
        s
    }
}

/// Splitting algorithms.
impl<'a, C> SliceMut<'a, C>
where
    C: UnsafeSubSequence + ReorderableCollection<SubSequence = C> + 'a,
{
    /// Splits `self` into two subsequences at position `p`:
    /// - the left part contains elements before `p`,
    /// - the right part contains elements starting at `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn split_at(mut self, p: C::Position) -> (Self, Self) {
        let r = self.pop_prefix_upto(p);
        (r, self)
    }

    /// Splits `self` into two subsequences at position `p`:
    /// - the left part contains elements before `p`,
    /// - the right part contains elements starting at `p`.
    ///
    /// # Precondition
    ///   - There must be a well defined position after `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn split_after(self, mut p: C::Position) -> (Self, Self) {
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
    ) -> SplitWhereIteratorMut<'a, C, Predicate>
    where
        Predicate: FnMut(&C::Element) -> bool,
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
    ) -> SplitEvenlyIteratorMut<'a, C> {
        precondition!(n > 0);
        let c = self.count();
        if c == 0 {
            return SplitEvenlyIteratorMut::new(self, 0, 0, 0);
        }
        let num_slices = match min_size == 0 {
            true => n,
            false => usize::min(usize::max(c / min_size, 1), n),
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
    pub fn split_evenly_in(self, n: usize) -> SplitEvenlyIteratorMut<'a, C> {
        precondition!(n > 0);
        self.split_evenly_in_with_min_size(n, 0)
    }
}

unsafe impl<'a, C> Send for SliceMut<'a, C> where
    C: UnsafeSubSequence + ReorderableCollection<SubSequence = C> + 'a
{
}

impl<'a, C> Collection for SliceMut<'a, C>
where
    C: UnsafeSubSequence + ReorderableCollection<SubSequence = C> + 'a,
{
    type Position = C::Position;

    type Element = C::Element;

    type ElementRef<'b>
        = <<C as Collection>::SubSequence as Collection>::ElementRef<'b>
    where
        Self: 'b;

    type SubSequence = C::SubSequence;

    fn start(&self) -> Self::Position {
        self.subsequence.start()
    }

    fn end(&self) -> Self::Position {
        self.subsequence.end()
    }

    fn form_next(&self, i: &mut Self::Position) {
        self.subsequence.form_next(i);
    }

    fn form_next_n(&self, i: &mut Self::Position, n: usize) {
        self.subsequence.form_next_n(i, n);
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.subsequence.form_next_n_limited_by(position, n, limit)
    }

    fn next(&self, i: Self::Position) -> Self::Position {
        self.subsequence.next(i)
    }

    fn next_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.subsequence.next_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.subsequence.distance(from, to)
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        unsafe { self.subsequence.unsafe_at(i) }
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::SubSequence> {
        unsafe { Slice::new(self.subsequence.unsafe_slice(from, to)) }
    }
}

impl<'a, C> LazyCollection for SliceMut<'a, C>
where
    C: UnsafeSubSequence
        + LazyCollection
        + ReorderableCollection<SubSequence = C>
        + 'a,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.subsequence.compute_at(i)
    }
}

impl<'a, C> BidirectionalCollection for SliceMut<'a, C>
where
    C: UnsafeSubSequence
        + BidirectionalCollection
        + ReorderableCollection<SubSequence = C>
        + 'a,
{
    fn form_prior(&self, i: &mut Self::Position) {
        self.subsequence.form_prior(i);
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        self.subsequence.form_prior_n(i, n);
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.subsequence.form_prior_n_limited_by(position, n, limit)
    }

    fn prior(&self, i: Self::Position) -> Self::Position {
        self.subsequence.prior(i)
    }

    fn prior_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.subsequence.prior_n(i, n)
    }
}

impl<'a, C> RandomAccessCollection for SliceMut<'a, C> where
    C: UnsafeSubSequence
        + RandomAccessCollection
        + ReorderableCollection<SubSequence = C>
        + 'a
{
}

impl<'a, C> ReorderableCollection for SliceMut<'a, C>
where
    C: UnsafeSubSequence
        + ReorderableCollection
        + ReorderableCollection<SubSequence = C>
        + 'a,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.subsequence.swap_at(i, j)
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::SubSequence> {
        unsafe { SliceMut::new(self.subsequence.unsafe_slice(from, to)) }
    }
}

impl<'a, C> MutableCollection for SliceMut<'a, C>
where
    C: UnsafeMutableSubSequence
        + MutableCollection
        + ReorderableCollection<SubSequence = C>
        + 'a,
{
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        unsafe { self.subsequence.unsafe_at_mut(i) }
    }
}
