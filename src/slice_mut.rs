// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::marker::PhantomData;

use crate::{iterators::*, *};

/// A safe mutable view into unsafe subsequence of a collection.
pub struct SliceMut<'a, MutableSubSequence>
where
    MutableSubSequence: UnsafeReorderableSubSequence<MutableSubSequence = MutableSubSequence>
        + 'a,
{
    /// The unsafe subsequence.
    subsequence: MutableSubSequence,

    /// A marker data managing lifetime of `self`.
    _marker: PhantomData<&'a ()>,
}

impl<'a, MutableSubSequence> SliceMut<'a, MutableSubSequence>
where
    MutableSubSequence: UnsafeReorderableSubSequence<MutableSubSequence = MutableSubSequence>
        + 'a,
{
    /// Returns a slice for unsafe subsequence `s`.
    pub unsafe fn new(s: MutableSubSequence) -> Self {
        SliceMut {
            subsequence: s,
            _marker: PhantomData,
        }
    }

    /// Removes and returns first element.
    ///
    /// The removed element becomes independent of `self` and can therefore be used
    /// in parallel with `self`.
    pub fn pop_first(&mut self) -> MutableSubSequence::ElementRef<'a> {
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
    pub fn pop_last(&mut self) -> MutableSubSequence::ElementRef<'a>
    where
        MutableSubSequence: BidirectionalCollection,
        MutableSubSequence::SubSequence: BidirectionalCollection,
    {
        precondition!(!self.subsequence.is_empty());
        let i = self.subsequence.prior(self.subsequence.end());
        let r = unsafe { self.subsequence.unsafe_at(&i) };
        unsafe { self.subsequence.set_end(i) };
        r
    }

    /// Removes and returns a subsequence of elements starting from `self.start()` upto but not
    /// including `p`.
    pub fn pop_prefix_upto(&mut self, p: MutableSubSequence::Position) -> Self {
        let s = unsafe {
            self.subsequence
                .unsafe_slice(self.subsequence.start(), p.clone())
        };
        unsafe { self.subsequence.set_start(p) };
        unsafe { Self::new(s) }
    }
}

/// Drop algorithms
impl<'a, MutableSubSequence> SliceMut<'a, MutableSubSequence>
where
    MutableSubSequence:
        UnsafeMutableSubSequence<MutableSubSequence = MutableSubSequence> + 'a,
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
        MutableSubSequence: BidirectionalCollection,
        MutableSubSequence::SubSequence: BidirectionalCollection,
    {
        _ = self.pop_last();
    }

    /// Removes subsequence of elements starting from `self.start()` upto but not
    /// including `p`.
    fn drop_prefix_upto(&mut self, p: MutableSubSequence::Position) {
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
    pub fn drop_prefix_through(&mut self, p: MutableSubSequence::Position) {
        _ = self.pop_prefix_through(p);
    }

    /// Removes the longest prefix of `self` whose elements satisfy `p`.
    ///
    /// # Complexity
    ///   - Atmost `self.count()` applications of `p`.
    pub fn drop_while<Predicate>(&mut self, p: Predicate)
    where
        Predicate: FnMut(&MutableSubSequence::Element) -> bool,
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
        MutableSubSequence: BidirectionalCollection,
        MutableSubSequence::SubSequence: BidirectionalCollection,
    {
        _ = self.pop_end(n);
    }

    /// Removes subsequence of last elements from `self` starting from `p`.
    pub fn drop_suffix_from(&mut self, p: MutableSubSequence::Position) {
        _ = self.pop_suffix_from(p);
    }
}

/// Pop algorithms
impl<'a, MutableSubSequence> SliceMut<'a, MutableSubSequence>
where
    MutableSubSequence: UnsafeReorderableSubSequence<MutableSubSequence = MutableSubSequence>
        + 'a,
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
    pub fn pop_prefix_through(
        &mut self,
        p: MutableSubSequence::Position,
    ) -> Self {
        self.pop_prefix_upto(self.next(p))
    }

    /// Removes and returns the longest prefix of `self` whose elements satisfy `p`.
    ///
    /// # Complexity
    ///   - Atmost `self.count()` applications of `p`.
    pub fn pop_while<Predicate>(&mut self, mut p: Predicate) -> Self
    where
        Predicate: FnMut(&MutableSubSequence::Element) -> bool,
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
        MutableSubSequence: BidirectionalCollection,
        MutableSubSequence::SubSequence: BidirectionalCollection,
    {
        let mut f = self.end();
        self.form_prior_n_limited_by(&mut f, n, self.start());
        self.pop_suffix_from(f)
    }

    /// Removes and returns subsequence of last elements from `self` starting from `p`.
    pub fn pop_suffix_from(&mut self, p: MutableSubSequence::Position) -> Self {
        let mut s = self.pop_prefix_upto(p);
        std::mem::swap(self, &mut s);
        s
    }
}

/// Splitting algorithms.
impl<'a, MutableSubSequence> SliceMut<'a, MutableSubSequence>
where
    MutableSubSequence: UnsafeReorderableSubSequence<MutableSubSequence = MutableSubSequence>
        + 'a,
{
    /// Splits `self` into two subsequences at position `p`:
    /// - the left part contains elements before `p`,
    /// - the right part contains elements starting at `p`.
    ///
    /// # Complexity
    ///   - O(1).
    pub fn split_at(mut self, p: MutableSubSequence::Position) -> (Self, Self) {
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
    pub fn split_after(
        self,
        mut p: MutableSubSequence::Position,
    ) -> (Self, Self) {
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
    ) -> SplitWhereIterator<'a, MutableSubSequence, Predicate>
    where
        Predicate: FnMut(&MutableSubSequence::Element) -> bool,
        Self: Sized,
    {
        SplitWhereIterator::new(self, p)
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
    ) -> SplitEvenlyIterator<'a, MutableSubSequence> {
        precondition!(n > 0);
        let c = self.count();
        if c == 0 {
            return SplitEvenlyIterator::new(self, 0, 0, 0);
        }
        let num_slices = match min_size == 0 {
            true => n,
            false => usize::min(usize::max(c / min_size, 1), n),
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
    pub fn split_evenly_in(
        self,
        n: usize,
    ) -> SplitEvenlyIterator<'a, MutableSubSequence> {
        precondition!(n > 0);
        self.split_evenly_in_with_min_size(n, 0)
    }
}

unsafe impl<'a, MutableSubSequence> Send for SliceMut<'a, MutableSubSequence> where
    MutableSubSequence: UnsafeReorderableSubSequence<MutableSubSequence = MutableSubSequence>
        + 'a
{
}

impl<'a, MutableSubSequence> Collection for SliceMut<'a, MutableSubSequence>
where
    MutableSubSequence: UnsafeReorderableSubSequence<MutableSubSequence = MutableSubSequence>
        + UnsafeSubSequence
        + 'a,
{
    type Position = MutableSubSequence::Position;

    type Element = MutableSubSequence::Element;

    type ElementRef<'b>
        = MutableSubSequence::ElementRef<'b>
    where
        Self: 'b;

    type SubSequence = MutableSubSequence::SubSequence;

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

impl<'a, SubSequence> LazyCollection for SliceMut<'a, SubSequence>
where
    SubSequence: LazyCollection
        + UnsafeReorderableSubSequence<MutableSubSequence = SubSequence>
        + 'a,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.subsequence.compute_at(i)
    }
}

impl<'a, SubSequence> BidirectionalCollection for SliceMut<'a, SubSequence>
where
    SubSequence: BidirectionalCollection
        + UnsafeReorderableSubSequence<MutableSubSequence = SubSequence>
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

impl<'a, SubSequence> RandomAccessCollection for SliceMut<'a, SubSequence> where
    SubSequence: RandomAccessCollection
        + UnsafeReorderableSubSequence<MutableSubSequence = SubSequence>
        + 'a
{
}
