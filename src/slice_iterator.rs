// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, Slice};

pub struct SliceIterator<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    slice: Slice<'a, Whole>,
}

impl<'a, Whole> SliceIterator<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    pub fn new(slice: Slice<'a, Whole>) -> Self {
        SliceIterator { slice }
    }
}

impl<'a, Whole> Iterator for SliceIterator<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    type Item = &'a Whole::Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.pop_first()
    }
}
