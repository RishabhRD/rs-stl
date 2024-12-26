// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn merge_by() {
        let arr1 = [(1, 1), (1, 3), (2, 3)];
        let arr2 = [(1, 2), (2, 2), (2, 4)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let out = dest.start();
        let i = algo::merge_by(
            &arr1,
            arr1.start(),
            arr1.end(),
            &arr2,
            arr2.start(),
            arr2.end(),
            &mut dest,
            out,
            |x, y| x.0 < y.0,
        );
        assert_eq!(i, 6);
        assert!(&dest[0..i].equals(&[
            (1, 1),
            (1, 3),
            (1, 2),
            (2, 3),
            (2, 2),
            (2, 4)
        ]));

        let arr1 = [(1, 1), (1, 3), (2, 3)];
        let arr2 = [(1, 2), (2, 2), (2, 4)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge_by(&arr1, &arr2, &mut dest, |x, y| x.0 < y.0);
        assert_eq!(i, 6);
        assert!(&dest[0..i].equals(&[
            (1, 1),
            (1, 3),
            (1, 2),
            (2, 3),
            (2, 2),
            (2, 4)
        ]));

        let arr1 = [(1, 1), (1, 3)];
        let arr2 = [(1, 2), (2, 2), (2, 4)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge_by(&arr1, &arr2, &mut dest, |x, y| x.0 < y.0);
        assert_eq!(i, 5);
        assert!(&dest[0..i].equals(&[(1, 1), (1, 3), (1, 2), (2, 2), (2, 4)]));

        let arr1 = [(1, 1), (1, 3), (2, 3)];
        let arr2 = [(1, 2), (2, 2)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge_by(&arr1, &arr2, &mut dest, |x, y| x.0 < y.0);
        assert_eq!(i, 5);
        assert!(&dest[0..i].equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2),]));

        let arr1 = [];
        let arr2 = [(1, 2), (2, 2)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge_by(&arr1, &arr2, &mut dest, |x, y| x.0 < y.0);
        assert_eq!(i, 2);
        assert!(&dest[0..i].equals(&[(1, 2), (2, 2)]));

        let arr1 = [(1, 1), (1, 3), (2, 3)];
        let arr2 = [];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge_by(&arr1, &arr2, &mut dest, |x, y| x.0 < y.0);
        assert_eq!(i, 3);
        assert!(&dest[0..i].equals(&[(1, 1), (1, 3), (2, 3)]));

        let arr1 = [];
        let arr2 = [];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge_by(&arr1, &arr2, &mut dest, |x, y| x.0 < y.0);
        assert_eq!(i, 0);
        assert!(&dest[0..i].equals(&[]));
    }

    #[test]
    fn merge() {
        let arr1 = [(1, 1), (1, 3), (2, 3)];
        let arr2 = [(1, 2), (2, 2), (2, 4)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let out = dest.start();
        let i = algo::merge(
            &arr1,
            arr1.start(),
            arr1.end(),
            &arr2,
            arr2.start(),
            arr2.end(),
            &mut dest,
            out,
        );
        assert_eq!(i, 6);
        assert!(&dest[0..i].equals(&[
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 2),
            (2, 3),
            (2, 4)
        ]));

        let arr1 = [(1, 1), (1, 3), (2, 3)];
        let arr2 = [(1, 2), (2, 2), (2, 4)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge(&arr1, &arr2, &mut dest);
        assert_eq!(i, 6);
        assert!(&dest[0..i].equals(&[
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 2),
            (2, 3),
            (2, 4)
        ]));

        let arr1 = [(1, 1), (1, 3)];
        let arr2 = [(1, 2), (2, 2), (2, 4)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge(&arr1, &arr2, &mut dest);
        assert_eq!(i, 5);
        assert!(&dest[0..i].equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 4)]));

        let arr1 = [(1, 1), (1, 3), (2, 3)];
        let arr2 = [(1, 2), (2, 2)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge(&arr1, &arr2, &mut dest);
        assert_eq!(i, 5);
        assert!(&dest[0..i].equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3),]));

        let arr1 = [];
        let arr2 = [(1, 2), (2, 2)];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge(&arr1, &arr2, &mut dest);
        assert_eq!(i, 2);
        assert!(&dest[0..i].equals(&[(1, 2), (2, 2)]));

        let arr1 = [(1, 1), (1, 3), (2, 3)];
        let arr2 = [];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge(&arr1, &arr2, &mut dest);
        assert_eq!(i, 3);
        assert!(&dest[0..i].equals(&[(1, 1), (1, 3), (2, 3)]));

        let arr1 = [];
        let arr2 = [];
        let mut dest = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
        let i = rng::merge(&arr1, &arr2, &mut dest);
        assert_eq!(i, 0);
        assert!(&dest[0..i].equals(&[]));
    }

    #[test]
    fn merge_inplace_by() {
        let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_by(&mut arr, start, 3, end, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));

        let mut arr = [(1, 1), (1, 3), (1, 2), (2, 2), (2, 4)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_by(&mut arr, start, 2, end, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 2), (2, 4)]));

        let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_by(&mut arr, start, 3, end, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2)]));

        let mut arr = [(1, 2), (2, 2), (2, 4)];
        rng::merge_inplace_by(&mut arr, 0, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 2), (2, 2), (2, 4)]));

        let mut arr = [(1, 2), (2, 2), (2, 4)];
        rng::merge_inplace_by(&mut arr, 3, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 2), (2, 2), (2, 4)]));

        let mut arr =
            [(1, "1"), (1, "3"), (2, "3"), (1, "2"), (2, "2"), (2, "4")];
        rng::merge_inplace_by(&mut arr, 3, |x, y| x.0 < y.0);
        assert!(arr.equals(&[
            (1, "1"),
            (1, "3"),
            (1, "2"),
            (2, "3"),
            (2, "2"),
            (2, "4")
        ]));
    }

    #[test]
    fn merge_inplace() {
        let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace(&mut arr, start, 3, end);
        assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));

        let mut arr = [(1, 1), (1, 3), (1, 2), (2, 2), (2, 4)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace(&mut arr, start, 2, end);
        assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 4)]));

        let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace(&mut arr, start, 3, end);
        assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3)]));

        let mut arr = [(1, 2), (2, 2), (2, 4)];
        rng::merge_inplace(&mut arr, 0);
        assert!(arr.equals(&[(1, 2), (2, 2), (2, 4)]));

        let mut arr = [(1, 2), (2, 2), (2, 4)];
        rng::merge_inplace(&mut arr, 3);
        assert!(arr.equals(&[(1, 2), (2, 2), (2, 4)]));

        let mut arr =
            [(1, "1"), (1, "3"), (2, "3"), (1, "2"), (2, "2"), (2, "4")];
        rng::merge_inplace(&mut arr, 3);
        assert!(arr.equals(&[
            (1, "1"),
            (1, "2"),
            (1, "3"),
            (2, "2"),
            (2, "3"),
            (2, "4")
        ]));
    }

    #[test]
    fn merge_inplace_by_no_alloc() {
        let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_by_no_alloc(&mut arr, start, 3, end, |x, y| {
            x.0 < y.0
        });
        assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2), (2, 4)]));

        let mut arr = [(1, 1), (1, 3), (1, 2), (2, 2), (2, 4)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_by_no_alloc(&mut arr, start, 2, end, |x, y| {
            x.0 < y.0
        });
        assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 2), (2, 4)]));

        let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_by_no_alloc(&mut arr, start, 3, end, |x, y| {
            x.0 < y.0
        });
        assert!(arr.equals(&[(1, 1), (1, 3), (1, 2), (2, 3), (2, 2)]));

        let mut arr = [(1, 2), (2, 2), (2, 4)];
        rng::merge_inplace_by_no_alloc(&mut arr, 0, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 2), (2, 2), (2, 4)]));

        let mut arr = [(1, 2), (2, 2), (2, 4)];
        rng::merge_inplace_by_no_alloc(&mut arr, 3, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 2), (2, 2), (2, 4)]));

        let mut arr =
            [(1, "1"), (1, "3"), (2, "3"), (1, "2"), (2, "2"), (2, "4")];
        rng::merge_inplace_by_no_alloc(&mut arr, 3, |x, y| x.0 < y.0);
        assert!(arr.equals(&[
            (1, "1"),
            (1, "3"),
            (1, "2"),
            (2, "3"),
            (2, "2"),
            (2, "4")
        ]));
    }

    #[test]
    fn merge_inplace_no_alloc() {
        let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2), (2, 4)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_no_alloc(&mut arr, start, 3, end);
        assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (2, 4)]));

        let mut arr = [(1, 1), (1, 3), (1, 2), (2, 2), (2, 4)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_no_alloc(&mut arr, start, 2, end);
        assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 4)]));

        let mut arr = [(1, 1), (1, 3), (2, 3), (1, 2), (2, 2)];
        let start = arr.start();
        let end = arr.end();
        algo::merge_inplace_no_alloc(&mut arr, start, 3, end);
        assert!(arr.equals(&[(1, 1), (1, 2), (1, 3), (2, 2), (2, 3)]));

        let mut arr = [(1, 2), (2, 2), (2, 4)];
        rng::merge_inplace_no_alloc(&mut arr, 0);
        assert!(arr.equals(&[(1, 2), (2, 2), (2, 4)]));

        let mut arr = [(1, 2), (2, 2), (2, 4)];
        rng::merge_inplace_no_alloc(&mut arr, 3);
        assert!(arr.equals(&[(1, 2), (2, 2), (2, 4)]));

        let mut arr =
            [(1, "1"), (1, "3"), (2, "3"), (1, "2"), (2, "2"), (2, "4")];
        rng::merge_inplace_no_alloc(&mut arr, 3);
        assert!(arr.equals(&[
            (1, "1"),
            (1, "2"),
            (1, "3"),
            (2, "2"),
            (2, "3"),
            (2, "4")
        ]));
    }
}
