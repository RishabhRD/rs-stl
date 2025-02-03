// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalRange, Collection, MutableCollection, MutableRange,
    RandomAccessRange, Range, ReorderableRange,
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

/// Mutable Slice of an `array-like` type.
pub struct MutableArraySlice<'this, T> {
    m_slice: &'this mut [T],
    m_start: usize,
    m_end: usize,
}

impl<'this, T> MutableArraySlice<'this, T> {
    pub fn new(slice: &'this mut [T], start: usize, end: usize) -> Self {
        MutableArraySlice {
            m_slice: slice,
            m_start: start,
            m_end: end,
        }
    }
}

impl<T> Range for ArraySlice<'_, T> {
    type Position = usize;

    type Element = T;

    type SubRange<'a>
        = ArraySlice<'a, T>
    where
        Self: 'a;

    type MutableSubRange<'a>
        = MutableArraySlice<'a, T>
    where
        Self: 'a;

    fn start(&self) -> Self::Position {
        self.m_start
    }

    fn end(&self) -> Self::Position {
        self.m_end
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        &self.m_slice[*i]
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubRange<'_> {
        Self::new(self.m_slice, from, to)
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }
}

impl<T> Range for MutableArraySlice<'_, T> {
    type Position = usize;

    type Element = T;

    type SubRange<'a>
        = ArraySlice<'a, T>
    where
        Self: 'a;

    type MutableSubRange<'a>
        = MutableArraySlice<'a, T>
    where
        Self: 'a;

    fn start(&self) -> Self::Position {
        self.m_start
    }

    fn end(&self) -> Self::Position {
        self.m_end
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        &self.m_slice[*i]
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubRange<'_> {
        ArraySlice::new(self.m_slice, from, to)
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }
}

impl<T> Collection for ArraySlice<'_, T> {
    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self.m_slice[*i]
    }
}

impl<T> Collection for MutableArraySlice<'_, T> {
    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self.m_slice[*i]
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

impl<T> BidirectionalRange for MutableArraySlice<'_, T> {
    fn before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i - n
    }
}

impl<T> RandomAccessRange for ArraySlice<'_, T> {}

impl<T> RandomAccessRange for MutableArraySlice<'_, T> {}

impl<T> MutableRange for MutableArraySlice<'_, T> {
    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::MutableSubRange<'_> {
        Self::new(self.m_slice, from, to)
    }
}

impl<T> ReorderableRange for MutableArraySlice<'_, T> {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.m_slice.swap(*i, *j);
    }
}

impl<T> MutableCollection for MutableArraySlice<'_, T> {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self.m_slice[*i]
    }
}
