// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, BidirectionalRange, BoundedRange, InputRange, OutputRange};

/// Returns generalized sum with given binary operation of init and all elements in given range in
/// left to right order.
///
/// # Precondition
///
/// # Postcondition
///   - Returns generalized sum with `op` of init and all elements in rng
///     in left to right order.
///   - Complexity: O(n) applications of `op`.
///
/// Where n == `rng.distance(rng.start(), rng.end())`.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = ["Hello", ", ", "World!"];
///
/// let res = rng::fold_left(&arr, String::from(""), |mut x, y|{
///   x.push_str(y);
///   x
/// });
/// assert_eq!(res, "Hello, World!");
///
/// let res = arr.fold_left(String::from(""), |mut x, y|{
///   x.push_str(y);
///   x
/// });
/// assert_eq!(res, "Hello, World!");
/// ```
pub fn fold_left<Range, Result, BinaryOp>(
    rng: &Range,
    mut init: Result,
    op: BinaryOp,
) -> Result
where
    Range: InputRange + ?Sized,
    BinaryOp: Fn(Result, &Range::Element) -> Result,
{
    let mut start = rng.start();
    while !rng.is_end(&start) {
        init = op(init, &rng.at(&start));
        start = rng.after(start);
    }
    init
}

/// Returns generalized sum with given binary operation of init and all elements in given range in
/// right to left order.
///
/// # Precondition
///
/// # Postcondition
///   - Returns generalized sum with `op` of init and all elements in rng at
///     in right to left order.
///   - Complexity: O(n) applications of `op`.
///
/// Where n == `rng.distance(rng.start(), rng.end())`.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = ["Hello", ", ", "World!"];
///
/// let res = rng::fold_right(&arr, String::from(""), |x, mut y|{
///   y.push_str(x);
///   y
/// });
/// assert_eq!(res, "World!, Hello");
///
/// let res = arr.fold_right(String::from(""), |x, mut y|{
///   y.push_str(x);
///   y
/// });
/// assert_eq!(res, "World!, Hello");
/// ```
pub fn fold_right<Range, Result, BinaryOp>(
    rng: &Range,
    init: Result,
    op: BinaryOp,
) -> Result
where
    Range: BidirectionalRange + BoundedRange + ?Sized,
    BinaryOp: Fn(&Range::Element, Result) -> Result,
{
    algo::fold_right(rng, rng.start(), rng.end(), init, op)
}

/// Modifies the given range such that it becomes the prefix sum by given operation of itself.
///
/// # Precondition
///
/// # Postcondition
///   - Modifies rng such that the resulting elements denote the prefix sum
///     by op of rng elements.
///   - Complexity: O(n) applications of op.
///
/// # Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 3];
/// rng::inclusive_scan_inplace(&mut arr, |x, y| x + y);
/// assert!(arr.equals(&[1, 3, 6]));
///
/// let mut arr = [1, 2, 3];
/// arr.inclusive_scan_inplace(|x, y| x + y);
/// assert!(arr.equals(&[1, 3, 6]));
/// ```
pub fn inclusive_scan_inplace<Range, BinaryOp>(rng: &mut Range, op: BinaryOp)
where
    Range: OutputRange + ?Sized,
    BinaryOp: Fn(&Range::Element, &Range::Element) -> Range::Element,
{
    let mut start = rng.start();
    if rng.is_end(&start) {
        return;
    }
    let mut prev = start.clone();
    start = rng.after(start);
    while !rng.is_end(&start) {
        let res = op(&rng.at(&prev), &rng.at(&start));
        *rng.at_mut(&start) = res;
        prev = start.clone();
        start = rng.after(start);
    }
}

pub mod infix {
    use crate::{
        rng, BidirectionalRange, BoundedRange, InputRange, OutputRange,
    };

    /// `fold_left`.
    pub trait STLNumericInputExt: InputRange {
        fn fold_left<Result, BinaryOp>(
            &self,
            init: Result,
            op: BinaryOp,
        ) -> Result
        where
            BinaryOp: Fn(Result, &Self::Element) -> Result,
        {
            rng::fold_left(self, init, op)
        }
    }

    impl<R> STLNumericInputExt for R where R: InputRange + ?Sized {}

    /// `fold_right`.
    pub trait STLNumericBidirExt: BidirectionalRange + BoundedRange {
        fn fold_right<Result, BinaryOp>(
            &self,
            init: Result,
            op: BinaryOp,
        ) -> Result
        where
            BinaryOp: Fn(&Self::Element, Result) -> Result,
        {
            rng::fold_right(self, init, op)
        }
    }

    impl<R> STLNumericBidirExt for R where
        R: BidirectionalRange + BoundedRange + ?Sized
    {
    }

    /// `inclusive_scan_inplace`.
    pub trait STLNumericOutputExt: OutputRange {
        fn inclusive_scan_inplace<BinaryOp>(&mut self, op: BinaryOp)
        where
            BinaryOp: Fn(&Self::Element, &Self::Element) -> Self::Element,
        {
            super::inclusive_scan_inplace(self, op);
        }
    }

    impl<R> STLNumericOutputExt for R where R: OutputRange + ?Sized {}
}
