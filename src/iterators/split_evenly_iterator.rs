// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    Collection, CollectionExt, ReorderableCollection, Slice, SliceMut,
};

/// An iterator yielding evenly sized slices of collection.
pub struct SplitEvenlyIterator<'a, C>
where
    C: Collection<Whole = C>,
{
    /// Remaining elements.
    rest: Slice<'a, C::Whole>,

    /// Number of slices.
    num_slices: usize,

    /// Size of a slice.
    slice_size: usize,

    /// Number of slices which would have 1 more elements than other.
    num_bigger_slices: usize,
}

impl<'a, C> SplitEvenlyIterator<'a, C>
where
    C: Collection<Whole = C>,
{
    /// Creates instance of SplitEvenlyIterator.
    pub(crate) fn new(
        slice: Slice<'a, C::Whole>,
        num_slices: usize,
        slice_size: usize,
        num_bigger_slices: usize,
    ) -> Self {
        Self {
            rest: slice,
            num_slices,
            slice_size,
            num_bigger_slices,
        }
    }
}

impl<'a, C> Iterator for SplitEvenlyIterator<'a, C>
where
    C: Collection<Whole = C>,
{
    type Item = Slice<'a, C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let mut size = self.slice_size;
        if self.num_bigger_slices > 0 {
            size += 1;
            self.num_bigger_slices -= 1;
        }

        Some(self.rest.pop_prefix(size))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num_slices, Some(self.num_slices))
    }
}

impl<'a, C> ExactSizeIterator for SplitEvenlyIterator<'a, C>
where
    C: Collection<Whole = C>,
{
    fn len(&self) -> usize {
        self.num_slices
    }
}

/// An iterator yielding evenly sized mutable slices of collection.
pub struct SplitEvenlyIteratorMut<'a, C>
where
    C: ReorderableCollection<Whole = C>,
{
    /// Remaining elements.
    rest: SliceMut<'a, C::Whole>,

    /// Number of slices.
    num_slices: usize,

    /// Size of a slice.
    slice_size: usize,

    /// Number of slices which would have 1 more elements than other.
    num_bigger_slices: usize,
}

impl<'a, C> SplitEvenlyIteratorMut<'a, C>
where
    C: ReorderableCollection<Whole = C>,
{
    /// Creates instance of SplitEvenlyIteratorMut.
    pub(crate) fn new(
        slice: SliceMut<'a, C::Whole>,
        num_slices: usize,
        slice_size: usize,
        num_bigger_slices: usize,
    ) -> Self {
        Self {
            rest: slice,
            num_slices,
            slice_size,
            num_bigger_slices,
        }
    }
}

impl<'a, C> Iterator for SplitEvenlyIteratorMut<'a, C>
where
    C: ReorderableCollection<Whole = C>,
{
    type Item = SliceMut<'a, C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let mut size = self.slice_size;
        if self.num_bigger_slices > 0 {
            size += 1;
            self.num_bigger_slices -= 1;
        }

        Some(self.rest.pop_prefix(size))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num_slices, Some(self.num_slices))
    }
}

impl<'a, C> ExactSizeIterator for SplitEvenlyIteratorMut<'a, C>
where
    C: ReorderableCollection<Whole = C>,
{
    fn len(&self) -> usize {
        self.num_slices
    }
}
