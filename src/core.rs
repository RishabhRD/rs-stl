// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// Any type that is movable, destructable and equality comparable.
///
/// As per Stepanov (not exact), Type is
///   - Movable
///   - Destructable
///   - Equality comparable
pub trait SemiRegular: Eq {}
impl<T> SemiRegular for T where T: Eq {}

/// Any SemiRegular type that is cloneable.
///
/// As per Stepanov (not exact), Type is
///   - SemiRegular
///   - Cloneable
pub trait Regular: SemiRegular + Clone {}
impl<T> Regular for T where T: SemiRegular + Clone {}

mod sealed {
    pub trait Sealed: Sized {}
    pub struct Bounds<T>(T);
    impl<T> Sealed for Bounds<T> {}
}
use sealed::{Bounds, Sealed};

pub trait HasPosition {
    /// Type of position in the collection.
    type Position: Regular;
}

pub trait CollectionLifetime<
    'this,
    ImplicitBounds: Sealed = Bounds<&'this Self>,
>: HasPosition
{
    /// Type of element in the collection.
    type ElementRef;

    /// Type of slice
    type Slice;
}

/// Models a multi-pass linear sequence of elements.
///
/// Representation:
/// ```text
///   _ _ _ _ _ _
///
///   ^            ^
///   |            |
/// start   -->   end
pub trait Collection: for<'this> CollectionLifetime<'this> {
    fn at<'a>(
        &'a self,
        i: &Self::Position,
    ) -> <Self as CollectionLifetime<'a>>::Element
    where
        Self: 'a;

    fn slice<'a>(
        &'a mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as CollectionLifetime<'a>>::Slice
    where
        Self: 'a;
}
