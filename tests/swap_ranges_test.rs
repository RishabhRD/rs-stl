// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn swap_ranges() {
        let mut arr1 = [1, 2];
        let mut arr2 = [3, 4, 5];
        let start1 = arr1.start();
        let end1 = arr1.end();
        let start2 = arr2.start();
        let end2 = arr2.end();
        let (i, j) =
            algo::swap_ranges(&mut arr1, start1, end1, &mut arr2, start2, end2);
        assert_eq!(i, 2);
        assert_eq!(j, 2);
        assert!(arr1.equals(&[3, 4]));
        assert!(arr2.equals(&[1, 2, 5]));

        let mut arr1 = [3, 4, 5];
        let mut arr2 = [1, 2];
        let (i, j) = rng::swap_ranges(&mut arr1, &mut arr2);
        assert_eq!(i, 2);
        assert_eq!(j, 2);
        assert!(arr1.equals(&[1, 2, 5]));
        assert!(arr2.equals(&[3, 4]));

        let mut arr1 = [];
        let mut arr2 = [1, 2];
        let (i, j) = rng::swap_ranges(&mut arr1, &mut arr2);
        assert_eq!(i, 0);
        assert_eq!(j, 0);
        assert!(arr1.equals(&[]));
        assert!(arr2.equals(&[1, 2]));

        let mut arr1 = [1, 2];
        let mut arr2 = [];
        let (i, j) = rng::swap_ranges(&mut arr1, &mut arr2);
        assert_eq!(i, 0);
        assert_eq!(j, 0);
        assert!(arr1.equals(&[1, 2]));
        assert!(arr2.equals(&[]));

        let mut arr1: [i32; 0] = [];
        let mut arr2 = [];
        let (i, j) = rng::swap_ranges(&mut arr1, &mut arr2);
        assert_eq!(i, 0);
        assert_eq!(j, 0);
        assert!(arr1.equals(&[]));
        assert!(arr2.equals(&[]));
    }
}
