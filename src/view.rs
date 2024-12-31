// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! # Views module
//!
//! The `view` module provides collection of views over given range.
//!
//! A view is just a range that implements `View` marker trait.
//!
//! For obtaining view from a range use `.view()` or `.view_mut()` function.
//!
//! ```rust
//! use stl::*;
//!
//! let arr = [1, 2, 3];
//! let v = arr.view(); // An immutable view of arr.
//!
//! let mut arr = [1, 2, 3];
//! let mut v = arr.view_mut(); // A mutable view of arr.
//! ```

use crate::{InputRange, SemiOutputRange};

pub mod view_details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    pub struct RangeView<'a, Range: InputRange + ?Sized> {
        pub range: &'a Range,
    }

    pub struct RangeMutView<'a, Range: SemiOutputRange + ?Sized> {
        pub range: &'a mut Range,
    }

    impl<R> Clone for RangeView<'_, R>
    where
        R: InputRange + ?Sized,
    {
        fn clone(&self) -> Self {
            Self { range: self.range }
        }
    }

    impl<R> View for RangeView<'_, R> where R: InputRange + ?Sized {}
    impl<R> View for RangeMutView<'_, R> where R: SemiOutputRange + ?Sized {}

    impl<R> InputRange for RangeView<'_, R>
    where
        R: InputRange + ?Sized,
    {
        type Element = R::Element;

        type Position = R::Position;

        fn start(&self) -> Self::Position {
            self.range.start()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i)
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            self.range.after(i)
        }

        fn at(&self, i: &Self::Position) -> &Self::Element {
            self.range.at(i)
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.after_n(i, n)
        }
    }

    impl<R> InputRange for RangeMutView<'_, R>
    where
        R: SemiOutputRange + ?Sized,
    {
        type Element = R::Element;

        type Position = R::Position;

        fn start(&self) -> Self::Position {
            self.range.start()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i)
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            self.range.after(i)
        }

        fn at(&self, i: &Self::Position) -> &Self::Element {
            self.range.at(i)
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.after_n(i, n)
        }
    }

    impl<R> BoundedRange for RangeView<'_, R>
    where
        R: BoundedRange + ?Sized,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<R> BoundedRange for RangeMutView<'_, R>
    where
        R: BoundedRange + SemiOutputRange + ?Sized,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<R> ForwardRange for RangeView<'_, R>
    where
        R: ForwardRange + ?Sized,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R> ForwardRange for RangeMutView<'_, R>
    where
        R: SemiOutputRange + ?Sized,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R> BidirectionalRange for RangeView<'_, R>
    where
        R: BidirectionalRange + ?Sized,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            self.range.before(i)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.before_n(i, n)
        }
    }

    impl<R> BidirectionalRange for RangeMutView<'_, R>
    where
        R: BidirectionalRange + SemiOutputRange + ?Sized,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            self.range.before(i)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.before_n(i, n)
        }
    }

    impl<R> RandomAccessRange for RangeView<'_, R> where
        R: RandomAccessRange + ?Sized
    {
    }
    impl<R> RandomAccessRange for RangeMutView<'_, R> where
        R: RandomAccessRange + SemiOutputRange + ?Sized
    {
    }

    impl<R> SemiOutputRange for RangeMutView<'_, R>
    where
        R: SemiOutputRange + ?Sized,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j);
        }
    }

    impl<R> OutputRange for RangeMutView<'_, R>
    where
        R: OutputRange + ?Sized,
    {
        fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
            self.range.at_mut(i)
        }
    }
}

/// Provides `view` method for ranges.
pub trait ViewExt: InputRange {
    /// Returns view that immutably borrows from self.
    ///
    /// `Position` for the view would be same as `Position` type of self.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    /// use rng::infix::*;
    ///
    /// let mut sum = 0;
    /// let arr = [1, 2, 3];
    /// arr.view().for_each(|x| sum += x);
    /// assert_eq!(sum, 6);
    /// ```
    fn view(&self) -> view_details::RangeView<Self> {
        view_details::RangeView { range: self }
    }
}
impl<R> ViewExt for R where R: InputRange + ?Sized {}

/// Provides `view_mut` method for ranges.
pub trait MutableViewExt: SemiOutputRange {
    /// Returns view that mutably borrows from self.
    ///
    /// `Position` for the view would be same as `Position` type of self.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    /// use rng::infix::*;
    ///
    /// let mut arr = [2, 1, 3];
    /// arr.view_mut().sort_range();
    /// assert_eq!(arr, [1, 2, 3]);
    /// ```
    fn view_mut(&mut self) -> view_details::RangeMutView<Self> {
        view_details::RangeMutView { range: self }
    }
}

impl<R> MutableViewExt for R where R: SemiOutputRange + ?Sized {}
