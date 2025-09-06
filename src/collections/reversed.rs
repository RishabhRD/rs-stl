// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, LazyCollection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

/// A collection that presents element in reverse order of base collection.
pub struct ReversedCollection<C>
where
    C: BidirectionalCollection,
    C::Whole: BidirectionalCollection,
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
    C::Whole: BidirectionalCollection,
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
    C::Whole: BidirectionalCollection,
{
    type Position = ReversedCollectionPosition<C::Position>;

    type Element = C::Element;

    type ElementRef<'a>
        = C::ElementRef<'a>
    where
        Self: 'a;

    type Whole = Self;

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
    ) -> Slice<'_, Self::Whole> {
        Slice::new(self, from, to)
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
    C::Whole: BidirectionalCollection,
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
    C::Whole: RandomAccessCollection,
{
}

impl<C> LazyCollection for ReversedCollection<C>
where
    C: LazyCollection + BidirectionalCollection,
    C::Whole: LazyCollection + BidirectionalCollection,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.base
            .compute_at(&self.base.prior(i.base_position.clone()))
    }
}

impl<C> ReorderableCollection for ReversedCollection<C>
where
    C: ReorderableCollection + BidirectionalCollection,
    C::Whole: ReorderableCollection + BidirectionalCollection,
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
    ) -> SliceMut<'_, Self::Whole> {
        SliceMut::new(self, from, to)
    }
}

impl<C> MutableCollection for ReversedCollection<C>
where
    C: MutableCollection + BidirectionalCollection,
    C::Whole: MutableCollection + BidirectionalCollection,
{
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        self.base.at_mut(&self.base.prior(i.base_position.clone()))
    }
}
