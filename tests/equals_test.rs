// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn equals_by_true() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![2, 4, 6, 8];

        let res = algo::equals_by(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
            vec2.end(),
            |x, y| *y == *x + 1,
        );
        assert!(res);

        let res = rng::equals_by(&vec1, &vec2, |x, y| *y == *x + 1);
        assert!(res);

        let res = vec1.equals_by(&vec2, |x, y| *y == *x + 1);
        assert!(res);
    }

    #[test]
    fn equals_by_false() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![2, 4, 7, 8];

        let res = algo::equals_by(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
            vec2.end(),
            |x, y| *y == *x + 1,
        );
        assert!(!res);

        let res = rng::equals_by(&vec1, &vec2, |x, y| *y == *x + 1);
        assert!(!res);
        let res = vec1.equals_by(&vec2, |x, y| *y == *x + 1);
        assert!(!res);
    }

    #[test]
    fn algo_equals_true() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![3, 3, 5];

        let res = algo::equals(&vec1, 1, 2, &vec2, 1, 2);
        assert!(res);
    }

    #[test]
    fn algo_equals_false() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![3, 3, 5];

        let res = algo::equals(&vec1, 1, 1, &vec2, 1, 2);
        assert!(!res);
    }

    #[test]
    fn equals_true() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![1, 3, 5, 7];

        let res = algo::equals(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
            vec2.end(),
        );
        assert!(res);

        let res = rng::equals(&vec1, &vec2);
        assert!(res);
        let res = vec1.equals(&vec2);
        assert!(res);
    }

    #[test]
    fn equals_false() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![1, 3, 4, 7];

        let res = algo::equals(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
            vec2.end(),
        );
        assert!(!res);

        let res = rng::equals(&vec1, &vec2);
        assert!(!res);
        let res = vec1.equals(&vec2);
        assert!(!res);
    }

    #[test]
    fn equals_false_different_len() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![1, 3, 5];

        let res = algo::equals(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
            vec2.end(),
        );
        assert!(!res);

        let res = rng::equals(&vec1, &vec2);
        assert!(!res);

        let vec1 = vec![1, 3, 5];
        let vec2 = vec![1, 3, 5, 7];

        let res = algo::equals(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
            vec2.end(),
        );
        assert!(!res);

        let res = rng::equals(&vec1, &vec2);
        assert!(!res);

        let res = vec1.equals(&vec2);
        assert!(!res);
        let res = vec2.equals(&vec1);
        assert!(!res);
    }

    #[test]
    fn equals_prefix_by() {
        let arr1 = [1, 3];
        let arr2 = [1, 3, 5, 7];
        assert!(rng::equals_prefix_by(&arr1, &arr2, |x, y| x == y));
        assert!(arr1.equals_prefix_by(&arr2, |x, y| x == y));

        let arr2 = [1, 3];
        assert!(rng::equals_prefix_by(&arr1, &arr2, |x, y| x == y));
        assert!(arr1.equals_prefix_by(&arr2, |x, y| x == y));

        let arr2 = [1];
        assert!(!rng::equals_prefix_by(&arr1, &arr2, |x, y| x == y));
        assert!(!arr1.equals_prefix_by(&arr2, |x, y| x == y));

        let arr2 = [];
        assert!(!rng::equals_prefix_by(&arr1, &arr2, |x, y| x == y));
        assert!(!arr1.equals_prefix_by(&arr2, |x, y| x == y));

        let arr1: [i32; 0] = [];
        assert!(rng::equals_prefix_by(&arr1, &arr2, |x, y| x == y));
        assert!(arr1.equals_prefix_by(&arr2, |x, y| x == y));

        let arr2 = [1];
        assert!(rng::equals_prefix_by(&arr1, &arr2, |x, y| x == y));
        assert!(arr1.equals_prefix_by(&arr2, |x, y| x == y));
    }

    #[test]
    fn equals_prefix() {
        let arr1 = [1, 3];
        let arr2 = [1, 3, 5, 7];
        assert!(rng::equals_prefix(&arr1, &arr2));
        assert!(arr1.equals_prefix(&arr2));

        let arr2 = [1, 3];
        assert!(rng::equals_prefix(&arr1, &arr2));
        assert!(arr1.equals_prefix(&arr2));

        let arr2 = [1];
        assert!(!rng::equals_prefix(&arr1, &arr2));
        assert!(!arr1.equals_prefix(&arr2));

        let arr2 = [];
        assert!(!rng::equals_prefix(&arr1, &arr2));
        assert!(!arr1.equals_prefix(&arr2));

        let arr1: [i32; 0] = [];
        assert!(rng::equals_prefix(&arr1, &arr2));
        assert!(arr1.equals_prefix(&arr2));

        let arr2 = [1];
        assert!(rng::equals_prefix(&arr1, &arr2));
        assert!(arr1.equals_prefix(&arr2));
    }
}
