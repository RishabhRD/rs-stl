// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalRange, Collection, RandomAccessRange, Range, RangeBase,
    SubRangeable,
};

/// Slice of an `array-like` type.
pub struct ArraySlice<'this, T> {
    m_slice: &'this [T],
    m_start: usize,
    m_end: usize,
}

impl<'this, T> ArraySlice<'this, T> {
    pub fn new(slice: &'this [T], start: usize, end: usize) -> Self {
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

impl<T> SubRangeable<'_> for ArraySlice<'_, T> {
    type SubRange = Self;
}

impl<T> Range for ArraySlice<'_, T> {
    fn start(&self) -> Self::Position {
        self.m_start
    }

    fn end(&self) -> Self::Position {
        self.m_end
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        assert!(i != self.m_end);
        i + 1
    }

    fn slice(&self, from: Self::Position, to: Self::Position) -> Self {
        Self::new(self.m_slice, from, to)
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        assert!(i + n <= self.m_end);
        n + i
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn at_as_deref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        self.at(i)
    }
}

impl<T> Collection for ArraySlice<'_, T> {
    fn at(&self, i: &Self::Position) -> &T {
        assert!(*i != self.m_end);
        &self.m_slice[*i]
    }
}

impl<T> BidirectionalRange for ArraySlice<'_, T> {
    fn before(&self, i: Self::Position) -> Self::Position {
        assert!(i > self.m_start);
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        assert!(i >= n + self.m_start);
        i - n
    }
}

impl<T> RandomAccessRange for ArraySlice<'_, T> {}
