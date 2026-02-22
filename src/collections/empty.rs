// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, LazyCollection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
    UnsafeMutableSubSequence, UnsafeSubSequence,
};

/// An empty collection.
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

    type SubSequence = Self;

    fn start(&self) -> Self::Position {}

    fn end(&self) -> Self::Position {}

    fn form_next(&self, _: &mut Self::Position) {}

    fn form_next_n(&self, _: &mut Self::Position, _: usize) {}

    fn form_next_n_limited_by(
        &self,
        _: &mut Self::Position,
        n: usize,
        _: Self::Position,
    ) -> bool {
        n == 0
    }

    fn distance(&self, _: Self::Position, _: Self::Position) -> usize {
        0
    }

    fn count(&self) -> usize {
        0
    }

    fn underestimated_count(&self) -> usize {
        0
    }

    fn at(&self, _: &Self::Position) -> Self::ElementRef<'_> {
        panic!("Out of bound access")
    }

    fn slice(
        &self,
        _: Self::Position,
        _: Self::Position,
    ) -> crate::Slice<'_, Self::SubSequence> {
        unsafe { Slice::new(Self::default()) }
    }
}

impl<E> BidirectionalCollection for EmptyCollection<E> {
    fn form_prior(&self, _: &mut Self::Position) {}

    fn form_prior_n(&self, _: &mut Self::Position, _: usize) {}

    fn form_prior_n_limited_by(
        &self,
        _: &mut Self::Position,
        n: usize,
        _: Self::Position,
    ) -> bool {
        n == 0
    }
}

impl<E> RandomAccessCollection for EmptyCollection<E> {}

impl<E> ReorderableCollection for EmptyCollection<E> {
    fn swap_at(&mut self, _: &Self::Position, _: &Self::Position) {
        panic!("Out of bound access")
    }

    fn slice_mut(
        &mut self,
        _: Self::Position,
        _: Self::Position,
    ) -> crate::SliceMut<'_, Self::SubSequence> {
        unsafe { SliceMut::new(Self::default()) }
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

impl<E> UnsafeSubSequence for EmptyCollection<E> {
    unsafe fn unsafe_at<'a>(&self, _: &Self::Position) -> Self::ElementRef<'a> {
        panic!("Out of bound access")
    }

    unsafe fn unsafe_slice(
        &self,
        _: Self::Position,
        _: Self::Position,
    ) -> Self::SubSequence {
        Self::default()
    }

    unsafe fn set_start(&mut self, _: Self::Position) {}

    unsafe fn set_end(&mut self, _: Self::Position) {}
}

impl<E> UnsafeMutableSubSequence for EmptyCollection<E> {
    unsafe fn unsafe_at_mut<'a>(
        &self,
        _: &Self::Position,
    ) -> &'a mut Self::Element {
        panic!("Out of bound access")
    }
}
