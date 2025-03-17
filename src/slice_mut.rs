// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice,
};

#[derive(PartialEq, Eq)]
pub struct SliceMut<'a, Core>
where
    Core: ReorderableCollection<SliceCore = Core>,
{
    core: &'a mut Core,
    from: Core::Position,
    to: Core::Position,
}

impl<'a, Core> SliceMut<'a, Core>
where
    Core: ReorderableCollection<SliceCore = Core>,
{
    pub fn new(
        collection: &'a mut Core,
        from: Core::Position,
        to: Core::Position,
    ) -> Self {
        Self {
            core: collection,
            from,
            to,
        }
    }
}

impl<Core> Collection for SliceMut<'_, Core>
where
    Core: ReorderableCollection<SliceCore = Core>,
{
    type Position = Core::Position;

    type Element = Core::Element;

    type SliceCore = Core;

    fn start(&self) -> Self::Position {
        self.from.clone()
    }

    fn end(&self) -> Self::Position {
        self.to.clone()
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        self.core.after(i)
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.core.after_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.core.distance(from, to)
    }

    fn at(&self, i: &Self::Position) -> &Self::Element {
        self.core.at(i)
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<Self::SliceCore> {
        Slice::new(self.core, from, to)
    }
}

impl<Core> BidirectionalCollection for SliceMut<'_, Core>
where
    Core: BidirectionalCollection<SliceCore = Core> + ReorderableCollection,
{
    fn before(&self, i: Self::Position) -> Self::Position {
        self.core.before(i)
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.core.before_n(i, n)
    }
}

impl<Core> RandomAccessCollection for SliceMut<'_, Core> where
    Core: RandomAccessCollection<SliceCore = Core> + ReorderableCollection
{
}

impl<Core> ReorderableCollection for SliceMut<'_, Core>
where
    Core: ReorderableCollection<SliceCore = Core>,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.core.swap_at(i, j);
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<Self::SliceCore> {
        SliceMut::new(self.core, from, to)
    }
}

impl<Core> MutableCollection for SliceMut<'_, Core>
where
    Core: MutableCollection<SliceCore = Core>,
{
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        self.core.at_mut(i)
    }
}
