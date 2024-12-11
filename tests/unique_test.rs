// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn unique_by() {
        let mut arr: Vec<(i32, i32)> = vec![];
        let i = algo::unique_by(&mut arr, 0, 0, |x, y| x.0 == y.0);
        assert_eq!(i, 0);
        assert!(arr.equals(&[]));

        let mut arr = [(1, 2), (1, 3), (2, 3)];
        let i = algo::unique_by(&mut arr, 0, 3, |x, y| x.0 == y.0);
        assert_eq!(i, 2);
        assert!(arr[..i].equals(&[(1, 2), (2, 3)]));

        let mut arr = [(1, 2), (1, 3), (2, 3)];
        let i = rng::unique_by(&mut arr, |x, y| x.0 == y.0);
        assert_eq!(i, 2);
        assert!(arr[..i].equals(&[(1, 2), (2, 3)]));

        let mut arr = [(1, 2), (1, 3), (2, 3)];
        let i = arr.unique_by(|x, y| x.0 == y.0);
        assert_eq!(i, 2);
        assert!(arr[..i].equals(&[(1, 2), (2, 3)]));
    }

    #[test]
    fn unique() {
        let mut arr = [1, 1, 2];
        let i = algo::unique(&mut arr, 0, 3);
        assert_eq!(i, 2);
        assert!(arr[..i].equals(&[1, 2]));

        let mut arr = [1, 1, 2];
        let i = rng::unique(&mut arr);
        assert_eq!(i, 2);
        assert!(arr[..i].equals(&[1, 2]));

        let mut arr = [1, 1, 2];
        let i = arr.unique();
        assert_eq!(i, 2);
        assert!(arr[..i].equals(&[1, 2]));
    }

    #[test]
    fn unique_copy_by() {
        let src = [(1, 2), (1, 3), (2, 3)];

        let mut dest = [(0, 0), (0, 0)];
        let i = algo::unique_copy_by(
            &src,
            src.start(),
            src.end(),
            &mut dest,
            0,
            |x, y| x.0 == y.0,
        );
        assert_eq!(i, 2);
        assert!(dest.equals(&[(1, 2), (2, 3)]));

        let mut dest = [(0, 0), (0, 0)];
        let i = algo::unique_copy_by(
            &src,
            src.start(),
            src.start(),
            &mut dest,
            0,
            |x, y| x.0 == y.0,
        );
        assert_eq!(i, 0);
        assert!(dest.equals(&[(0, 0), (0, 0)]));

        let mut dest = [(0, 0), (0, 0)];
        let i = rng::unique_copy_by(&src, &mut dest, |x, y| x.0 == y.0);
        assert_eq!(i, 2);
        assert!(dest.equals(&[(1, 2), (2, 3)]));
    }

    #[test]
    fn unique_copy() {
        let src = [1, 1, 2];

        let mut dest = [0, 0];
        let i = algo::unique_copy(&src, src.start(), src.end(), &mut dest, 0);
        assert_eq!(i, 2);
        assert!(dest.equals(&[1, 2]));

        let mut dest = [0, 0];
        let i = rng::unique_copy(&src, &mut dest);
        assert_eq!(i, 2);
        assert!(dest.equals(&[1, 2]));

        let src = [1, 2, 1];
        let mut dest = [0, 0, 0];
        let i = rng::unique_copy(&src, &mut dest);
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, 2, 1]));
    }
}
