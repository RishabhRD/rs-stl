// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, LazyCollection, RandomAccessCollection,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    whole: &'a Whole,
    from: Whole::Position,
    to: Whole::Position,
}

impl<'a, Whole> Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    pub fn new(
        collection: &'a Whole,
        from: Whole::Position,
        to: Whole::Position,
    ) -> Self {
        Self {
            whole: collection,
            from,
            to,
        }
    }

    /// Removes and returns the first element if non-empty; returns
    /// None otherwise.
    pub fn pop_first(
        &mut self,
    ) -> Option<<Self as Collection>::ElementRef<'a>> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self.whole.at(&self.from));
            self.whole.form_next(&mut self.from);
            e
        }
    }

    /// Removes the first element if non-empty and returns true; returns
    /// false otherwise.
    pub fn drop_first(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            self.whole.form_next(&mut self.from);
            true
        }
    }
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
        self.whole.form_next(i);
    }

    fn form_next_n(&self, i: &mut Self::Position, n: usize) {
        self.whole.form_next_n(i, n);
    }

    fn next(&self, i: Self::Position) -> Self::Position {
        self.whole.next(i)
    }

    fn next_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.whole.next_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.whole.distance(from, to)
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        self.whole.at(i)
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<Self::Whole> {
        Slice::new(self.whole, from, to)
    }
}

impl<Whole> LazyCollection for Slice<'_, Whole>
where
    Whole: LazyCollection<Whole = Whole>,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.whole.compute_at(i)
    }
}

impl<Whole> BidirectionalCollection for Slice<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole>,
{
    fn form_prior(&self, i: &mut Self::Position) {
        self.whole.form_prior(i);
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        self.whole.form_prior_n(i, n);
    }

    fn prior(&self, i: Self::Position) -> Self::Position {
        self.whole.prior(i)
    }

    fn prior_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.whole.prior_n(i, n)
    }
}

impl<Whole> RandomAccessCollection for Slice<'_, Whole> where
    Whole: RandomAccessCollection<Whole = Whole>
{
}
