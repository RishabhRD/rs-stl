// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn is_partitioned() {
        let arr = [1, 3, 5, 2, 4];
        assert!(algo::is_partitioned(&arr, arr.start(), arr.end(), |x| x
            % 2
            == 1));
        assert!(rng::is_partitioned(&arr, |x| x % 2 == 1));
        assert!(arr.is_partitioned(|x| x % 2 == 1));

        let arr = [2, 4];
        assert!(arr.is_partitioned(|x| x % 2 == 1));

        let arr = [1, 3];
        assert!(arr.is_partitioned(|x| x % 2 == 1));

        let arr = [];
        assert!(arr.is_partitioned(|x| x % 2 == 1));

        let arr = [1, 2, 3];
        assert!(!(arr.is_partitioned(|x| x % 2 == 1)));
    }

    #[test]
    fn partiton() {
        let mut arr = [1, 3, 2, 5, 4];
        let start = arr.start();
        let end = arr.end();
        let i = algo::partition(&mut arr, start, end, |x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(arr[0..i].all_of(|x| x % 2 == 1));
        assert!(arr[i..].all_of(|x| x % 2 == 0));

        let mut arr = [1, 3, 2, 5, 4];
        let i = rng::partition(&mut arr, |x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(arr[0..i].all_of(|x| x % 2 == 1));
        assert!(arr[i..].all_of(|x| x % 2 == 0));

        let mut arr = [1, 3, 2, 5, 4];
        let i = arr.partition(|x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(arr[0..i].all_of(|x| x % 2 == 1));
        assert!(arr[i..].all_of(|x| x % 2 == 0));

        let mut arr = [1, 3, 5];
        let i = arr.partition(|x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(arr[0..i].all_of(|x| x % 2 == 1));
        assert!(arr[i..].all_of(|x| x % 2 == 0));

        let mut arr = [2, 4];
        let i = arr.partition(|x| x % 2 == 1);
        assert_eq!(i, 0);
        assert!(arr[0..i].all_of(|x| x % 2 == 1));
        assert!(arr[i..].all_of(|x| x % 2 == 0));

        let mut arr = [];
        let i = arr.partition(|x| x % 2 == 1);
        assert_eq!(i, 0);
        assert!(arr[0..i].all_of(|x| x % 2 == 1));
        assert!(arr[i..].all_of(|x| x % 2 == 0));
    }

    #[test]
    fn stable_partition() {
        let mut arr = [1, 3, 2, 5, 4];
        let start = arr.start();
        let end = arr.end();
        let i = algo::stable_partition(&mut arr, start, end, |x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(&arr[0..i].equals(&[1, 3, 5]));
        assert!(&arr[i..].equals(&[2, 4]));

        let mut arr = [1, 3, 2, 5, 4];
        let i = rng::stable_partition(&mut arr, |x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(&arr[0..i].equals(&[1, 3, 5]));
        assert!(&arr[i..].equals(&[2, 4]));

        let mut arr = [1, 3, 2, 5, 4];
        let i = arr.stable_partition(|x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(&arr[0..i].equals(&[1, 3, 5]));
        assert!(&arr[i..].equals(&[2, 4]));

        let mut arr: [i32; 0] = [];
        let i = arr.stable_partition(|x| x % 2 == 1);
        assert_eq!(i, 0);
        assert!(&arr[0..i].equals(&[]));
        assert!(&arr[i..].equals(&[]));

        let mut arr = [1, 3, 5];
        let i = arr.stable_partition(|x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(&arr[0..i].equals(&[1, 3, 5]));
        assert!(&arr[i..].equals(&[]));

        let mut arr = [2, 4];
        let i = arr.stable_partition(|x| x % 2 == 1);
        assert_eq!(i, 0);
        assert!(&arr[0..i].equals(&[]));
        assert!(&arr[i..].equals(&[2, 4]));
    }
}
