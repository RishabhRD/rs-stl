// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    pub fn copy_if_to_end() {
        let vec1 = vec![1, 2, 3, 4];
        let mut vec2 = vec![0, 0, 0];

        let j = rng::copy_if(&vec1, &mut vec2, |x| x % 2 == 0);
        assert!(vec2.equals(&vec![2, 4, 0]));
        assert_eq!(j, 2);
    }

    #[test]
    pub fn copy_to_end() {
        let vec1 = vec![1, 2, 3, 4];
        let mut vec2 = vec![0, 0, 0, 0, 0];

        let j = rng::copy(&vec1, &mut vec2);
        assert!(vec2.equals(&vec![1, 2, 3, 4, 0]));
        assert_eq!(j, 4);
    }

    #[test]
    pub fn copy_if_empty_range() {
        let vec1 = vec![];
        let mut vec2 = vec![0, 0, 0];

        let j = rng::copy_if(&vec1, &mut vec2, |x| x % 2 == 0);
        assert!(vec2.equals(&vec![0, 0, 0]));
        assert_eq!(j, 0);
    }

    #[test]
    pub fn copy_empty_range() {
        let vec1 = vec![];
        let mut vec2 = vec![0, 0, 0, 0, 0];

        let j = rng::copy(&vec1, &mut vec2);
        assert!(vec2.equals(&vec![0, 0, 0, 0, 0]));
        assert_eq!(j, 0);
    }

    #[test]
    pub fn copy_if_with_overflow() {
        let vec1 = [1, 2, 3, 4, 5, 6, 7, 8];

        let mut vec2 = vec![0, 0, 0];
        let j = vec1.copy_if(&mut vec2, |x| x % 2 == 0);
        assert!(vec2.equals(&vec![2, 4, 6]));
        assert_eq!(j, 3);

        let mut vec2 = vec![0, 0, 0];
        let j = rng::copy_if(&vec1, &mut vec2, |x| x % 2 == 0);
        assert!(vec2.equals(&vec![2, 4, 6]));
        assert_eq!(j, 3);
    }

    #[test]
    pub fn copy_with_overflow() {
        let vec1 = [1, 2, 3];

        let mut vec2 = vec![0, 0];
        let j = vec1.copy(&mut vec2);
        assert!(vec2.equals(&vec![1, 2]));
        assert_eq!(j, 2);

        let mut vec2 = vec![0, 0];
        let j = rng::copy(&vec1, &mut vec2);
        assert!(vec2.equals(&vec![1, 2]));
        assert_eq!(j, 2);
    }
}
