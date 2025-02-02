// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    util::ValueRef, BidirectionalRange, Collection, LazyCollection,
    MutableRange, RandomAccessRange, Range, ReorderableRange,
};

pub struct MapView<R, Operation, OutputElement>
where
    R: Collection,
    Operation: Fn(&R::Element) -> OutputElement,
{
    range: R,
    f: Operation,
}

impl<R, Operation, OutputElement> MapView<R, Operation, OutputElement>
where
    R: Collection,
    Operation: Fn(&R::Element) -> OutputElement,
{
    pub fn new(range: R, f: Operation) -> Self {
        MapView { range, f }
    }
}

impl<R, Operation, OutputElement> Range for MapView<R, Operation, OutputElement>
where
    R: Collection,
    Operation: Fn(&R::Element) -> OutputElement,
{
    type Position = R::Position;

    type Element = OutputElement;

    fn start(&self) -> Self::Position {
        self.range.start()
    }

    fn end(&self) -> Self::Position {
        self.range.end()
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        self.range.after(i)
    }

    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        ValueRef {
            val: (self.f)(self.range.at(i)),
        }
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.range.after_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.range.distance(from, to)
    }
}

impl<R, Operation, OutputElement> LazyCollection
    for MapView<R, Operation, OutputElement>
where
    R: Collection,
    Operation: Fn(&R::Element) -> OutputElement,
{
    fn at(&self, i: &Self::Position) -> Self::Element {
        (self.f)(self.range.at(i))
    }
}

impl<R, Operation, OutputElement> BidirectionalRange
    for MapView<R, Operation, OutputElement>
where
    R: Collection + BidirectionalRange,
    Operation: Fn(&R::Element) -> OutputElement,
{
    fn before(&self, i: Self::Position) -> Self::Position {
        self.range.before(i)
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.range.before_n(i, n)
    }
}

impl<R, Operation, OutputElement> RandomAccessRange
    for MapView<R, Operation, OutputElement>
where
    R: Collection + RandomAccessRange,
    Operation: Fn(&R::Element) -> OutputElement,
{
}

impl<R, Operation, OutputElement> MutableRange
    for MapView<R, Operation, OutputElement>
where
    R: Collection + MutableRange,
    Operation: Fn(&R::Element) -> OutputElement,
{
}

impl<R, Operation, OutputElement> ReorderableRange
    for MapView<R, Operation, OutputElement>
where
    R: Collection + ReorderableRange,
    Operation: Fn(&R::Element) -> OutputElement,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.range.swap_at(i, j);
    }
}
