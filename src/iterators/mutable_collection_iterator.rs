// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalCollection, MutableCollection, SliceMut};

/// An iterator to iterate over mutable reference of elements of collection.
pub struct MutableCollectionIter<'a, C>
where
    C: MutableCollection<Whole = C>,
{
    /// Slice representing remaining elements to iterate.
    slice: SliceMut<'a, C>,
}

impl<'a, C> MutableCollectionIter<'a, C>
where
    C: MutableCollection<Whole = C>,
{
    /// Creates a new instance of Self with given slice.
    pub(crate) fn new(slice: SliceMut<'a, C>) -> Self {
        Self { slice }
    }
}

impl<'a, C> Iterator for MutableCollectionIter<'a, C>
where
    C: MutableCollection<Whole = C>,
{
    type Item = &'a mut C::Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.pop_first_mut()
    }
}

impl<'a, C> DoubleEndedIterator for MutableCollectionIter<'a, C>
where
    C: BidirectionalCollection<Whole = C> + MutableCollection,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.slice.pop_last_mut()
    }
}
