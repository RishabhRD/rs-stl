// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[doc(hidden)]
pub mod __details_view_ints {
    use crate::{
        util::ValueRef, BidirectionalRange, ForwardRange, InputRange,
        RandomAccessRange, View,
    };

    #[derive(Clone)]
    pub struct IntView {
        pub init: i32,
    }

    impl InputRange for IntView {
        type Element = i32;

        type ElementRef<'a>
            = ValueRef<i32>
        where
            Self: 'a;

        type Position = i32;

        fn start(&self) -> Self::Position {
            self.init
        }

        fn is_end(&self, _: &Self::Position) -> bool {
            false
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            i + 1
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            ValueRef { val: *i }
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            i + n as i32
        }
    }

    impl View for IntView {}

    impl ForwardRange for IntView {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            (to - from) as usize
        }
    }

    impl BidirectionalRange for IntView {
        fn before(&self, i: Self::Position) -> Self::Position {
            i - 1
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            i - n as i32
        }
    }

    impl RandomAccessRange for IntView {}
}

/// Returns an infinite sequence of i32s starting from init.
///
/// # Precondition
///
/// # Postcondition
///   - InputRange -> YES
///   - BoundedRange -> NO
///   - ForwardRange -> YES
///   - BidirectionalRange -> YES
///   - RandomAccessRange -> YES
///   - SemiOutputRange -> NO
///   - OutputRange -> NO
///
/// # Infix Supported
/// N/A
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let nums = view::ints(0); // 0, 1, 2, 3, ...
/// let i = nums.find_if(|x| *x == 10);
/// assert_eq!(i, 10);
/// ```
pub fn ints(init: i32) -> __details_view_ints::IntView {
    __details_view_ints::IntView { init }
}
