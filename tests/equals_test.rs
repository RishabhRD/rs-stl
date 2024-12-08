// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn algo_equals_unbounded_by_true() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![3, 4, 6];

        let res = algo::equals_unbounded_by(&vec1, 1, 2, &vec2, 1, |x, y| {
            *y == *x + 1
        });
        assert!(res);
    }

    #[test]
    fn equals_unbounded_by_true() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![2, 4, 6, 8];

        let res = algo::equals_unbounded_by(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
            |x, y| *y == *x + 1,
        );
        assert!(res);

        let res =
            rng::equals_unbounded_by(&vec1, &vec2, vec2.start(), |x, y| {
                *y == *x + 1
            });
        assert!(res);
    }

    #[test]
    fn equals_unbounded_by_false() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![2, 4, 7, 8];

        let res = algo::equals_unbounded_by(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
            |x, y| *y == *x + 1,
        );
        assert!(!res);

        let res =
            rng::equals_unbounded_by(&vec1, &vec2, vec2.start(), |x, y| {
                *y == *x + 1
            });
        assert!(!res);
    }

    #[test]
    fn algo_equals_unbounded_true() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![3, 3, 5];

        let res = algo::equals_unbounded(&vec1, 1, 2, &vec2, 1);
        assert!(res);
    }

    #[test]
    fn equals_unbounded_true() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![1, 3, 5, 7];

        let res = algo::equals_unbounded(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
        );
        assert!(res);

        let res = rng::equals_unbounded(&vec1, &vec2, vec2.start());
        assert!(res);
    }

    #[test]
    fn equals_unbounded_false() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![1, 3, 4, 7];

        let res = algo::equals_unbounded(
            &vec1,
            vec1.start(),
            vec1.end(),
            &vec2,
            vec2.start(),
        );
        assert!(!res);

        let res = rng::equals_unbounded(&vec1, &vec2, vec2.start());
        assert!(!res);
    }

    #[test]
    fn algo_equals_by_true() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![3, 4, 6];

        let res =
            algo::equals_by(&vec1, 1, 2, &vec2, 1, 2, |x, y| *y == *x + 1);
        assert!(res);
    }

    #[test]
    fn algo_equals_by_false() {
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![3, 4, 6];

        let res =
            algo::equals_by(&vec1, 1, 2, &vec2, 1, 1, |x, y| *y == *x + 1);
        assert!(!res);
    }

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
}
