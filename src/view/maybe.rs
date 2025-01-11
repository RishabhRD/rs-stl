// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    #[derive(Clone)]
    pub struct MaybeView<T> {
        pub val: Option<T>,
    }

    impl<T> InputRange for MaybeView<T> {
        type Element = T;

        type Position = bool;

        type ElementRef<'a>
            = &'a T
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            self.val.is_some()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            !i
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            assert!(i, "call to after at end");
            false
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            assert!(i, "call to at on end");
            self.val.as_ref().unwrap()
        }
    }

    impl<T> View for MaybeView<T> {}

    impl<T> BoundedRange for MaybeView<T> {
        fn end(&self) -> Self::Position {
            false
        }
    }

    impl<T> ForwardRange for MaybeView<T> {}

    impl<T> BidirectionalRange for MaybeView<T> {
        fn before(&self, i: Self::Position) -> Self::Position {
            assert!(!i, "call to before on start");
            true
        }
    }

    impl<T> RandomAccessRange for MaybeView<T> {}

    impl<T> SemiOutputRange for MaybeView<T> {
        fn swap_at(&mut self, _: &Self::Position, _: &Self::Position) {}
    }

    impl<T> OutputRange for MaybeView<T> {
        type ElementMutRef<'a>
            = &'a mut T
        where
            Self: 'a;

        fn at_mut<'a>(
            &'a mut self,
            i: &Self::Position,
        ) -> Self::ElementMutRef<'a> {
            assert!(i, "call to at on end");
            self.val.as_mut().unwrap()
        }
    }
}

/// Returns a view that contains a single instance of val or is empty.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view that contains a single instance of val or is empty.
///   - View satisfies following range traits:
///     - InputRange -> YES
///     - BoundedRange -> YES
///     - ForwardRange -> YES
///     - BidirectionalRange -> YES
///     - RandomAccessRange -> YES
///     - SemiOutputRange -> YES
///     - OutputRange -> YES
///
/// # Infix Supported
/// NO
///
/// # Examples
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = view::maybe(Some(2));
/// assert!(arr.equals(&[2]));
///
/// let arr = view::maybe(Option::<i32>::None);
/// assert!(arr.equals(&[]));
/// ```
pub fn maybe<T>(val: Option<T>) -> __details::MaybeView<T> {
    __details::MaybeView { val }
}
