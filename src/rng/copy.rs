// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange, OutputRange};

/// Copies elements from src to out of dest if it satisfies predicate.
///
/// # Precondition
///   - dest should be able to accomodate all copied elements starting from out.
///
/// # Postcondition
///   - copies elements from src satisfying `pred` to out position of
///     dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n pred applications and maximum n copy operations.
///
/// where n is number of elements in src.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
///
/// let mut dest = [0, 0];
/// let i = rng::copy_if(&src, &mut dest, |x| x % 2 == 1);
/// assert!(dest.equals(&[1, 3]));
/// assert_eq!(i, 2);
/// ```
pub fn copy_if<SrcRange, DestRange, Pred>(
    src: &SrcRange,
    dest: &mut DestRange,
    pred: Pred,
) -> DestRange::Position
where
    SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange + ?Sized,
    Pred: Fn(&SrcRange::Element) -> bool,
{
    algo::copy_if(src, src.start(), src.end(), dest, dest.start(), pred)
}

/// Copies elements from src to dest.
///
/// # Precondition
///   - dest should be able to accomodate n elements
///
/// # Postcondition
///   - copies elements from src to dest.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n copy operations.
///
/// where n is number of elements in src.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0, 0];
///
/// let i = rng::copy(&src, &mut dest);
/// assert!(dest.equals(&[1, 2, 3]));
/// assert_eq!(i, 3);
/// ```
pub fn copy<SrcRange, DestRange>(
    rng: &SrcRange,
    dest: &mut DestRange,
) -> DestRange::Position
where
    SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange + ?Sized,
{
    algo::copy(rng, rng.start(), rng.end(), dest, dest.start())
}
