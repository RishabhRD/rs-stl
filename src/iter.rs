// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, LazyCollection};

/// Iterator to iterate over reference of elements of collection.
pub struct CollectionIterator<'a, R>
where
    R: Collection + ?Sized,
{
    range: &'a R,
    pos: R::Position,
}

impl<'a, Range> CollectionIterator<'a, Range>
where
    Range: Collection + ?Sized,
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
{
    range: &'a R,
    pos: R::Position,
}

impl<'a, Range> LazyCollectionIterator<'a, Range>
where
    Range: LazyCollection + ?Sized,
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
