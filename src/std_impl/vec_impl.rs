// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

impl<T> Collection for Vec<T> {
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
        self.len()
    }

    fn form_next(&self, i: &mut Self::Position) {
        *i += 1
    }

    fn form_next_n(&self, i: &mut Self::Position, n: usize) {
        *i += n
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        if *position + n <= limit {
            *position += n;
            true
        } else {
            *position = limit;
            false
        }
    }

    fn next(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn next_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
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
    ) -> Slice<'_, Self::Whole> {
        Slice::new(self, from, to)
    }
}

impl<T> BidirectionalCollection for Vec<T> {
    fn form_prior(&self, i: &mut Self::Position) {
        *i -= 1
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        *i -= n
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        if *position - n >= limit {
            *position -= n;
            true
        } else {
            *position = limit;
            false
        }
    }
}

impl<T> RandomAccessCollection for Vec<T> {}

impl<T> ReorderableCollection for Vec<T> {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j)
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::SliceMut<'_, Self::Whole> {
        SliceMut::new(self, from, to)
    }
}

impl<T> MutableCollection for Vec<T> {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }
}
