// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

mod __details {
    use std::cell::RefCell;

    use crate::{InputRange, View};

    // TODO: RefCell is an ovehead.
    //
    // This raises question if InputRange should even exist or not?
    // As pure InputRange like file stream have tendency to mutate while
    // iteration. However, after and at assumes
    pub struct GenerateView<F, O>
    where
        F: FnMut() -> Option<O>,
    {
        pub f: RefCell<F>,
        pub val: RefCell<Option<O>>,
    }

    impl<F, O> GenerateView<F, O>
    where
        F: FnMut() -> Option<O>,
    {
        pub fn new(f: F) -> Self {
            GenerateView {
                f: RefCell::new(f),
                val: RefCell::new(None),
            }
        }
    }

    impl<F, O> InputRange for GenerateView<F, O>
    where
        F: FnMut() -> Option<O>,
    {
        type Element = O;

        type Position = ();

        type ElementRef<'a>
            = std::cell::Ref<'a, O>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            self.val.replace(self.f.borrow_mut()());
        }

        fn is_end(&self, _: &Self::Position) -> bool {
            self.val.borrow().is_none()
        }

        fn after(&self, _: Self::Position) -> Self::Position {
            self.start();
        }

        fn at<'a>(&'a self, _: &Self::Position) -> Self::ElementRef<'a> {
            std::cell::Ref::map(self.val.borrow(), |opt| {
                opt.as_ref().expect("Called `at` on `end` Position")
            })
        }
    }

    impl<F, O> View for GenerateView<F, O> where F: FnMut() -> Option<O> {}
}

/// Returns a view which contains element by repeatedly calling generator, until it returs a `None` Option.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view which contains element by repeatedly calling generator,
///     until it returs a `None` Option.
///   - It satisfies following range traits:
///     - InputRange: YES
///     - BoundedRange: NO
///     - ForwardRange: NO
///     - BidirectionalRange: NO
///     - RandomAccessRange: NO
///     - SemiOutputRange: NO
///     - OutputRange: NO
///
/// # Infix Supported
///   - NO
///
/// # Examples
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut counter = 0;
/// let generator = || {
///     if counter <= 3 {
///         let result = Some(counter);
///         counter += 1;
///         result
///     } else {
///         None
///     }
/// };
/// let arr = view::generate(generator);
/// assert!(arr.equals(&[0, 1, 2, 3]));
/// ```
pub fn generate<Generator, T>(
    generator: Generator,
) -> __details::GenerateView<Generator, T>
where
    Generator: FnMut() -> Option<T>,
{
    __details::GenerateView::new(generator)
}
