// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, LazyCollection, LazyCollectionExt,
    RandomAccessCollection, Slice,
};

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
    pub(crate) fn new(slice: Slice<'a, C>) -> Self {
        Self { slice }
    }
}

impl<C> Iterator for LazyCollectionIter<'_, C>
where
    C: LazyCollection<Whole = C>,
{
    type Item = C::Element;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.slice.lazy_first();
        self.slice.drop_first();
        r
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.slice.underestimated_count(), None)
    }
}

impl<C> DoubleEndedIterator for LazyCollectionIter<'_, C>
where
    C: BidirectionalCollection<Whole = C> + LazyCollection,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let r = self.slice.lazy_last();
        self.slice.drop_last();
        r
    }
}

impl<'a, C> ExactSizeIterator for LazyCollectionIter<'a, C>
where
    C: RandomAccessCollection<Whole = C> + LazyCollection,
{
    fn len(&self) -> usize {
        self.slice.count()
    }
}
