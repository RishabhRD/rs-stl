// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, CollectionExt, LazyCollection,
    MutableCollection, RandomAccessCollection, ReorderableCollection,
    ReorderableCollectionExt, Slice, SliceMut, UnsafeMutableSubSequence,
    UnsafeReorderableSubSequence, UnsafeSubSequence,
};

/// A collection that presents element in reverse order of base collection.
pub struct ReversedCollection<C>
where
    C: BidirectionalCollection,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence: BidirectionalCollection,
{
    /// The base collection.
    pub base: C,
}

/// Position type of ReversedCollection.
///
/// Ordering of positions should be inverted in ReversedCollection.
#[derive(PartialEq, Eq, Clone)]
pub struct ReversedCollectionPosition<P> {
    pub base_position: P,
}

impl<P> PartialOrd for ReversedCollectionPosition<P>
where
    P: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.base_position.partial_cmp(&other.base_position) {
            Some(std::cmp::Ordering::Less) => Some(std::cmp::Ordering::Greater),
            Some(std::cmp::Ordering::Equal) => Some(std::cmp::Ordering::Equal),
            Some(std::cmp::Ordering::Greater) => Some(std::cmp::Ordering::Less),
            None => None,
        }
    }
}

impl<P> Ord for ReversedCollectionPosition<P>
where
    P: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.base_position.cmp(&other.base_position) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        }
    }
}

impl<C> ReversedCollection<C>
where
    C: BidirectionalCollection,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence: BidirectionalCollection,
{
    /// Returns a new instance of ReversedCollection created from given base collection.
    pub fn new(base: C) -> Self {
        ReversedCollection { base }
    }

    /// Returns the reversed collection of self i.e., base collection.
    pub fn reversed(self) -> C {
        self.base
    }
}

impl<C> Collection for ReversedCollection<C>
where
    C: BidirectionalCollection,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence: BidirectionalCollection,
{
    type Position = ReversedCollectionPosition<C::Position>;

    type Element = C::Element;

    type ElementRef<'a>
        = C::ElementRef<'a>
    where
        Self: 'a;

    type SubSequence = ReversedCollection<C::SubSequence>;

    type MutableSubSequence = ReversedCollection<C::MutableSubSequence>;

    fn start(&self) -> Self::Position {
        ReversedCollectionPosition {
            base_position: self.base.end(),
        }
    }

    fn end(&self) -> Self::Position {
        ReversedCollectionPosition {
            base_position: self.base.start(),
        }
    }

    fn form_next(&self, position: &mut Self::Position) {
        self.base.form_prior(&mut position.base_position)
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        self.base.at(&self.base.prior(i.base_position.clone()))
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::SubSequence> {
        unsafe {
            let s = self.full().subsequence();
            Slice::new(s.unsafe_slice(from, to))
        }
    }

    fn form_next_n(&self, position: &mut Self::Position, n: usize) {
        self.base.form_prior_n(&mut position.base_position, n)
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.base.form_prior_n_limited_by(
            &mut position.base_position,
            n,
            limit.base_position,
        )
    }

    fn next(&self, position: Self::Position) -> Self::Position {
        ReversedCollectionPosition {
            base_position: self.base.prior(position.base_position),
        }
    }

    fn next_n(&self, position: Self::Position, n: usize) -> Self::Position {
        ReversedCollectionPosition {
            base_position: self.base.prior_n(position.base_position, n),
        }
    }

    fn next_n_limited_by(
        &self,
        position: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        self.base
            .prior_n_limited_by(position.base_position, n, limit.base_position)
            .map(|e| ReversedCollectionPosition { base_position: e })
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.base.distance(to.base_position, from.base_position)
    }

    fn count(&self) -> usize {
        self.base.count()
    }

    fn underestimated_count(&self) -> usize {
        self.base.underestimated_count()
    }
}

impl<C> BidirectionalCollection for ReversedCollection<C>
where
    C: BidirectionalCollection,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence: BidirectionalCollection,
{
    fn form_prior(&self, position: &mut Self::Position) {
        self.base.form_next(&mut position.base_position)
    }

    fn form_prior_n(&self, position: &mut Self::Position, n: usize) {
        self.base.form_next_n(&mut position.base_position, n)
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.base.form_next_n_limited_by(
            &mut position.base_position,
            n,
            limit.base_position,
        )
    }

    fn prior(&self, position: Self::Position) -> Self::Position {
        ReversedCollectionPosition {
            base_position: self.base.next(position.base_position),
        }
    }

    fn prior_n(&self, position: Self::Position, n: usize) -> Self::Position {
        ReversedCollectionPosition {
            base_position: self.base.next_n(position.base_position, n),
        }
    }

    fn prior_n_limited_by(
        &self,
        position: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        self.base
            .next_n_limited_by(position.base_position, n, limit.base_position)
            .map(|e| ReversedCollectionPosition { base_position: e })
    }
}

impl<C> RandomAccessCollection for ReversedCollection<C>
where
    C: RandomAccessCollection,
    C::SubSequence: RandomAccessCollection,
    C::MutableSubSequence: RandomAccessCollection,
{
}

impl<C> LazyCollection for ReversedCollection<C>
where
    C: LazyCollection + BidirectionalCollection,
    C::SubSequence: LazyCollection + BidirectionalCollection,
    C::MutableSubSequence: LazyCollection + BidirectionalCollection,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.base
            .compute_at(&self.base.prior(i.base_position.clone()))
    }
}

impl<C> ReorderableCollection for ReversedCollection<C>
where
    C: ReorderableCollection + BidirectionalCollection,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence:
        UnsafeReorderableSubSequence + BidirectionalCollection,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.base.swap_at(
            &self.base.prior(i.base_position.clone()),
            &self.base.prior(j.base_position.clone()),
        )
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::MutableSubSequence> {
        unsafe {
            let s = self.full_mut().subsequence();
            SliceMut::new(s.unsafe_slice_mut(from, to))
        }
    }
}

impl<C> MutableCollection for ReversedCollection<C>
where
    C: MutableCollection + BidirectionalCollection,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence: UnsafeMutableSubSequence + BidirectionalCollection,
{
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        self.base.at_mut(&self.base.prior(i.base_position.clone()))
    }
}

impl<C> UnsafeSubSequence for ReversedCollection<C>
where
    C: BidirectionalCollection + UnsafeSubSequence,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence: BidirectionalCollection,
{
    unsafe fn unsafe_at<'a>(&self, i: &Self::Position) -> Self::ElementRef<'a> {
        self.base
            .unsafe_at(&self.base.prior(i.base_position.clone()))
    }

    unsafe fn unsafe_slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubSequence {
        ReversedCollection {
            base: self.base.unsafe_slice(to.base_position, from.base_position),
        }
    }

    unsafe fn set_start(&mut self, p: Self::Position) {
        self.base.set_end(p.base_position)
    }

    unsafe fn set_end(&mut self, p: Self::Position) {
        self.base.set_start(p.base_position)
    }
}

impl<C> UnsafeReorderableSubSequence for ReversedCollection<C>
where
    C: BidirectionalCollection + UnsafeReorderableSubSequence,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence:
        UnsafeReorderableSubSequence + BidirectionalCollection,
{
    unsafe fn unsafe_slice_mut(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::MutableSubSequence {
        ReversedCollection {
            base: self
                .base
                .unsafe_slice_mut(to.base_position, from.base_position),
        }
    }
}

impl<C> UnsafeMutableSubSequence for ReversedCollection<C>
where
    C: BidirectionalCollection + UnsafeMutableSubSequence,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence: UnsafeMutableSubSequence + BidirectionalCollection,
{
    unsafe fn unsafe_at_mut<'a>(
        &self,
        i: &Self::Position,
    ) -> &'a mut Self::Element {
        self.base
            .unsafe_at_mut(&self.base.prior(i.base_position.clone()))
    }
}
