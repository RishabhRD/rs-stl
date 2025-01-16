// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalRange, InputRange, OutputRange};

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

/// Modifies the given range such that it becomes the prefix sum by given operation of itself.
///
/// # Precondition
///   - `[start, end)` represents valid positions in rng.
///   - start != end
///
/// # Postcondition
///   - Modifies rng at `[start, end)` such that the resulting elements denote
///     the prefix sum by op of rng `[start, end)` elements.
///   - Complexity: O(n) applications of op.
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 2, 3];
/// let start = arr.start();
/// let end = arr.end();
/// algo::inclusive_scan_inplace(&mut arr, start, end, |x, y| x + y);
/// assert!(arr.equals(&[1, 3, 6]));
/// ```
pub fn inclusive_scan_inplace<Range, BinaryOp>(
    rng: &mut Range,
    mut start: Range::Position,
    end: Range::Position,
    op: BinaryOp,
) where
    Range: OutputRange + ?Sized,
    BinaryOp: Fn(&Range::Element, &Range::Element) -> Range::Element,
{
    let mut prev = start.clone();
    start = rng.after(start);
    while start != end {
        let res = op(&rng.at(&prev), &rng.at(&start));
        *rng.at_mut(&start) = res;
        prev = start.clone();
        start = rng.after(start);
    }
}
