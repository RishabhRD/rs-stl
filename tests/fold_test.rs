// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn fold_left() {
        let arr = [1, 2, 3];
        assert_eq!(arr.fold_left(0, |x, y| x + y), 6);
        assert_eq!(arr.prefix(0).fold_left(1, |x, y| x + y), 1);

        let arr = [1];
        assert_eq!(arr.fold_left(0, |x, y| x - y), -1);
    }

    #[test]
    fn lazy_fold_left() {
        let arr = 1..=3;
        assert_eq!(arr.lazy_fold_left(0, |x, y| x + y), 6);
        assert_eq!(arr.slice(1, 1).lazy_fold_left(1, |x, y| x + y), 1);

        let arr = 1..=1;
        assert_eq!(arr.lazy_fold_left(0, |x, y| x - y), -1);
    }

    #[test]
    fn fold_right() {
        let arr = [1, 2, 3];
        assert_eq!(arr.fold_right(0, |x, y| x + y), 6);
        assert_eq!(arr.prefix(0).fold_right(1, |x, y| x + y), 1);

        let arr = [1];
        assert_eq!(arr.fold_right(0, |x, y| x - y), 1);
    }

    #[test]
    fn lazy_fold_right() {
        let arr = 1..4;
        assert_eq!(arr.lazy_fold_right(0, |x, y| x + y), 6);
        assert_eq!(arr.prefix(0).lazy_fold_right(1, |x, y| x + y), 1);

        let arr = 1..2;
        assert_eq!(arr.lazy_fold_right(0, |x, y| x - y), 1);
    }
}
