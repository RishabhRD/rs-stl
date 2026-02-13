// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::marker::PhantomData;

use crate::*;

/// A safe view into unsafe subsequence of a collection.
pub struct Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence> + 'a,
{
    /// The unsafe subsequence.
    subsequence: SubSequence,

    /// A marker data managing lifetime of `self`.
    _marker: PhantomData<&'a ()>,
}

impl<'a, SubSequence> Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence> + 'a,
{
    /// Returns a slice for unsafe subsequence `s`.
    pub unsafe fn new(s: SubSequence) -> Self {
        Slice {
            subsequence: s,
            _marker: PhantomData,
        }
    }

    /// Removes and returns first element.
    ///
    /// The removed element becomes independent of `self` and can therefore be used
    /// in parallel with `self`.
    fn pop_first(&mut self) -> SubSequence::ElementRef<'a> {
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
    fn pop_last(&mut self) -> SubSequence::ElementRef<'a>
    where
        SubSequence: BidirectionalCollection,
    {
        precondition!(!self.subsequence.is_empty());
        let i = self.subsequence.prior(self.subsequence.end());
        let r = unsafe { self.subsequence.unsafe_at(&i) };
        unsafe { self.subsequence.set_end(i) };
        r
    }

    /// Removes and returns a subsequence of elements starting from `self.start()` upto but not
    /// including `p`.
    fn pop_prefix_upto(&mut self, p: SubSequence::Position) -> Self {
        let s = unsafe {
            self.subsequence
                .unsafe_slice(self.subsequence.start(), p.clone())
        };
        unsafe { self.subsequence.set_start(p) };
        unsafe { Self::new(s) }
    }
}

/// Drop algorithms
impl<'a, SubSequence> Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence> + 'a,
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
        SubSequence: BidirectionalCollection,
    {
        _ = self.pop_last();
    }

    /// Removes subsequence of elements starting from `self.start()` upto but not
    /// including `p`.
    fn drop_prefix_upto(&mut self, p: SubSequence::Position) {
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
    pub fn drop_prefix_through(&mut self, p: SubSequence::Position) {
        _ = self.pop_prefix_through(p);
    }

    /// Removes the longest prefix of `self` whose elements satisfy `p`.
    ///
    /// # Complexity
    ///   - Atmost `self.count()` applications of `p`.
    pub fn drop_while<Predicate>(&mut self, p: Predicate)
    where
        Predicate: FnMut(&SubSequence::Element) -> bool,
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
        SubSequence: BidirectionalCollection,
    {
        _ = self.pop_end(n);
    }

    /// Removes subsequence of last elements from `self` starting from `p`.
    pub fn drop_suffix_from(&mut self, p: SubSequence::Position) {
        _ = self.pop_suffix_from(p);
    }
}

/// Pop algorithms
impl<'a, SubSequence> Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence> + 'a,
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
    pub fn pop_prefix_through(&mut self, p: SubSequence::Position) -> Self {
        self.pop_prefix_upto(self.next(p))
    }

    /// Removes and returns the longest prefix of `self` whose elements satisfy `p`.
    ///
    /// # Complexity
    ///   - Atmost `self.count()` applications of `p`.
    pub fn pop_while<Predicate>(&mut self, mut p: Predicate) -> Self
    where
        Predicate: FnMut(&SubSequence::Element) -> bool,
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
        SubSequence: BidirectionalCollection,
    {
        let mut f = self.end();
        self.form_prior_n_limited_by(&mut f, n, self.start());
        self.pop_suffix_from(f)
    }

    /// Removes and returns subsequence of last elements from `self` starting from `p`.
    pub fn pop_suffix_from(&mut self, p: SubSequence::Position) -> Self {
        let mut s = self.pop_prefix_upto(p);
        std::mem::swap(self, &mut s);
        s
    }
}

unsafe impl<'a, SubSequence> Send for Slice<'a, SubSequence> where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence> + 'a
{
}

impl<'a, SubSequence> Collection for Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence> + 'a,
{
    type Position = SubSequence::Position;

    type Element = SubSequence::Element;

    type ElementRef<'b>
        = SubSequence::ElementRef<'b>
    where
        Self: 'b;

    type SubSequence = SubSequence;

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

impl<'a, SubSequence> LazyCollection for Slice<'a, SubSequence>
where
    SubSequence:
        LazyCollection + UnsafeSubSequence<SubSequence = SubSequence> + 'a,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.subsequence.compute_at(i)
    }
}

impl<'a, SubSequence> BidirectionalCollection for Slice<'a, SubSequence>
where
    SubSequence: BidirectionalCollection
        + UnsafeSubSequence<SubSequence = SubSequence>
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

impl<'a, SubSequence> RandomAccessCollection for Slice<'a, SubSequence> where
    SubSequence: RandomAccessCollection
        + UnsafeSubSequence<SubSequence = SubSequence>
        + 'a
{
}
