// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    Collection, CollectionExt, ReorderableCollection, Slice, SliceMut,
};

/// An iterator that splits the collection evenly into number of given slices.
pub struct SplitEvenlyIterator<'a, C>
where
    C: Collection<Whole = C>,
{
    /// Number of slices.
    num_slices: usize,

    /// Maximum size of a split slice.
    slice_size: usize,

    /// Remaining slice.
    rest: Slice<'a, C::Whole>,
}

impl<'a, C> SplitEvenlyIterator<'a, C>
where
    C: Collection<Whole = C>,
{
    /// Creates a new instance of `SplitEvenlyIterator`.
    pub(crate) fn new(
        slice: Slice<'a, C::Whole>,
        split_size: usize,
        num_splits: usize,
    ) -> Self {
        SplitEvenlyIterator {
            num_slices: num_splits,
            slice_size: split_size,
            rest: slice,
        }
    }
}

impl<'a, C> Iterator for SplitEvenlyIterator<'a, C>
where
    C: Collection<Whole = C>,
{
    type Item = Slice<'a, C::Whole>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }
        let mut next_pos = self.rest.start();
        self.rest.form_next_n_limited_by(
            &mut next_pos,
            self.slice_size,
            self.rest.end(),
        );

        Some(self.rest.pop_prefix_upto(next_pos))
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

/// An iterator that splits the collection evenly into number of given mutable slices.
pub struct SplitEvenlyIteratorMut<'a, C>
where
    C: ReorderableCollection<Whole = C>,
{
    /// Number of slices.
    num_slices: usize,

    /// Maximum size of a split slice.
    slice_size: usize,

    /// Remaining slice.
    rest: SliceMut<'a, C::Whole>,
}

impl<'a, C> SplitEvenlyIteratorMut<'a, C>
where
    C: ReorderableCollection<Whole = C>,
{
    /// Creates a new instance of `SplitEvenlyIterator`.
    pub(crate) fn new(
        slice: SliceMut<'a, C::Whole>,
        split_size: usize,
        num_splits: usize,
    ) -> Self {
        SplitEvenlyIteratorMut {
            num_slices: num_splits,
            slice_size: split_size,
            rest: slice,
        }
    }
}

impl<'a, C> Iterator for SplitEvenlyIteratorMut<'a, C>
where
    C: ReorderableCollection<Whole = C>,
{
    type Item = SliceMut<'a, C::Whole>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }
        let mut next_pos = self.rest.start();
        self.rest.form_next_n_limited_by(
            &mut next_pos,
            self.slice_size,
            self.rest.end(),
        );

        Some(self.rest.pop_prefix_upto(next_pos))
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
