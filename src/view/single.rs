// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    #[derive(Clone)]
    pub struct SingleView<T> {
        pub val: T,
    }

    impl<T> InputRange for SingleView<T> {
        type Element = T;

        type Position = bool;

        type ElementRef<'a>
            = &'a T
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            false
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            *i
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            assert!(!i, "call to after at end of range");
            true
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            assert!(!i, "call to at at end of range");
            &self.val
        }
    }

    impl<T> View for SingleView<T> {}

    impl<T> BoundedRange for SingleView<T> {
        fn end(&self) -> Self::Position {
            true
        }
    }

    impl<T> ForwardRange for SingleView<T> {}

    impl<T> BidirectionalRange for SingleView<T> {
        fn before(&self, i: Self::Position) -> Self::Position {
            assert!(i, "call to before on start of range");
            false
        }
    }

    impl<T> RandomAccessRange for SingleView<T> {}

    impl<T> SemiOutputRange for SingleView<T> {
        fn swap_at(&mut self, _: &Self::Position, _: &Self::Position) {}
    }

    impl<T> OutputRange for SingleView<T> {
        type ElementMutRef<'a>
            = &'a mut T
        where
            Self: 'a;

        fn at_mut<'a>(
            &'a mut self,
            i: &Self::Position,
        ) -> Self::ElementMutRef<'a> {
            assert!(!i, "call to at_mut at end of range");
            &mut self.val
        }
    }
}

/// Returns a view that contains an instance of val.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view that contains an instance of val.
///   - View satisfies following range traits:
///     - InputRange: YES
///     - BoundedRange: YES
///     - BidirectionalRange: YES
///     - RandomAccessRange: YES
///     - SemiOutputRange: YES
///     - OutputRange: YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = view::single(2);
/// arr.equals(&[2]);
/// ```
pub fn single<T>(val: T) -> __details::SingleView<T> {
    __details::SingleView { val }
}
