// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, CollectionExt, MutableCollection,
    RandomAccessCollection, SliceMut, UnsafeMutableSubSequence,
};

/// An iterator to iterate over mutable reference of elements of collection.
pub struct MutableCollectionIter<'a, C>
where
    C: MutableCollection + 'a,
    C::MutableSubSequence: UnsafeMutableSubSequence,
{
    /// Slice representing remaining elements to iterate.
    slice: SliceMut<'a, C::MutableSubSequence>,
}

impl<'a, C> MutableCollectionIter<'a, C>
where
    C: MutableCollection + 'a,
    C::MutableSubSequence: UnsafeMutableSubSequence,
{
    /// Creates a new instance of Self with given slice.
    pub(crate) fn new(slice: SliceMut<'a, C::MutableSubSequence>) -> Self {
        Self { slice }
    }
}

impl<'a, C> Iterator for MutableCollectionIter<'a, C>
where
    C: MutableCollection + 'a,
    C::MutableSubSequence: UnsafeMutableSubSequence,
{
    type Item = &'a mut <C::MutableSubSequence as Collection>::Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            Some(self.slice.pop_first_mut())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.slice.underestimated_count(), None)
    }
}

impl<'a, C> DoubleEndedIterator for MutableCollectionIter<'a, C>
where
    C: BidirectionalCollection + MutableCollection + 'a,
    C::SubSequence: BidirectionalCollection,
    C::MutableSubSequence: BidirectionalCollection + UnsafeMutableSubSequence,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            Some(self.slice.pop_last_mut())
        }
    }
}

impl<'a, C> ExactSizeIterator for MutableCollectionIter<'a, C>
where
    C: RandomAccessCollection + MutableCollection + 'a,
    C::SubSequence: RandomAccessCollection,
    C::MutableSubSequence: RandomAccessCollection + UnsafeMutableSubSequence,
{
    fn len(&self) -> usize {
        self.slice.count()
    }
}
