// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalCollection, LazyCollection, Slice};

/// An iterator to iterate over lazily computed elements of collection.
pub struct LazyCollectionIter<'a, C>
where
    C: LazyCollection<Whole = C>,
{
    /// Slice representing remaining elements to iterate.
    slice: Slice<'a, C>,
}

impl<'a, C> LazyCollectionIter<'a, C>
where
    C: LazyCollection<Whole = C>,
{
    /// Creates a new instance of Self with given slice.
    pub fn new(slice: Slice<'a, C>) -> Self {
        Self { slice }
    }
}

impl<C> Iterator for LazyCollectionIter<'_, C>
where
    C: LazyCollection<Whole = C>,
{
    type Item = C::Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.lazy_pop_first()
    }
}

impl<C> DoubleEndedIterator for LazyCollectionIter<'_, C>
where
    C: BidirectionalCollection<Whole = C> + LazyCollection,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.slice.lazy_pop_last()
    }
}
