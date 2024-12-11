// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn unique_by() {
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
}
