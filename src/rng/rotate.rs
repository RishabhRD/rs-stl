// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BoundedRange, ForwardRange, OutputRange, SemiOutputRange};

use super::__details_copy;

/// Rotates the given range at mid.
///
/// # Precondition
///   - mid is a valid position in rng.
///
/// # Postcondition
///   - Swaps element of rng in such a way that the elements at
///     `[rng.start(), mid)` are placed after elements at `[mid, rng.end())`
///     while the orders of both ranges are preserved.
///   - Returns position to element originally at `rng.start()`.
///   - Complexity: O(n). At most n swaps.
///
///   Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [0, 1, 2, 3, 4];
/// let i = rng::rotate(&mut arr, 2); // Position type for array is usize
/// assert_eq!(i, 3);
/// assert!(arr.equals(&[2, 3, 4, 0, 1]));
///
/// let mut arr = [0, 1, 2, 3, 4];
/// let i = arr.rotate(2); // Position type for array is usize
/// assert_eq!(i, 3);
/// assert!(arr.equals(&[2, 3, 4, 0, 1]));
/// ```
///
/// # TODO
///   - There are efficient implementations for BidirectionalRange and
///     RandomAccessRange in rust. How to overload for them in rust?
pub fn rotate<Range>(rng: &mut Range, mid: Range::Position) -> Range::Position
where
    Range: SemiOutputRange + BoundedRange + ?Sized,
{
    __details_rotate::__rotate(rng, rng.start(), mid, rng.end())
}

/// Copies the given range to dest as if it is rotated at mid.
///
/// # Precondition
///   - mid is valid position in src.
///   - dest should be able to accomodate n elements.
///
/// # Postcondition
///   - Copies elements from src to dest in such a way, that the element
///     at mid becomes first element at out and element at (mid - 1) becomes
///     last element.
///   - Complexity: O(n). Exactly n assignments.
///
///   Where n is number of elements in src.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let arr = [0, 1, 2, 3, 4];
/// let mut dest = [0, 0, 0, 0, 0];
/// let i = rng::rotate_copy(&arr, 2, &mut dest); // Position type for array is usize
/// assert_eq!(i, 5);
/// assert!(dest.equals(&[2, 3, 4, 0, 1]));
/// ```
pub fn rotate_copy<SrcRange, DestRange>(
    src: &SrcRange,
    mid: SrcRange::Position,
    dest: &mut DestRange,
) -> DestRange::Position
where
    SrcRange: ForwardRange + BoundedRange + ?Sized,
    SrcRange::Element: Clone,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
{
    let mut out = dest.start();
    out = __details_copy::__copy(src, mid.clone(), src.end(), dest, out);
    __details_copy::__copy(src, src.start(), mid, dest, out)
}

pub mod infix {
    use crate::{rng, BoundedRange, SemiOutputRange};

    /// `rotate`.
    pub trait STLRotateExt: SemiOutputRange + BoundedRange {
        /// TODO: there are efficient implementations for BidirectionalRange and
        /// RandomAccessRange in rust. How to overload for them in rust?
        fn rotate(&mut self, mid: Self::Position) -> Self::Position {
            rng::rotate(self, mid)
        }
    }

    impl<R> STLRotateExt for R where R: SemiOutputRange + BoundedRange + ?Sized {}
}

pub mod __details_rotate {
    use crate::{BoundedRange, SemiOutputRange};

    pub fn __rotate<Range>(
        rng: &mut Range,
        start: Range::Position,
        mid: Range::Position,
        end: Range::Position,
    ) -> Range::Position
    where
        Range: SemiOutputRange + BoundedRange + ?Sized,
    {
        if start == mid {
            return end;
        }
        if mid == end {
            return start;
        }
        let mut write = start.clone();
        let mut next_read = start;
        let mut read = mid.clone();
        while read != end {
            if write == next_read {
                next_read = read.clone();
            }
            rng.swap_at(&write, &read);
            write = rng.after(write);
            read = rng.after(read);
        }
        __rotate(rng, write.clone(), next_read, end);
        write
    }
}
