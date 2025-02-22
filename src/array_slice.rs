// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalRange, Collection, RandomAccessRange, Range, RangeBase,
    RangeLifetime,
};

pub struct ArraySlice<'a, T> {
    m_slice: &'a [T],
    m_start: usize,
    m_end: usize,
}

impl<'a, T> ArraySlice<'a, T> {
    pub fn new(slice: &'a [T], start: usize, end: usize) -> Self {
        ArraySlice {
            m_slice: slice,
            m_start: start,
            m_end: end,
        }
    }
}

impl<T> RangeBase for ArraySlice<'_, T> {
    type Position = usize;

    type Element = T;
}

impl<'a, T> RangeLifetime<'a> for ArraySlice<'_, T>
where
    Self: 'a,
{
    type Slice = ArraySlice<'a, T>;

    type MutableSlice = ArraySlice<'a, T>;
}

impl<T> Range for ArraySlice<'_, T> {
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
    ) -> <Self as RangeLifetime<'a>>::Slice
    where
        Self: 'a,
    {
        ArraySlice::new(self.m_slice, from, to)
    }
}

impl<T> BidirectionalRange for ArraySlice<'_, T> {
    fn before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i - n
    }
}

impl<T> RandomAccessRange for ArraySlice<'_, T> {}

impl<T> Collection for ArraySlice<'_, T> {
    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self.m_slice[*i]
    }
}
