// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, LazyCollection, Slice};

pub struct CollectionIterator<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    slice: Slice<'a, Whole>,
}

impl<'a, Whole> CollectionIterator<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    pub fn new(slice: Slice<'a, Whole>) -> Self {
        CollectionIterator { slice }
    }
}

impl<'a, Whole> Iterator for CollectionIterator<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    type Item = <Slice<'a, Whole> as Collection>::ElementRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.pop_first()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let cnt = self.slice.underestimated_count();
        (cnt, None)
    }
}

pub struct LazyCollectionIterator<'a, Whole>
where
    Whole: LazyCollection<Whole = Whole>,
{
    slice: Slice<'a, Whole>,
}

impl<'a, Whole> LazyCollectionIterator<'a, Whole>
where
    Whole: LazyCollection<Whole = Whole>,
{
    pub fn new(slice: Slice<'a, Whole>) -> Self {
        LazyCollectionIterator { slice }
    }
}

impl<'a, Whole> Iterator for LazyCollectionIterator<'a, Whole>
where
    Whole: LazyCollection<Whole = Whole>,
{
    type Item = <Slice<'a, Whole> as Collection>::Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.lazy_pop_first()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let cnt = self.slice.underestimated_count();
        (cnt, None)
    }
}
