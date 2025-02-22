// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    ArraySlice, BidirectionalRange, Collection, HasSlice, MutableCollection,
    MutableRange, RandomAccessRange, Range, RangeBase,
};

pub struct MutableArraySlice<'a, T> {
    m_slice: &'a mut [T],
    m_start: usize,
    m_end: usize,
}

impl<'a, T> MutableArraySlice<'a, T> {
    pub fn new(slice: &'a mut [T], start: usize, end: usize) -> Self {
        MutableArraySlice {
            m_slice: slice,
            m_start: start,
            m_end: end,
        }
    }
}

impl<T> RangeBase for MutableArraySlice<'_, T> {
    type Position = usize;

    type Element = T;
}

impl<'a, T> HasSlice<'a> for MutableArraySlice<'_, T> {
    type Slice = ArraySlice<'a, T>;

    type MutableSlice = MutableArraySlice<'a, T>;
}

impl<T> Range for MutableArraySlice<'_, T> {
    fn start(&self) -> Self::Position {
        self.m_start
    }

    fn end(&self) -> Self::Position {
        self.m_end
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        &self.m_slice[*i]
    }

    fn slice<'a>(
        &'a self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as HasSlice<'a>>::Slice
    where
        Self: 'a,
    {
        ArraySlice::new(self.m_slice, from, to)
    }
}

impl<T> BidirectionalRange for MutableArraySlice<'_, T> {
    fn before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i - n
    }
}

impl<T> RandomAccessRange for MutableArraySlice<'_, T> {}

impl<T> MutableRange for MutableArraySlice<'_, T> {
    fn slice_mut<'a>(
        &'a mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as HasSlice<'a>>::MutableSlice
    where
        Self: 'a,
    {
        MutableArraySlice::new(self.m_slice, from, to)
    }

    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.m_slice.swap(*i, *j)
    }
}

impl<T> Collection for MutableArraySlice<'_, T> {
    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self.m_slice[*i]
    }
}

impl<T> MutableCollection for MutableArraySlice<'_, T> {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self.m_slice[*i]
    }
}
