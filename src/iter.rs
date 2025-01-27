// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, LazyCollection, SubRangeable};

/// Iterator to iterate over reference of elements of collection.
pub struct CollectionIterator<'a, R>
where
    R: Collection + ?Sized,
    for<'this> <R as SubRangeable<'this>>::SubRange: Collection,
{
    range: &'a R,
    pos: R::Position,
}

impl<'a, Range> CollectionIterator<'a, Range>
where
    Range: Collection + ?Sized,
    for<'this> <Range as SubRangeable<'this>>::SubRange: Collection,
{
    pub fn new(range: &'a Range) -> Self {
        Self {
            range,
            pos: range.start(),
        }
    }
}

impl<'a, R> Iterator for CollectionIterator<'a, R>
where
    R: Collection + ?Sized,
    for<'this> <R as SubRangeable<'this>>::SubRange: Collection,
{
    type Item = &'a R::Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.range.end() {
            None
        } else {
            let element = self.range.at(&self.pos);
            self.pos = self.range.after(self.pos.clone());
            Some(element)
        }
    }
}

/// Iterator to iterate over values of lazy collection.
pub struct LazyCollectionIterator<'a, R>
where
    R: LazyCollection + ?Sized,
    for<'this> <R as SubRangeable<'this>>::SubRange: LazyCollection,
{
    range: &'a R,
    pos: R::Position,
}

impl<'a, Range> LazyCollectionIterator<'a, Range>
where
    Range: LazyCollection + ?Sized,
    for<'this> <Range as SubRangeable<'this>>::SubRange: LazyCollection,
{
    pub fn new(range: &'a Range) -> Self {
        Self {
            range,
            pos: range.start(),
        }
    }
}

impl<R> Iterator for LazyCollectionIterator<'_, R>
where
    R: LazyCollection + ?Sized,
    for<'this> <R as SubRangeable<'this>>::SubRange: LazyCollection,
{
    type Item = R::Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.range.end() {
            None
        } else {
            let element = self.range.at(&self.pos);
            self.pos = self.range.after(self.pos.clone());
            Some(element)
        }
    }
}
