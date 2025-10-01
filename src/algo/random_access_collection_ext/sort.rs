// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, CollectionExt, RandomAccessCollection,
    ReorderableCollection, ReorderableCollectionExt,
};

/// Sorts the collection in place, using the given predicate as comparision between elements.
///
/// # Precondition:
///   - `are_in_increasing_order` should follow strict weak ordering.
///
/// # Postcondition:
///   - Relative ordering of equivalent elements are NOT guaranteed to be presevered.
///
/// # Complexity:
///   - O(n * log(n)) worst case where `n == collection.count()`.
pub(crate) fn sort_unstable_by<C, Compare>(
    collection: &mut C,
    are_in_increasing_order: Compare,
) where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool + Clone,
{
    let n = collection.count();
    if n <= 16 {
        insertion_sort(collection, are_in_increasing_order);
    } else {
        let quick_sort_depth = 2 * n.ilog2() as usize;
        if !quick_sort_within(
            collection,
            are_in_increasing_order.clone(),
            quick_sort_depth,
        ) {
            heap_sort(collection, are_in_increasing_order);
        }
    }
}

/// Sorts the collection in place, using the given predicate as comparision between elements.
///
/// # Precondition:
///   - `are_in_increasing_order` should follow strict weak ordering.
///
/// # Postcondition:
///   - Relative ordering of equivalent elements are guaranteed to be preserved.
///
/// # Complexity:
///   - O(n * n) where `n == collection.count()`.
pub(crate) fn insertion_sort<C, Compare>(
    collection: &mut C,
    are_in_increasing_order: Compare,
) where
    C: ReorderableCollection + BidirectionalCollection + ?Sized,
    C::Whole: ReorderableCollection + BidirectionalCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool,
{
    if collection.is_empty() {
        return;
    }

    let mut sorted_end = collection.next_n(collection.start(), 1);
    let end = collection.end();
    while sorted_end != end {
        let mut i = sorted_end.clone();
        loop {
            let j = collection.prior(i.clone());
            if !are_in_increasing_order(&collection.at(&i), &collection.at(&j))
            {
                break;
            }
            collection.swap_at(&i, &j);
            i = j;
            if i == collection.start() {
                break;
            }
        }
        collection.form_next(&mut sorted_end);
    }
}

/// Sorts the collection in place, using the given predicate as comparision between elements
/// using recursion within given depth and returns true. If sorting can't be done  within
/// given depth, returns false.
///
/// # Precondition:
///   - `are_in_increasing_order` should follow strict weak ordering.
///
/// # Postcondition:
///   - Relative ordering of equivalent elements are NOT guaranteed to be preserved.
///
/// # Complexity:
///   - O(n * `depth`) where `n == collection.count()`.
pub(crate) fn quick_sort_within<C, Compare>(
    collection: &mut C,
    are_in_increasing_order: Compare,
    depth: usize,
) -> bool
where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool + Clone,
{
    if collection.start() == collection.end()
        || collection.next(collection.start()) == collection.end()
    {
        return true;
    }

    if depth == 0 {
        return false;
    }

    let start = collection.start();

    // Partition collection except first element.
    let p = {
        let mut rest = collection.full_mut();
        let pivot = unsafe { rest.pop_first().unwrap_unchecked() };
        rest.partition(|e| !are_in_increasing_order(e, &pivot))
    };

    // Fix posiiton of first element.
    let partition_point = collection.prior(p);
    collection.swap_at(&start, &partition_point);

    // Quick sort both parts.
    let left = quick_sort_within(
        &mut collection.prefix_upto_mut(partition_point.clone()),
        are_in_increasing_order.clone(),
        depth - 1,
    );

    let right = quick_sort_within(
        &mut collection.suffix_from_mut(partition_point),
        are_in_increasing_order,
        depth - 1,
    );

    left && right
}

/// Restore the property of heap where all elements are heap except the root.
///
/// # Precondition
///   - `are_in_increasing_order` should follow a total preorder.
///
/// # Complexity
///   - O(log n) where `n == elements.count()`.
pub(crate) fn heapify<C, Compare>(
    elements: &mut C,
    are_in_increasing_order: Compare,
) where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool,
{
    let n = elements.count();
    let mut root = 0;
    loop {
        let left_child = 2 * root + 1;
        let right_child = 2 * root + 2;

        let root_pos = elements.next_n(elements.start(), root);

        let mut largest = root;
        let mut largest_pos = root_pos.clone();

        if left_child < n {
            let left_pos = elements.next_n(elements.start(), left_child);
            if are_in_increasing_order(
                &elements.at(&largest_pos),
                &elements.at(&left_pos),
            ) {
                largest_pos = left_pos;
                largest = left_child;
            }
        }

        if right_child < n {
            let right_pos = elements.next_n(elements.start(), right_child);
            if are_in_increasing_order(
                &elements.at(&largest_pos),
                &elements.at(&right_pos),
            ) {
                largest_pos = right_pos;
                largest = right_child;
            }
        }

        if largest == root {
            break;
        }

        elements.swap_at(&root_pos, &largest_pos);
        root = largest;
    }
}

/// Transforms `elements` into max heap according to `are_in_increasing_order`.
///
/// # Precondition
///   - `are_in_increasing_order` follows total preorder.
///
/// # Complexity
///   - O(n) where `n == elements.count()`.
pub(crate) fn make_heap<C, Compare>(
    elements: &mut C,
    are_in_increasing_order: Compare,
) where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool + Clone,
{
    let n = elements.count();

    let mut cur = elements.next_n(elements.start(), n / 2);
    loop {
        heapify(
            &mut elements.suffix_from_mut(cur.clone()),
            are_in_increasing_order.clone(),
        );
        if cur == elements.start() {
            break;
        }
        elements.form_prior(&mut cur);
    }
}

/// Sorts the `elements` in place, using the given predicate as comparision between elements.
///
/// # Precondition:
///   - `are_in_increasing_order` should follow total preorder.
///
/// # Postcondition:
///   - Relative ordering of equivalent elements are NOT guaranteed to be presevered.
///
/// # Complexity:
///   - O(n * log(n)) worst case where `n == elements.count()`.
pub(crate) fn heap_sort<C, Compare>(
    elements: &mut C,
    are_in_increasing_order: Compare,
) where
    C: ReorderableCollection + RandomAccessCollection + ?Sized,
    C::Whole: ReorderableCollection + RandomAccessCollection,
    Compare: Fn(&C::Element, &C::Element) -> bool + Clone,
{
    make_heap(elements, are_in_increasing_order.clone());

    let mut heap = elements.full_mut();
    while heap.count() > 1 {
        heap.swap_at(&heap.start(), &heap.prior(heap.end()));
        heap.drop_last();
        heapify(&mut heap, are_in_increasing_order.clone());
    }
}

mod tests {
    #[test]
    fn heap_sort_test() {
        let mut arr = [3, 2, 1, 4];
        crate::algo::random_access_collection_ext::sort::heap_sort(
            &mut arr,
            |x, y| x < y,
        );
        assert_eq!(arr, [1, 2, 3, 4]);

        let mut arr = [1];
        crate::algo::random_access_collection_ext::sort::heap_sort(
            &mut arr,
            |x, y| x < y,
        );
        assert_eq!(arr, [1]);

        let mut arr: [i32; 0] = [];
        crate::algo::random_access_collection_ext::sort::heap_sort(
            &mut arr,
            |x, y| x < y,
        );
        assert_eq!(arr, []);
    }

    #[test]
    fn insertion_sort_test() {
        let mut arr = [3, 2, 1, 4];
        crate::algo::random_access_collection_ext::sort::insertion_sort(
            &mut arr,
            |x, y| x < y,
        );
        assert_eq!(arr, [1, 2, 3, 4]);

        let mut arr = [1];
        crate::algo::random_access_collection_ext::sort::insertion_sort(
            &mut arr,
            |x, y| x < y,
        );
        assert_eq!(arr, [1]);

        let mut arr: [i32; 0] = [];
        crate::algo::random_access_collection_ext::sort::insertion_sort(
            &mut arr,
            |x, y| x < y,
        );
        assert_eq!(arr, []);
    }
}
