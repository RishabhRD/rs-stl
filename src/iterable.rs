// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, CollectionExt, Slice, SliceIterator};

/// Models an entity, that can be iterated upon without consumption.
pub trait Iterable {
    /// Type of item during iteration.
    type Item;

    /// Type of iterator for iteration which iterates `&Self::Item`.
    type Iter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    /// Returns `Self::Iter` for iterating over `self`.
    fn iter(&self) -> Self::Iter<'_>;
}

impl<T> Iterable for T
where
    T: Collection<Whole = T>,
{
    type Item = <Self as Collection>::Element;

    type Iter<'a>
        = SliceIterator<'a, T>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        SliceIterator::new(self.all())
    }
}

impl<T> Iterable for Slice<'_, T>
where
    T: Collection<Whole = T>,
{
    type Item = T::Element;

    type Iter<'a>
        = SliceIterator<'a, T>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        SliceIterator::new(self.all())
    }
}
