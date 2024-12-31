// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    fn assert_lower_bound<const N: usize>(
        arr: [i32; N],
        n: i32,
        expected: usize,
    ) {
        let i =
            algo::lower_bound_by(&arr, arr.start(), arr.end(), &n, |x, y| {
                x < y
            });
        assert_eq!(i, expected);

        let i = rng::lower_bound_by(&arr, &n, |x, y| x < y);
        assert_eq!(i, expected);

        let i = arr.lower_bound_by(&n, |x, y| x < y);
        assert_eq!(i, expected);

        let i = algo::lower_bound(&arr, arr.start(), arr.end(), &n);
        assert_eq!(i, expected);

        let i = rng::lower_bound(&arr, &n);
        assert_eq!(i, expected);

        let i = arr.lower_bound(&n);
        assert_eq!(i, expected);
    }

    fn assert_upper_bound<const N: usize>(
        arr: [i32; N],
        n: i32,
        expected: usize,
    ) {
        let i =
            algo::upper_bound_by(&arr, arr.start(), arr.end(), &n, |x, y| {
                x < y
            });
        assert_eq!(i, expected);

        let i = rng::upper_bound_by(&arr, &n, |x, y| x < y);
        assert_eq!(i, expected);

        let i = arr.upper_bound_by(&n, |x, y| x < y);
        assert_eq!(i, expected);

        let i = algo::upper_bound(&arr, arr.start(), arr.end(), &n);
        assert_eq!(i, expected);

        let i = rng::upper_bound(&arr, &n);
        assert_eq!(i, expected);

        let i = arr.upper_bound(&n);
        assert_eq!(i, expected);
    }

    fn assert_equal_range<const N: usize>(
        arr: [i32; N],
        n: i32,
        expected_i: usize,
        expected_j: usize,
    ) {
        let (i, j) =
            algo::equal_range_by(&arr, arr.start(), arr.end(), &n, |x, y| {
                x < y
            });
        assert_eq!(i, expected_i);
        assert_eq!(j, expected_j);

        let (i, j) = rng::equal_range_by(&arr, &n, |x, y| x < y);
        assert_eq!(i, expected_i);
        assert_eq!(j, expected_j);

        let (i, j) = arr.equal_range_by(&n, |x, y| x < y);
        assert_eq!(i, expected_i);
        assert_eq!(j, expected_j);

        let (i, j) = algo::equal_range(&arr, arr.start(), arr.end(), &n);
        assert_eq!(i, expected_i);
        assert_eq!(j, expected_j);

        let (i, j) = rng::equal_range(&arr, &n);
        assert_eq!(i, expected_i);
        assert_eq!(j, expected_j);

        let (i, j) = arr.equal_range(&n);
        assert_eq!(i, expected_i);
        assert_eq!(j, expected_j);
    }

    fn assert_binary_search<const N: usize>(
        arr: [i32; N],
        n: i32,
        expected: bool,
    ) {
        let i =
            algo::binary_search_by(&arr, arr.start(), arr.end(), &n, |x, y| {
                x < y
            });
        assert_eq!(i, expected);

        let i = rng::binary_search_by(&arr, &n, |x, y| x < y);
        assert_eq!(i, expected);

        let i = arr.binary_search_by(&n, |x, y| x < y);
        assert_eq!(i, expected);

        let i = algo::binary_search(&arr, arr.start(), arr.end(), &n);
        assert_eq!(i, expected);

        let i = rng::binary_search(&arr, &n);
        assert_eq!(i, expected);

        let i = arr.binary_search(&n);
        assert_eq!(i, expected);
    }

    #[test]
    pub fn lower_bound() {
        let arr = [2, 1, 4, 8, 7];
        assert_lower_bound(arr, 4, 2);

        let arr = [2, 1, 4, 4, 8, 7];
        assert_lower_bound(arr, 4, 2);

        let arr = [2, 1, 5, 8, 7];
        assert_lower_bound(arr, 4, 2);

        let arr = [2, 1];
        assert_lower_bound(arr, 4, 2);

        let arr = [5, 8, 7];
        assert_lower_bound(arr, 4, 0);

        let arr = [];
        assert_lower_bound(arr, 4, 0);
    }

    #[test]
    pub fn upper_bound() {
        let arr = [2, 1, 4, 8, 7];
        assert_upper_bound(arr, 4, 3);

        let arr = [2, 1, 4, 4, 8, 7];
        assert_upper_bound(arr, 4, 4);

        let arr = [2, 1, 5, 8, 7];
        assert_upper_bound(arr, 4, 2);

        let arr = [2, 1];
        assert_upper_bound(arr, 4, 2);

        let arr = [5, 8, 7];
        assert_upper_bound(arr, 4, 0);

        let arr = [];
        assert_upper_bound(arr, 4, 0);
    }

    #[test]
    pub fn equal_range() {
        let arr = [2, 1, 4, 4, 8, 7];
        assert_equal_range(arr, 4, 2, 4);

        let arr = [2, 1, 4, 8, 7];
        assert_equal_range(arr, 4, 2, 3);

        let arr = [2, 1, 8, 7];
        assert_equal_range(arr, 4, 2, 2);

        let arr = [2, 1, 4, 4];
        assert_equal_range(arr, 4, 2, 4);

        let arr = [2, 1];
        assert_equal_range(arr, 4, 2, 2);

        let arr = [8, 7];
        assert_equal_range(arr, 4, 0, 0);

        let arr = [];
        assert_equal_range(arr, 4, 0, 0);
    }

    #[test]
    pub fn binary_search() {
        let arr = [2, 1, 4, 4, 8, 7];
        assert_binary_search(arr, 4, true);

        let arr = [2, 1, 4, 8, 7];
        assert_binary_search(arr, 4, true);

        let arr = [2, 1, 8, 7];
        assert_binary_search(arr, 4, false);

        let arr = [];
        assert_binary_search(arr, 4, false);

        let arr = [(1, 3)];
        assert!(arr.binary_search_by(&(1, 2), |x, y| x.0 < y.0));
    }
}
