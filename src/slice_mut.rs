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

impl<Whole> BidirectionalCollection for SliceMut<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole> + ReorderableCollection,
{
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
