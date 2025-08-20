// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::algo::reorderable_collection_ext::ReorderableCollectionExt;
use crate::ReorderableCollection;

/// Moves all elements satisfying `belongs_in_second_partition` into a suffix of
/// the collection, preserving their relative order, and returns the start of
/// the resulting suffix.
///
/// # Precondition
///   - `n == c.count()`
///
/// # Postcondition
///   - If no element exists in suffix, returns `c.end()`.
///
/// # Complexity
///   - O(n log(n))
pub fn stable_partition<C, F>(
    c: &mut C,
    mut belongs_in_second_partition: F,
    n: usize,
) -> C::Position
where
    C: ReorderableCollection + ?Sized,
    C::Whole: ReorderableCollection,
    F: FnMut(&C::Element) -> bool + Clone,
{
    if n == 0 {
        return c.start();
    }
    if n == 1 {
        if belongs_in_second_partition(&c.at(&c.start())) {
            return c.start();
        } else {
            return c.end();
        }
    }

    let h = n / 2;
    let i = c.next_n(c.start(), h);
    let j = stable_partition(
        &mut c.prefix_upto_mut(i.clone()),
        belongs_in_second_partition.clone(),
        h,
    );
    let k = stable_partition(
        &mut c.suffix_from_mut(i.clone()),
        belongs_in_second_partition,
        n - h,
    );

    c.slice_mut(j, k).rotate(i)
}
