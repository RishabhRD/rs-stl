// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BoundedRange, InputRange, OutputRange};

/// Copies elements from src to out of dest if it satisfies predicate.
///
/// # Precondition
///
/// # Postcondition
///   - copies elements from src satisfying `pred` to out position of
///     dest.
///   - If elements to be copied are more than dest can occupy, then drop the
///     exceeding elements from last.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n copy operations and maximum N pred applications.
///
/// Where n is number of elements in dest and N is number of elements in src.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
/// let mut dest = [0, 0];
///
/// let i = rng::copy_if(&src, &mut dest, |x| x % 2 == 1);
/// assert!(dest.equals(&[1, 3]));
/// assert_eq!(i, 2);
///
/// let i = src.copy_if(&mut dest, |x| x % 2 == 1);
/// assert!(dest.equals(&[1, 3]));
/// assert_eq!(i, 2);
/// ```
pub fn copy_if<SrcRange, DestRange, Pred>(
    src: &SrcRange,
    dest: &mut DestRange,
    pred: Pred,
) -> DestRange::Position
where
    SrcRange: InputRange<Element = DestRange::Element> + BoundedRange + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange + ?Sized,
    Pred: Fn(&SrcRange::Element) -> bool,
{
    let write = dest.start();
    __details_copy::__copy_if(src, src.start(), src.end(), dest, write, pred)
}

/// Copies elements from src to dest.
///
/// # Precondition
///
/// # Postcondition
///   - copies elements from src to dest.
///   - If elements to be copied are more than dest can occupy, then drop the
///     exceeding elements from last.
///   - Returns the position of dest just after last copy position.
///   - Complexity: O(n). Total n copy operations.
///
/// where n is number of elements in dest.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 2, 3];
///
/// let mut dest = [0, 0, 0];
/// let i = rng::copy(&src, &mut dest);
/// assert!(dest.equals(&[1, 2, 3]));
/// assert_eq!(i, 3);
///
/// let mut dest = [0, 0, 0];
/// let i = src.copy(&mut dest);
/// assert!(dest.equals(&[1, 2, 3]));
/// assert_eq!(i, 3);
/// ```
pub fn copy<SrcRange, DestRange>(
    src: &SrcRange,
    dest: &mut DestRange,
) -> DestRange::Position
where
    SrcRange: InputRange<Element = DestRange::Element> + BoundedRange + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange + ?Sized,
{
    let write = dest.start();
    __details_copy::__copy(src, src.start(), src.end(), dest, write)
}

pub mod infix {
    use crate::{rng, BoundedRange, InputRange, OutputRange};

    /// `copy`, `copy_if`.
    pub trait STLCopyExt: InputRange + BoundedRange {
        fn copy<DestRange>(&self, dest: &mut DestRange) -> DestRange::Position
        where
            DestRange: OutputRange<Element = Self::Element> + ?Sized,
            DestRange::Element: Clone,
        {
            rng::copy(self, dest)
        }

        fn copy_if<DestRange, Pred>(
            &self,
            dest: &mut DestRange,
            pred: Pred,
        ) -> DestRange::Position
        where
            DestRange: OutputRange<Element = Self::Element> + ?Sized,
            DestRange::Element: Clone,
            Pred: Fn(&Self::Element) -> bool,
        {
            rng::copy_if(self, dest, pred)
        }
    }

    impl<R> STLCopyExt for R where R: InputRange + BoundedRange + ?Sized {}
}

pub mod __details_copy {
    use crate::{BoundedRange, InputRange, OutputRange};

    pub fn __copy<SrcRange, DestRange>(
        src: &SrcRange,
        mut start: SrcRange::Position,
        end: SrcRange::Position,
        dest: &mut DestRange,
        mut write: DestRange::Position,
    ) -> DestRange::Position
    where
        SrcRange: InputRange<Element = DestRange::Element> + ?Sized,
        SrcRange::Element: Clone,
        DestRange: OutputRange + ?Sized,
    {
        while start != end && dest.is_end(&write) {
            *dest.at_mut(&write) = src.at(&start).clone();
            start = src.after(start);
            write = dest.after(write);
        }
        write
    }

    pub fn __copy_if<SrcRange, DestRange, Pred>(
        src: &SrcRange,
        mut start: SrcRange::Position,
        end: SrcRange::Position,
        dest: &mut DestRange,
        mut write: DestRange::Position,
        pred: Pred,
    ) -> DestRange::Position
    where
        SrcRange:
            InputRange<Element = DestRange::Element> + BoundedRange + ?Sized,
        SrcRange::Element: Clone,
        DestRange: OutputRange + ?Sized,
        Pred: Fn(&SrcRange::Element) -> bool,
    {
        while start != end && dest.is_end(&write) {
            if pred(src.at(&start)) {
                *dest.at_mut(&write) = src.at(&start).clone();
                write = dest.after(write);
            }
            start = src.after(start);
        }
        write
    }
}
