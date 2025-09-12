// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalCollection, Collection, Slice};

/// An iterator to iterate over element-ref of collection.
pub struct CollectionIter<'a, C>
where
    C: Collection<Whole = C>,
{
    /// Slice representing remaining elements to iterate.
    slice: Slice<'a, C>,
}

impl<'a, C> CollectionIter<'a, C>
where
    C: Collection<Whole = C>,
{
    /// Creates a new instance of Self with given slice.
    pub(crate) fn new(slice: Slice<'a, C>) -> Self {
        Self { slice }
    }
}

impl<'a, C> Iterator for CollectionIter<'a, C>
where
    C: Collection<Whole = C>,
{
    type Item = C::ElementRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.pop_first()
    }
}

impl<'a, C> DoubleEndedIterator for CollectionIter<'a, C>
where
    C: BidirectionalCollection<Whole = C>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.slice.pop_last()
    }
}
