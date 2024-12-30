// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    fn assert_equals_prefix_by<const N: usize>(
        arr1: [i32; N],
        arr2: [i32; N],
        expected: bool,
    ) {
        let res = algo::equals_prefix_by(
            &arr1,
            arr1.start(),
            arr1.end(),
            &arr2,
            arr2.start(),
            arr2.end(),
            |x, y| x == y,
        );
        assert_eq!(res, expected);

        let res = rng::equals_prefix_by(&arr1, &arr2, |x, y| x == y);
        assert_eq!(res, expected);

        let res = arr1.equals_prefix_by(&arr2, |x, y| x == y);
        assert_eq!(res, expected);
    }

    fn assert_equals_prefix<const N: usize>(
        arr1: [i32; N],
        arr2: [i32; N],
        expected: bool,
    ) {
        let res = algo::equals_prefix(
            &arr1,
            arr1.start(),
            arr1.end(),
            &arr2,
            arr2.start(),
            arr2.end(),
        );
        assert_eq!(res, expected);

        let res = rng::equals_prefix(&arr1, &arr2);
        assert_eq!(res, expected);

        let res = arr1.equals_prefix(&arr2);
        assert_eq!(res, expected);
    }

    fn assert_equals_by<const N: usize>(
        arr1: [i32; N],
        arr2: [i32; N],
        expected: bool,
    ) {
        let res = algo::equals_by(
            &arr1,
            arr1.start(),
            arr1.end(),
            &arr2,
            arr2.start(),
            arr2.end(),
            |x, y| x == y,
        );
        assert_eq!(res, expected);

        let res = rng::equals_by(&arr1, &arr2, |x, y| x == y);
        assert_eq!(res, expected);

        let res = arr1.equals_by(&arr2, |x, y| x == y);
        assert_eq!(res, expected);
    }

    fn assert_equals<const N: usize>(
        arr1: [i32; N],
        arr2: [i32; N],
        expected: bool,
    ) {
        let res = algo::equals(
            &arr1,
            arr1.start(),
            arr1.end(),
            &arr2,
            arr2.start(),
            arr2.end(),
        );
        assert_eq!(res, expected);

        let res = rng::equals(&arr1, &arr2);
        assert_eq!(res, expected);

        let res = arr1.equals(&arr2);
        assert_eq!(res, expected);
    }

    #[test]
    fn equals_prefix_by() {
        let arr1 = [1, 2, 3, 4];
        let arr2 = [1, 2, 3, 4];
        assert_equals_prefix_by(arr1, arr2, true);

        let arr2 = [1, 2, 3];
        assert_equals_prefix_by(arr1, arr2, false);

        let arr1 = [1, 2];
        assert_equals_prefix_by(arr1, arr2, true);

        let arr2 = [2, 2];
        assert_equals_prefix_by(arr1, arr2, false);

        let arr1 = [];
        assert_equals_prefix_by(arr1, arr2, true);

        let arr2 = [];
        assert_equals_prefix_by(arr1, arr2, true);
    }

    #[test]
    fn equals_prefix() {
        let arr1 = [1, 2, 3, 4];
        let arr2 = [1, 2, 3, 4];
        assert_equals_prefix(arr1, arr2, true);

        let arr2 = [1, 2, 3];
        assert_equals_prefix(arr1, arr2, false);

        let arr1 = [1, 2];
        assert_equals_prefix(arr1, arr2, true);

        let arr2 = [2, 2];
        assert_equals_prefix(arr1, arr2, false);

        let arr1 = [];
        assert_equals_prefix(arr1, arr2, true);

        let arr2 = [];
        assert_equals_prefix(arr1, arr2, true);
    }

    #[test]
    fn equals_by() {
        let arr1 = [1, 2, 3, 4];
        let arr2 = [1, 2, 3, 4];
        assert_equals_by(arr1, arr2, true);

        let arr2 = [2, 2, 3, 4];
        assert_equals_by(arr1, arr2, false);

        let arr2 = [1, 2];
        assert_equals_by(arr1, arr2, false);

        let arr1 = [1];
        assert_equals_by(arr1, arr2, false);
        assert_equals_by([], [], true);
    }

    #[test]
    fn equals() {
        let arr1 = [1, 2, 3, 4];
        let arr2 = [1, 2, 3, 4];
        assert_equals(arr1, arr2, true);

        let arr2 = [2, 2, 3, 4];
        assert_equals(arr1, arr2, false);

        let arr2 = [1, 2];
        assert_equals(arr1, arr2, false);

        let arr1 = [1];
        assert_equals(arr1, arr2, false);
        assert_equals([], [], true);
    }
}
