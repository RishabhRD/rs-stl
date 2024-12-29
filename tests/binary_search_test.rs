// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    pub fn lower_bound() {
        let arr = [2, 1, 4, 8, 7];
        let i = rng::lower_bound_by(&arr, &4, |x, y| x < y);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 8, 7];
        let i = arr.lower_bound_by(&4, |x, y| x < y);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 8, 7];
        let i = rng::lower_bound(&arr, &4);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 8, 7];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 4, 8, 7];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 2);

        let arr = [2, 1, 5, 8, 7];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 2);

        let arr = [2, 1];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 2);

        let arr = [5, 8, 7];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 0);

        let arr = [];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 0);
    }

    #[test]
    pub fn upper_bound() {
        let arr = [2, 1, 4, 8, 7];
        let i = rng::upper_bound_by(&arr, &4, |x, y| x < y);
        assert_eq!(i, 3);

        let arr = [2, 1, 4, 8, 7];
        let i = arr.upper_bound_by(&4, |x, y| x < y);
        assert_eq!(i, 3);

        let arr = [2, 1, 4, 8, 7];
        let i = rng::upper_bound(&arr, &4);
        assert_eq!(i, 3);

        let arr = [2, 1, 4, 8, 7];
        let i = arr.upper_bound(&4);
        assert_eq!(i, 3);

        let arr = [2, 1, 4, 4, 8, 7];
        let i = arr.upper_bound(&4);
        assert_eq!(i, 4);

        let arr = [2, 1, 5, 8, 7];
        let i = arr.upper_bound(&4);
        assert_eq!(i, 2);

        let arr = [2, 1];
        let i = arr.upper_bound(&4);
        assert_eq!(i, 2);

        let arr = [5, 8, 7];
        let i = arr.upper_bound(&4);
        assert_eq!(i, 0);

        let arr = [];
        let i = arr.upper_bound(&4);
        assert_eq!(i, 0);
    }

    #[test]
    pub fn equal_range() {
        let arr = [2, 1, 4, 4, 8, 7];

        let (i, j) = rng::equal_range_by(&arr, &4, |x, y| x < y);
        assert_eq!(i, 2);
        assert_eq!(j, 4);

        let (i, j) = arr.equal_range_by(&4, |x, y| x < y);
        assert_eq!(i, 2);
        assert_eq!(j, 4);

        let (i, j) = rng::equal_range(&arr, &4);
        assert_eq!(i, 2);
        assert_eq!(j, 4);

        let (i, j) = arr.equal_range(&4);
        assert_eq!(i, 2);
        assert_eq!(j, 4);

        let arr = [2, 1, 4, 8, 7];
        let (i, j) = arr.equal_range(&4);
        assert_eq!(i, 2);
        assert_eq!(j, 3);

        let arr = [2, 1, 8, 7];
        let (i, j) = arr.equal_range(&4);
        assert_eq!(i, 2);
        assert_eq!(j, 2);

        let arr = [2, 1, 4, 4];
        let (i, j) = arr.equal_range(&4);
        assert_eq!(i, 2);
        assert_eq!(j, 4);

        let arr = [2, 1];
        let (i, j) = arr.equal_range(&4);
        assert_eq!(i, 2);
        assert_eq!(j, 2);

        let arr = [8, 7];
        let (i, j) = arr.equal_range(&4);
        assert_eq!(i, 0);
        assert_eq!(j, 0);

        let arr = [];
        let (i, j) = arr.equal_range(&4);
        assert_eq!(i, 0);
        assert_eq!(j, 0);
    }

    #[test]
    pub fn binary_search() {
        let arr = [2, 1, 4, 4, 8, 7];

        assert!(rng::binary_search_by(&arr, &4, |x, y| x < y));

        assert!(arr.binary_search_by(&4, |x, y| x < y));

        assert!(rng::binary_search(&arr, &4));

        assert!(arr.binary_search(&4));

        let arr = [2, 1, 4, 8, 7];
        assert!(arr.binary_search(&4));

        let arr = [2, 1, 8, 7];
        assert!(!arr.binary_search(&4));

        let arr = [];
        assert!(!arr.binary_search(&4));

        let arr = [(1, 3)];
        assert!(arr.binary_search_by(&(1, 2), |x, y| x.0 < y.0));
    }
}
