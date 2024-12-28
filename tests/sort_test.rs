// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn sort_range() {
        let mut arr = [2, 4, 4, 1, 2, 5];
        let start = arr.start();
        let end = arr.end();
        algo::sort_range_by(&mut arr, start, end, |x, y| y < x);
        assert!(arr.equals(&[5, 4, 4, 2, 2, 1]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        rng::sort_range_by(&mut arr, |x, y| y < x);
        assert!(arr.equals(&[5, 4, 4, 2, 2, 1]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        arr.sort_range_by(|x, y| y < x);
        assert!(arr.equals(&[5, 4, 4, 2, 2, 1]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        let start = arr.start();
        let end = arr.end();
        algo::sort_range(&mut arr, start, end);
        assert!(arr.equals(&[1, 2, 2, 4, 4, 5]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        rng::sort_range(&mut arr);
        assert!(arr.equals(&[1, 2, 2, 4, 4, 5]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        arr.sort_range();
        assert!(arr.equals(&[1, 2, 2, 4, 4, 5]));

        let mut arr: [i32; 0] = [];
        arr.sort_range();
        assert!(arr.equals(&[]));
    }

    #[test]
    fn stable_sort_by() {
        let mut arr = [(3, "1"), (1, "2"), (2, "2"), (1, "1")];
        let start = arr.start();
        let end = arr.end();
        algo::stable_sort_by(&mut arr, start, end, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, "2"), (1, "1"), (2, "2"), (3, "1")]));

        let mut arr = [(3, "1"), (1, "2"), (2, "2"), (1, "1")];
        rng::stable_sort_by(&mut arr, |x, y| x < y);
        assert!(arr.equals(&[(1, "1"), (1, "2"), (2, "2"), (3, "1")]));

        let mut arr = [(1, "2"), (2, "2"), (1, "1")];
        arr.stable_sort_by(|x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, "2"), (1, "1"), (2, "2")]));

        let mut arr = [(1, "2"), (1, "1")];
        arr.stable_sort_by(|x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, "2"), (1, "1")]));

        let mut arr = [(1, "2")];
        arr.stable_sort_by(|x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, "2")]));

        let mut arr: [i32; 0] = [];
        arr.stable_sort_by(|x, y| x < y);
        assert!(arr.equals(&[]));
    }

    #[test]
    fn stable_sort() {
        let mut arr = [(3, "1"), (1, "2"), (2, "2"), (1, "1")];
        let start = arr.start();
        let end = arr.end();
        algo::stable_sort(&mut arr, start, end);
        assert!(arr.equals(&[(1, "1"), (1, "2"), (2, "2"), (3, "1")]));

        let mut arr = [(3, "1"), (1, "2"), (2, "2"), (1, "1")];
        rng::stable_sort(&mut arr);
        assert!(arr.equals(&[(1, "1"), (1, "2"), (2, "2"), (3, "1")]));

        let mut arr = [(1, "2"), (2, "2"), (1, "1")];
        arr.stable_sort();
        assert!(arr.equals(&[(1, "1"), (1, "2"), (2, "2")]));

        let mut arr = [(1, "2"), (1, "1")];
        arr.stable_sort();
        assert!(arr.equals(&[(1, "1"), (1, "2")]));

        let mut arr = [(1, "2")];
        arr.stable_sort();
        assert!(arr.equals(&[(1, "2")]));

        let mut arr: [i32; 0] = [];
        arr.stable_sort();
        assert!(arr.equals(&[]));
    }

    #[test]
    fn stable_sort_by_no_alloc() {
        let mut arr = [(3, "1"), (1, "2"), (2, "2"), (1, "1")];
        let start = arr.start();
        let end = arr.end();
        algo::stable_sort_by_no_alloc(&mut arr, start, end, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, "2"), (1, "1"), (2, "2"), (3, "1")]));

        let mut arr = [(3, "1"), (1, "2"), (2, "2"), (1, "1")];
        rng::stable_sort_by_no_alloc(&mut arr, |x, y| x < y);
        assert!(arr.equals(&[(1, "1"), (1, "2"), (2, "2"), (3, "1")]));

        let mut arr = [(1, "2"), (2, "2"), (1, "1")];
        arr.stable_sort_by_no_alloc(|x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, "2"), (1, "1"), (2, "2")]));

        let mut arr = [(1, "2"), (1, "1")];
        arr.stable_sort_by_no_alloc(|x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, "2"), (1, "1")]));

        let mut arr = [(1, "2")];
        arr.stable_sort_by_no_alloc(|x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, "2")]));

        let mut arr: [i32; 0] = [];
        arr.stable_sort_by_no_alloc(|x, y| x < y);
        assert!(arr.equals(&[]));
    }

    #[test]
    fn stable_sort_no_alloc() {
        let mut arr = [(3, "1"), (1, "2"), (2, "2"), (1, "1")];
        let start = arr.start();
        let end = arr.end();
        algo::stable_sort_no_alloc(&mut arr, start, end);
        assert!(arr.equals(&[(1, "1"), (1, "2"), (2, "2"), (3, "1")]));

        let mut arr = [(3, "1"), (1, "2"), (2, "2"), (1, "1")];
        rng::stable_sort_no_alloc(&mut arr);
        assert!(arr.equals(&[(1, "1"), (1, "2"), (2, "2"), (3, "1")]));

        let mut arr = [(1, "2"), (2, "2"), (1, "1")];
        arr.stable_sort_no_alloc();
        assert!(arr.equals(&[(1, "1"), (1, "2"), (2, "2")]));

        let mut arr = [(1, "2"), (1, "1")];
        arr.stable_sort_no_alloc();
        assert!(arr.equals(&[(1, "1"), (1, "2")]));

        let mut arr = [(1, "2")];
        arr.stable_sort_no_alloc();
        assert!(arr.equals(&[(1, "2")]));

        let mut arr: [i32; 0] = [];
        arr.stable_sort_no_alloc();
        assert!(arr.equals(&[]));
    }

    #[test]
    fn is_sorted_until() {
        let arr = [1, 2, 3];

        let i =
            algo::is_sorted_until_by(&arr, arr.start(), arr.end(), |x, y| {
                x < y
            });
        assert_eq!(i, 3);

        let i = rng::is_sorted_until_by(&arr, |x, y| x < y);
        assert_eq!(i, 3);

        let i = arr.is_sorted_until_by(|x, y| x < y);
        assert_eq!(i, 3);

        let i = algo::is_sorted_until(&arr, arr.start(), arr.end());
        assert_eq!(i, 3);

        let i = rng::is_sorted_until(&arr);
        assert_eq!(i, 3);

        let i = arr.is_sorted_until();
        assert_eq!(i, 3);

        let arr = [9, 1, 2, 1];
        let i = algo::is_sorted_until_by(
            &arr,
            arr.start() + 1,
            arr.end(),
            |x, y| x < y,
        );
        assert_eq!(i, 3);

        let arr: [i32; 0] = [];
        let i = arr.is_sorted_until();
        assert_eq!(i, 0);

        let arr = [1, 1, 2, 3];
        let i = arr.is_sorted_until();
        assert_eq!(i, 4);

        let arr = [3, 2, 1, 1];
        let i = arr.is_sorted_until_by(|x, y| x > y);
        assert_eq!(i, 4);

        let arr = [(1, 2), (1, 1)];
        let i = arr.is_sorted_until_by(|x, y| x.0 < y.0);
        assert_eq!(i, 2);

        let arr = [(1, 2), (1, 1)];
        let i = arr.is_sorted_until();
        assert_eq!(i, 1);
    }

    #[test]
    fn is_sorted() {
        let arr = [1, 2, 3];
        assert!(algo::is_sorted_by(&arr, arr.start(), arr.end(), |x, y| x < y));
        assert!(rng::is_sorted_by(&arr, |x, y| x < y));
        assert!(arr.is_sorted_by(|x, y| x < y));

        assert!(algo::is_sorted(&arr, arr.start(), arr.end()));
        assert!(rng::is_sorted(&arr));
        assert!(arr.is_sorted());

        let arr = [1, 2, 3, 0];
        assert!(algo::is_sorted_by(
            &arr,
            arr.start(),
            arr.end() - 1,
            |x, y| x < y
        ));

        let arr = [3, 2, 1];
        assert!(!arr.is_sorted());
        assert!(arr.is_sorted_by(|x, y| x > y));

        let arr: [i32; 0] = [];
        assert!(arr.is_sorted());

        let arr = [1, 1, 2, 3];
        assert!(arr.is_sorted());

        let arr = [(1, 2), (1, 1)];
        assert!(arr.is_sorted_by(|x, y| x.0 < y.0));
        assert!(!arr.is_sorted());
    }

    #[test]
    fn partial_sort() {
        let mut arr = [4, 1, 5, 1, 2];
        let start = arr.start();
        let mid = arr.after_n(start, 3);
        let end = arr.end();
        algo::partial_sort_by(&mut arr, start, mid, end, |x, y| x < y);
        assert!(&arr[0..3].equals(&[1, 1, 2]));

        let mut arr = [4, 1, 5, 1, 2];
        let mid = arr.after_n(start, 3);
        rng::partial_sort_by(&mut arr, mid, |x, y| x < y);
        assert!(&arr[0..3].equals(&[1, 1, 2]));

        let mut arr = [4, 1, 5, 1, 2];
        let mid = arr.after_n(start, 3);
        arr.partial_sort_by(mid, |x, y| x < y);
        assert!(&arr[0..3].equals(&[1, 1, 2]));

        let mut arr = [4, 1, 5, 1, 2];
        let start = arr.start();
        let mid = arr.after_n(start, 3);
        let end = arr.end();
        algo::partial_sort(&mut arr, start, mid, end);
        assert!(&arr[0..3].equals(&[1, 1, 2]));

        let mut arr = [4, 1, 5, 1, 2];
        let mid = arr.after_n(start, 3);
        rng::partial_sort(&mut arr, mid);
        assert!(&arr[0..3].equals(&[1, 1, 2]));

        let mut arr = [4, 1, 5, 1, 2];
        let mid = arr.after_n(start, 3);
        arr.partial_sort(mid);
        assert!(&arr[0..3].equals(&[1, 1, 2]));

        let mut arr = [4, 1, 5, 1, 2];
        let mid = arr.after_n(start, 3);
        arr.partial_sort_by(mid, |x, y| x > y);
        assert!(&arr[0..3].equals(&[5, 4, 2]));

        let mut arr = [4, 1, 5, 1, 2];
        arr.partial_sort(arr.start());
        assert!(&arr[0..0].equals(&[]));

        let mut arr: [i32; 0] = [];
        arr.partial_sort(arr.start());
        assert!(&arr[0..0].equals(&[]));

        let mut arr = [4, 1, 5, 1, 2];
        arr.partial_sort(arr.end());
        assert!(arr.equals(&[1, 1, 2, 4, 5]));
    }

    #[test]
    fn partial_sort_copy() {
        let arr = [4, 1, 5, 1, 2];
        let mut dest = [0, 0, 0, 0];
        let dest_start = dest.start();
        let dest_end = dest.end();
        let i = algo::partial_sort_copy_by(
            &arr,
            arr.start(),
            arr.end(),
            &mut dest,
            dest_start,
            dest_end,
            |x, y| x < y,
        );
        assert_eq!(i, 4);
        assert!(dest.equals(&[1, 1, 2, 4]));

        let arr = [4, 1, 5, 1, 2];
        let mut dest = [0, 0, 0, 0];
        let i = rng::partial_sort_copy_by(&arr, &mut dest, |x, y| x < y);
        assert_eq!(i, 4);
        assert!(dest.equals(&[1, 1, 2, 4]));

        let arr = [4, 1, 5, 1, 2];
        let mut dest = [0, 0, 0, 0];
        let dest_start = dest.start();
        let dest_end = dest.end();
        let i = algo::partial_sort_copy(
            &arr,
            arr.start(),
            arr.end(),
            &mut dest,
            dest_start,
            dest_end,
        );
        assert_eq!(i, 4);
        assert!(dest.equals(&[1, 1, 2, 4]));

        let arr = [4, 1, 5, 1, 2];
        let mut dest = [0, 0, 0, 0];
        let i = rng::partial_sort_copy(&arr, &mut dest);
        assert_eq!(i, 4);
        assert!(dest.equals(&[1, 1, 2, 4]));

        let arr = [4, 1, 5];
        let mut dest = [0, 0, 0, 0];
        let i = rng::partial_sort_copy(&arr, &mut dest);
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, 4, 5, 0]));

        let arr = [];
        let mut dest = [0, 0, 0, 0];
        let i = rng::partial_sort_copy(&arr, &mut dest);
        assert_eq!(i, 0);
        assert!(dest.equals(&[0, 0, 0, 0]));

        let arr = [4, 1, 5, 1, 2];
        let mut dest = [0, 0, 0, 0, 0, 0];
        let i = rng::partial_sort_copy(&arr, &mut dest);
        assert_eq!(i, 5);
        assert!(dest.equals(&[1, 1, 2, 4, 5, 0]));
    }
}
