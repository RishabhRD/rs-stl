// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalCollection, Collection, RandomAccessCollection};

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
    pub fn pop_first(&mut self) -> Option<&'a <Self as Collection>::Element> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self.whole.at(&self.from));
            self.whole.advance(&mut self.from);
            e
        }
    }

    /// Removes the first element if non-empty and returns true; returns
    /// false otherwise.
    pub fn drop_first(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            self.whole.advance(&mut self.from);
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

    type Whole = Whole;

    fn start(&self) -> Self::Position {
        self.from.clone()
    }

    fn end(&self) -> Self::Position {
        self.to.clone()
    }

    fn advance(&self, i: &mut Self::Position) {
        self.whole.advance(i);
    }

    fn advance_n(&self, i: &mut Self::Position, n: usize) {
        self.whole.advance_n(i, n);
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        self.whole.after(i)
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.whole.after_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.whole.distance(from, to)
    }

    fn at(&self, i: &Self::Position) -> &Self::Element {
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

impl<Whole> BidirectionalCollection for Slice<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole>,
{
    fn backstep(&self, i: &mut Self::Position) {
        self.whole.backstep(i);
    }

    fn backstep_n(&self, i: &mut Self::Position, n: usize) {
        self.whole.backstep_n(i, n);
    }

    fn before(&self, i: Self::Position) -> Self::Position {
        self.whole.before(i)
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.whole.before_n(i, n)
    }
}

impl<Whole> RandomAccessCollection for Slice<'_, Whole> where
    Whole: RandomAccessCollection<Whole = Whole>
{
}
