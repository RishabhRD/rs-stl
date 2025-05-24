// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, LazyCollection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

/// Models an empty collection.
pub struct EmptyCollection<E> {
    phantom: std::marker::PhantomData<E>,
}

impl<E> EmptyCollection<E> {
    pub fn new() -> Self {
        EmptyCollection {
            phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for EmptyCollection<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Collection for EmptyCollection<E> {
    type Position = ();

    type Element = E;

    type ElementRef<'a>
        = &'a E
    where
        Self: 'a;

    type Whole = Self;

    fn start(&self) -> Self::Position {}

    fn end(&self) -> Self::Position {}

    fn form_next(&self, _: &mut Self::Position) {}

    fn at(&self, _: &Self::Position) -> Self::ElementRef<'_> {
        panic!("Out of bound access")
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::Slice<Self::Whole> {
        Slice::new(self, from, to)
    }

    fn form_next_n(&self, _: &mut Self::Position, _: usize) {}

    fn distance(&self, _: Self::Position, _: Self::Position) -> usize {
        0
    }

    fn count(&self) -> usize {
        0
    }

    fn underestimated_count(&self) -> usize {
        0
    }
}

impl<E> BidirectionalCollection for EmptyCollection<E> {
    fn form_prior(&self, _: &mut Self::Position) {}

    fn form_prior_n(&self, _: &mut Self::Position, _: usize) {}
}

impl<E> RandomAccessCollection for EmptyCollection<E> {}

impl<E> ReorderableCollection for EmptyCollection<E> {
    fn swap_at(&mut self, _: &Self::Position, _: &Self::Position) {
        panic!("Out of bound access")
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::SliceMut<Self::Whole> {
        SliceMut::new(self, from, to)
    }
}

impl<E> MutableCollection for EmptyCollection<E> {
    fn at_mut(&mut self, _: &Self::Position) -> &mut Self::Element {
        panic!("Out of bound access")
    }
}

impl<E> LazyCollection for EmptyCollection<E> {
    fn compute_at(&self, _: &Self::Position) -> Self::Element {
        panic!("Out of bound access")
    }
}
