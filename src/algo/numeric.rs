// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalRange, InputRange};

/// Returns generalized sum with given binary operation of init and all elements in given range in
/// left to right order.
///
/// # Precondition
///   - `[start, end)` denotes valid positions in rng.
///
/// # Postcondition
///   - Returns generalized sum with `op` of init and all elements in rng at
///     `[start, end)` in left to right order.
///   - Complexity: O(n) applications of `op`.
///
/// Where n == `rng.distance(start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = ["Hello", ", ", "World!"];
/// let res = algo::fold_left(&arr, arr.start(), arr.end(), String::from(""), |mut x, y|{
///   x.push_str(y);
///   x
/// });
/// assert_eq!(res, "Hello, World!");
/// ```
pub fn fold_left<Range, Result, BinaryOp>(
    rng: &Range,
    mut start: Range::Position,
    end: Range::Position,
    mut init: Result,
    op: BinaryOp,
) -> Result
where
    Range: InputRange + ?Sized,
    BinaryOp: Fn(Result, &Range::Element) -> Result,
{
    while start != end {
        init = op(init, &rng.at(&start));
        start = rng.after(start);
    }
    init
}

/// Returns generalized sum with given binary operation of init and all elements in given range in
/// right to left order.
///
/// # Precondition
///   - `[start, end)` denotes valid positions in rng.
///
/// # Postcondition
///   - Returns generalized sum with `op` of init and all elements in rng at
///     `[start, end)` in right to left order.
///   - Complexity: O(n) applications of `op`.
///
/// Where n == `rng.distance(start, end)`.
///
/// # Example
/// ```rust
/// use stl::*;
///
/// let arr = ["Hello", ", ", "World!"];
/// let res = algo::fold_right(&arr, arr.start(), arr.end(), String::from(""), |x, mut y|{
///   y.push_str(x);
///   y
/// });
/// assert_eq!(res, "World!, Hello");
/// ```
pub fn fold_right<Range, Result, BinaryOp>(
    rng: &Range,
    start: Range::Position,
    mut end: Range::Position,
    mut init: Result,
    op: BinaryOp,
) -> Result
where
    Range: BidirectionalRange + ?Sized,
    BinaryOp: Fn(&Range::Element, Result) -> Result,
{
    while start != end {
        end = rng.before(end);
        init = op(&rng.at(&end), init);
    }
    init
}
