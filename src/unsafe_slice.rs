// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, LazyCollection,
    RandomAccessCollection, Slice, UnsafeSubSequence,
};

pub struct UnsafeSlice<C: Collection> {
    /// The whole collection.
    whole: *const C,

    /// Start position of slice.
    start_position: C::Position,

    /// End position of slice.
    end_position: C::Position,
}

impl<C: Collection> UnsafeSlice<C> {
    /// Creates a new unsafe slice of `whole` collection starting from
    /// `start_position` and ending and `end_position`.
    pub unsafe fn new(
        whole: &C,
        start_position: C::Position,
        end_position: C::Position,
    ) -> Self {
        UnsafeSlice {
            whole,
            start_position,
            end_position,
        }
    }
}

impl<C: Collection> Collection for UnsafeSlice<C> {
    type Position = C::Position;

    type Element = C::Element;

    type ElementRef<'a>
        = C::ElementRef<'a>
    where
        Self: 'a;

    type SubSequence = Self;

    fn start(&self) -> Self::Position {
        self.start_position.clone()
    }

    fn end(&self) -> Self::Position {
        self.end_position.clone()
    }

    fn form_next(&self, p: &mut Self::Position) {
        unsafe { self.whole.as_ref_unchecked() }.form_next(p)
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        unsafe { self.whole.as_ref_unchecked() }.at(i)
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::Slice<'_, Self::SubSequence> {
        unsafe { Slice::new(self.unsafe_slice(from, to)) }
    }

    fn form_next_n(&self, p: &mut Self::Position, n: usize) {
        unsafe { self.whole.as_ref_unchecked() }.form_next_n(p, n)
    }

    fn form_next_n_limited_by(
        &self,
        p: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        unsafe { self.whole.as_ref_unchecked() }
            .form_next_n_limited_by(p, n, limit)
    }

    fn next(&self, p: Self::Position) -> Self::Position {
        unsafe { self.whole.as_ref_unchecked() }.next(p)
    }

    fn next_n(&self, p: Self::Position, n: usize) -> Self::Position {
        unsafe { self.whole.as_ref_unchecked() }.next_n(p, n)
    }

    fn next_n_limited_by(
        &self,
        p: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        unsafe { self.whole.as_ref_unchecked() }.next_n_limited_by(p, n, limit)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        unsafe { self.whole.as_ref_unchecked() }.distance(from, to)
    }

    fn count(&self) -> usize {
        unsafe { self.whole.as_ref_unchecked() }.count()
    }

    fn underestimated_count(&self) -> usize {
        unsafe { self.whole.as_ref_unchecked() }.underestimated_count()
    }
}

impl<C> LazyCollection for UnsafeSlice<C>
where
    C: LazyCollection,
    C::SubSequence: LazyCollection,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        unsafe { self.whole.as_ref_unchecked() }.compute_at(i)
    }
}

impl<C> BidirectionalCollection for UnsafeSlice<C>
where
    C: BidirectionalCollection,
    C::SubSequence: BidirectionalCollection,
{
    fn form_prior(&self, p: &mut Self::Position) {
        unsafe { self.whole.as_ref_unchecked() }.form_prior(p)
    }

    fn form_prior_n(&self, p: &mut Self::Position, n: usize) {
        unsafe { self.whole.as_ref_unchecked() }.form_prior_n(p, n)
    }

    fn form_prior_n_limited_by(
        &self,
        p: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        unsafe { self.whole.as_ref_unchecked() }
            .form_prior_n_limited_by(p, n, limit)
    }

    fn prior(&self, p: Self::Position) -> Self::Position {
        unsafe { self.whole.as_ref_unchecked() }.prior(p)
    }

    fn prior_n(&self, p: Self::Position, n: usize) -> Self::Position {
        unsafe { self.whole.as_ref_unchecked() }.prior_n(p, n)
    }

    fn prior_n_limited_by(
        &self,
        p: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        unsafe { self.whole.as_ref_unchecked() }.prior_n_limited_by(p, n, limit)
    }
}

impl<C> RandomAccessCollection for UnsafeSlice<C>
where
    C: RandomAccessCollection,
    C::SubSequence: RandomAccessCollection,
{
}

impl<C: Collection> UnsafeSubSequence for UnsafeSlice<C> {
    unsafe fn unsafe_at<'a>(&self, i: &Self::Position) -> Self::ElementRef<'a> {
        unsafe { self.whole.as_ref_unchecked() }.at(i)
    }

    unsafe fn unsafe_slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubSequence {
        UnsafeSlice::new(self.whole.as_ref_unchecked(), from, to)
    }

    unsafe fn set_start(&mut self, p: Self::Position) {
        self.start_position = p
    }

    unsafe fn set_end(&mut self, p: Self::Position) {
        self.end_position = p
    }
}
