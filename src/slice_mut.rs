// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice,
};

#[derive(PartialEq, Eq)]
pub struct SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    whole: &'a mut Whole,
    from: Whole::Position,
    to: Whole::Position,
}

impl<'a, Whole> SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    pub fn new(
        collection: &'a mut Whole,
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
    pub fn pop_first(&mut self) -> Option<&<Self as Collection>::Element> {
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

impl<Whole> Collection for SliceMut<'_, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
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

    fn count(&self) -> usize {
        self.whole.count()
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

impl<Whole> BidirectionalCollection for SliceMut<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole> + ReorderableCollection,
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

impl<Whole> RandomAccessCollection for SliceMut<'_, Whole> where
    Whole: RandomAccessCollection<Whole = Whole> + ReorderableCollection
{
}

impl<Whole> ReorderableCollection for SliceMut<'_, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.whole.swap_at(i, j);
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<Self::Whole> {
        SliceMut::new(self.whole, from, to)
    }
}

impl<Whole> MutableCollection for SliceMut<'_, Whole>
where
    Whole: MutableCollection<Whole = Whole>,
{
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        self.whole.at_mut(i)
    }
}
