// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

impl<T, const N: usize> Collection for [T; N] {
    type Position = usize;

    type Element = T;

    type ElementRef<'a>
        = &'a T
    where
        Self: 'a;

    type Whole = Self;

    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        N
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self[*i]
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<Self::Whole> {
        Slice::new(self, from, to)
    }

    fn form_next(&self, i: &mut Self::Position) {
        *i += 1
    }

    fn form_next_n(&self, i: &mut Self::Position, n: usize) {
        *i += n
    }
}

impl<T, const N: usize> BidirectionalCollection for [T; N] {
    fn form_prior(&self, i: &mut Self::Position) {
        *i -= 1
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        *i -= n
    }
}

impl<T, const N: usize> RandomAccessCollection for [T; N] {}

impl<T, const N: usize> ReorderableCollection for [T; N] {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j)
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::SliceMut<Self::Whole> {
        SliceMut::new(self, from, to)
    }
}

impl<T, const N: usize> MutableCollection for [T; N] {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }
}
