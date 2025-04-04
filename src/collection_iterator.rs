// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, Slice};

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
    type Item = &'a Whole::Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.pop_first()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.slice.underestimated_count(), None)
    }
}
