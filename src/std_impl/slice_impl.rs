// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

impl<T> Collection for &[T] {
    type Position = usize;

    type Element = T;

    type ElementRef<'a>
        = &'a T
    where
        Self: 'a;

    type Whole = Self;

    type Iter<'a>
        = std::slice::Iter<'a, T>
    where
        Self: 'a;

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

    fn iter_within(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::Iter<'_> {
        <[T]>::iter(&self[from..to])
    }

    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self)
    }
}

impl<T> BidirectionalCollection for &[T] {
    fn form_prior(&self, i: &mut Self::Position) {
        *i -= 1;
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        *i -= n;
    }
}

impl<T> RandomAccessCollection for &[T] {}

impl<T> Collection for &mut [T] {
    type Position = usize;

    type Element = T;

    type ElementRef<'a>
        = &'a T
    where
        Self: 'a;

    type Whole = Self;

    type Iter<'a>
        = std::slice::Iter<'a, T>
    where
        Self: 'a;

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

    fn iter_within(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::Iter<'_> {
        <[T]>::iter(&self[from..to])
    }

    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self)
    }
}

impl<T> BidirectionalCollection for &mut [T] {
    fn form_prior(&self, i: &mut Self::Position) {
        *i -= 1;
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        *i -= n;
    }
}

impl<T> RandomAccessCollection for &mut [T] {}

impl<T> ReorderableCollection for &mut [T] {
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

impl<T> MutableCollection for &mut [T] {
    type IterMut<'a>
        = std::slice::IterMut<'a, T>
    where
        Self: 'a;

    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }

    fn iter_mut_within(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::IterMut<'_> {
        <[T]>::iter_mut(&mut self[from..to])
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <[T]>::iter_mut(self)
    }
}
