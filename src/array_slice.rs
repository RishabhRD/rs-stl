// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalCollection, Collection, RandomAccessCollection};

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

impl<T> Collection for ArraySlice<'_, T> {
    type Position = usize;

    type Element<'a>
        = &'a T
    where
        Self: 'a;

    type MutableElement<'a>
        = &'a T
    where
        Self: 'a;

    type Slice<'a>
        = ArraySlice<'a, T>
    where
        Self: 'a;

    type MutableSlice<'a>
        = ArraySlice<'a, T>
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

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn slice<'a, Callback, ReturnType>(
        &'a self,
        from: Self::Position,
        to: Self::Position,
        mut callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(&Self::Slice<'a>) -> ReturnType,
        Self: 'a,
    {
        let slice = ArraySlice::new(self.m_slice, from, to);
        callback(&slice)
    }

    fn at<'a, Callback, ReturnType>(
        &'a self,
        i: &Self::Position,
        mut callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(Self::Element<'a>) -> ReturnType,
        Self: 'a,
    {
        callback(&self.m_slice[*i])
    }
}

impl<T> Collection for MutableArraySlice<'_, T> {
    type Position = usize;

    type Element<'a>
        = &'a T
    where
        Self: 'a;

    type MutableElement<'a>
        = &'a mut T
    where
        Self: 'a;

    type Slice<'a>
        = ArraySlice<'a, T>
    where
        Self: 'a;

    type MutableSlice<'a>
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

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn slice<'a, Callback, ReturnType>(
        &'a self,
        from: Self::Position,
        to: Self::Position,
        mut callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(&Self::Slice<'a>) -> ReturnType,
        Self: 'a,
    {
        let slice = ArraySlice::new(self.m_slice, from, to);
        callback(&slice)
    }

    fn at<'a, Callback, ReturnType>(
        &'a self,
        i: &Self::Position,
        mut callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(Self::Element<'a>) -> ReturnType,
        Self: 'a,
    {
        callback(&self.m_slice[*i])
    }
}

impl<T> BidirectionalCollection for ArraySlice<'_, T> {
    fn before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i - n
    }
}

impl<T> BidirectionalCollection for MutableArraySlice<'_, T> {
    fn before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i - n
    }
}

impl<T> RandomAccessCollection for ArraySlice<'_, T> {}

impl<T> RandomAccessCollection for MutableArraySlice<'_, T> {}
