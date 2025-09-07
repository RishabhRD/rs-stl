// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalCollection, Collection, Slice};

use ::lender::prelude::*;

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
    pub fn new(slice: Slice<'a, C>) -> Self {
        Self { slice }
    }
}

impl<'a, C> Lending<'a> for CollectionIter<'_, C>
where
    C: Collection<Whole = C>,
{
    type Lend = C::ElementRef<'a>;
}

impl<C> Lender for CollectionIter<'_, C>
where
    C: Collection<Whole = C>,
{
    fn next(&mut self) -> Option<Lend<'_, Self>> {
        self.slice.pop_first()
    }
}

impl<C> DoubleEndedLender for CollectionIter<'_, C>
where
    C: BidirectionalCollection<Whole = C>,
{
    fn next_back(&mut self) -> Option<Lend<'_, Self>> {
        self.slice.pop_last()
    }
}
