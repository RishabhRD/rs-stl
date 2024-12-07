// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::{algo, rng, InputRange};
    #[test]
    fn mismatch_unbounded_by_in_mid() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![2, 3, 5, 6];
        let (i, j) = algo::mismatch_unbounded_by(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            |x, y| *y == *x + 1,
        );
        assert_eq!(i, 2);
        assert_eq!(j, 2);
    }

    #[test]
    fn mismatch_unbounded_by_in_end() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![2, 3, 4, 5];
        let (i, j) = algo::mismatch_unbounded_by(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            |x, y| *y == *x + 1,
        );
        assert_eq!(i, 4);
        assert_eq!(j, 4);
    }

    #[test]
    fn mismatch_unbounded_in_mid() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![1, 2, 4, 5];
        let (i, j) = algo::mismatch_unbounded(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
        );
        assert_eq!(i, 2);
        assert_eq!(j, 2);
    }

    #[test]
    fn mismatch_unbounded_in_end() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![1, 2, 3, 4];
        let (i, j) = algo::mismatch_unbounded(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
        );
        assert_eq!(i, 4);
        assert_eq!(j, 4);
    }

    #[test]
    fn mismatch_by_in_mid() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![2, 3, 5, 6];
        let (i, j) = algo::mismatch_by(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            v2.end(),
            |x, y| *y == *x + 1,
        );
        assert_eq!(i, 2);
        assert_eq!(j, 2);
        let (i, j) = rng::mismatch_by(&v1, &v2, |x, y| *y == *x + 1);
        assert_eq!(i, 2);
        assert_eq!(j, 2);
    }

    #[test]
    fn mismatch_by_in_both_end() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![2, 3, 4, 5];
        let (i, j) = algo::mismatch_by(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            v2.end(),
            |x, y| *y == *x + 1,
        );
        assert_eq!(i, 4);
        assert_eq!(j, 4);
        let (i, j) = rng::mismatch_by(&v1, &v2, |x, y| *y == *x + 1);
        assert_eq!(i, 4);
        assert_eq!(j, 4);
    }

    #[test]
    fn mismatch_by_in_first_end() {
        let v1 = vec![1, 2];
        let v2 = vec![2, 3, 4, 5];
        let (i, j) = algo::mismatch_by(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            v2.end(),
            |x, y| *y == *x + 1,
        );
        assert_eq!(i, 2);
        assert_eq!(j, 2);
        let (i, j) = rng::mismatch_by(&v1, &v2, |x, y| *y == *x + 1);
        assert_eq!(i, 2);
        assert_eq!(j, 2);
    }

    #[test]
    fn mismatch_by_in_second_end() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![2, 3];
        let (i, j) = algo::mismatch_by(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            v2.end(),
            |x, y| *y == *x + 1,
        );
        assert_eq!(i, 2);
        assert_eq!(j, 2);
        let (i, j) = rng::mismatch_by(&v1, &v2, |x, y| *y == *x + 1);
        assert_eq!(i, 2);
        assert_eq!(j, 2);
    }

    #[test]
    fn mismatch_in_mid() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![1, 2, 4, 5];
        let (i, j) = algo::mismatch(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            v2.end(),
        );
        assert_eq!(i, 2);
        assert_eq!(j, 2);
        let (i, j) = rng::mismatch(&v1, &v2);
        assert_eq!(i, 2);
        assert_eq!(j, 2);
    }

    #[test]
    fn mismatch_in_both_end() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![1, 2, 3, 4];
        let (i, j) = algo::mismatch(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            v2.end(),
        );
        assert_eq!(i, 4);
        assert_eq!(j, 4);
        let (i, j) = rng::mismatch(&v1, &v2);
        assert_eq!(i, 4);
        assert_eq!(j, 4);
    }

    #[test]
    fn mismatch_in_first_end() {
        let v1 = vec![1, 2];
        let v2 = vec![1, 2, 3, 4];
        let (i, j) = algo::mismatch(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            v2.end(),
        );
        assert_eq!(i, 2);
        assert_eq!(j, 2);
        let (i, j) = rng::mismatch(&v1, &v2);
        assert_eq!(i, 2);
        assert_eq!(j, 2);
    }

    #[test]
    fn mismatch_in_second_end() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![1, 2];
        let (i, j) = algo::mismatch(
            &v1,
            v1.start(),
            v1.end(),
            &v2,
            v2.start(),
            v2.end(),
        );
        assert_eq!(i, 2);
        assert_eq!(j, 2);
        let (i, j) = rng::mismatch(&v1, &v2);
        assert_eq!(i, 2);
        assert_eq!(j, 2);
    }
}
