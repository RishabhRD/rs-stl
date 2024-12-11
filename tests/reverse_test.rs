// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn reverse() {
        let mut arr = [0, 1, 2];
        algo::reverse(&mut arr, 0, 2);
        assert!(arr.equals(&[1, 0, 2]));

        let mut arr = [0, 1, 2];
        rng::reverse(&mut arr);
        assert!(arr.equals(&[2, 1, 0]));

        let mut arr = [0, 1, 2];
        arr.reverse();
        assert!(arr.equals(&[2, 1, 0]));

        let mut arr = [0, 1, 2];
        algo::reverse(&mut arr, 0, 0);
        assert!(arr.equals(&[0, 1, 2]));
    }

    #[test]
    fn reverse_copy() {
        let arr = [0, 1, 2];

        let mut dest = [0, 0];
        let i = algo::reverse_copy(&arr, 0, 2, &mut dest, 0);
        assert_eq!(i, 2);
        assert!(dest.equals(&[1, 0]));

        let mut dest = [0, 0];
        let i = algo::reverse_copy(&arr, 0, 0, &mut dest, 0);
        assert_eq!(i, 0);
        assert!(dest.equals(&[0, 0]));

        let mut dest = [0, 0, 0];
        let i = rng::reverse_copy(&arr, &mut dest);
        assert_eq!(i, 3);
        assert!(dest.equals(&[2, 1, 0]));
    }
}
