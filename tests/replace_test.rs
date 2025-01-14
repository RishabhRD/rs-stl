// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    pub fn replace_if() {
        let mut arr = [1, 2, 3];
        let start = arr.start();
        let end = arr.end();
        algo::replace_if(&mut arr, start, end, |x| x % 2 == 1, &2);
        assert!(arr.equals(&vec![2, 2, 2]));

        let mut arr = [1, 2, 3];
        rng::replace_if(&mut arr, |x| x % 2 == 1, &2);
        assert!(arr.equals(&vec![2, 2, 2]));

        let mut arr = [1, 2, 3];
        arr.replace_if(|x| x % 2 == 1, &2);
        assert!(arr.equals(&vec![2, 2, 2]));
    }

    #[test]
    pub fn replace() {
        let mut arr = [1, 2, 1];
        let start = arr.start();
        let end = arr.end();
        algo::replace(&mut arr, start, end, &1, &3);
        assert!(arr.equals(&vec![3, 2, 3]));

        let mut arr = [1, 2, 1];
        rng::replace(&mut arr, &1, &3);
        assert!(arr.equals(&vec![3, 2, 3]));

        let mut arr = [1, 2, 1];
        arr[..].replace(&1, &3);
        assert!(arr.equals(&vec![3, 2, 3]));
    }

    #[test]
    pub fn replace_copy_if() {
        let src = [1, 2, 3];
        let mut dest = [0, 0, 0, 0];
        algo::replace_copy_if(
            &src,
            src.start(),
            src.end(),
            &mut dest,
            1,
            |x| x % 2 == 1,
            &2,
        );
        assert!(dest.equals(&[0, 2, 2, 2]));

        let mut dest = [0, 0, 0, 0];
        rng::replace_copy_if(&src, &mut dest[1..], |x| x % 2 == 1, &2);
        assert!(dest.equals(&[0, 2, 2, 2]));
    }

    #[test]
    pub fn replace_copy() {
        let src = [1, 2, 1];
        let mut dest = [0, 0, 0, 0];
        algo::replace_copy(&src, src.start(), src.end(), &mut dest, 1, &1, &3);
        assert!(dest.equals(&[0, 3, 2, 3]));

        let mut dest = [0, 0, 0, 0];
        rng::replace_copy(&src, &mut dest[1..], &1, &3);
        assert!(dest.equals(&[0, 3, 2, 3]));
    }
}
