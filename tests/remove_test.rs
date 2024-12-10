// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn remove_if() {
        let mut arr = [1, 2, 3, 4];
        let j = algo::remove_if(&mut arr, 0, 4, |x| x % 2 == 1);
        assert_eq!(j, 2);
        assert!(arr[0..j].equals(&[2, 4]));

        let mut arr = [1, 2, 3, 4];
        let j = rng::remove_if(&mut arr, |x| x % 2 == 1);
        assert_eq!(j, 2);
        assert!(arr[0..j].equals(&[2, 4]));

        let mut arr = vec![1, 2, 3, 4];
        let j = arr.remove_if(|x| x % 2 == 1);
        assert_eq!(j, 2);
        assert!(arr[0..j].equals(&[2, 4]));
    }

    #[test]
    fn remove() {
        let mut arr = [1, 2, 1, 4];
        let j = algo::remove(&mut arr, 0, 4, &1);
        assert_eq!(j, 2);
        assert!(arr[0..j].equals(&[2, 4]));

        let mut arr = [1, 2, 1, 4];
        let j = rng::remove(&mut arr, &1);
        assert_eq!(j, 2);
        assert!(arr[0..j].equals(&[2, 4]));

        let mut arr = [1, 2, 1, 4];
        let j = arr.remove(&1);
        assert_eq!(j, 2);
        assert!(arr[0..j].equals(&[2, 4]));
    }

    #[test]
    fn remove_if_copy() {
        let arr = [1, 2, 3, 4];

        let mut dest = [0, 0, 0];
        let j = algo::remove_copy_if(&arr, 0, 4, &mut dest, 1, |x| x % 2 == 1);
        assert_eq!(j, 3);
        assert!(dest.equals(&[0, 2, 4]));

        let mut dest = [0, 0, 0];
        let j = rng::remove_copy_if(&arr, &mut dest[1..], |x| x % 2 == 1);
        assert_eq!(j, 2);
        assert!(dest.equals(&[0, 2, 4]));
    }

    #[test]
    fn remove_copy() {
        let arr = [1, 2, 1, 4];

        let mut dest = [0, 0, 0];
        let j = algo::remove_copy(&arr, 0, 4, &mut dest, 1, &1);
        assert_eq!(j, 3);
        assert!(dest.equals(&[0, 2, 4]));

        let mut dest = [0, 0, 0];
        let j = rng::remove_copy(&arr, &mut dest[1..], &1);
        assert_eq!(j, 2);
        assert!(dest.equals(&[0, 2, 4]));
    }
}
