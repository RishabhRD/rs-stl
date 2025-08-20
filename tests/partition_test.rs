// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn partition_point() {
        let arr = [1, 3, 5, 2, 4];
        let i = arr.partition_point(|x| x % 2 == 0);
        assert_eq!(i, 3);

        let arr = [];
        let i = arr.partition_point(|x| x % 2 == 0);
        assert_eq!(i, 0);

        let arr = [1, 3, 5];
        let i = arr.partition_point(|x| x % 2 == 0);
        assert_eq!(i, 3);

        let arr = [2, 4];
        let i = arr.partition_point(|x| x % 2 == 0);
        assert_eq!(i, 0);
    }

    #[test]
    fn partition_when_both_parts_are_not_empty() {
        let mut arr = [1, 2, 3, 4, 5];
        let i = arr.partition(|x| x % 2 == 1);
        assert_eq!(i, 2);
        assert!(arr.prefix_upto(i).all_satisfy(|x| x % 2 == 0));
        assert!(arr.suffix_from(i).all_satisfy(|x| x % 2 == 1));
    }

    #[test]
    fn partition_when_first_part_is_empty() {
        let mut arr = [1, 3, 5];
        let i = arr.partition(|x| x % 2 == 1);
        assert_eq!(i, 0);
        assert!(arr.prefix_upto(i).all_satisfy(|x| x % 2 == 0));
        assert!(arr.suffix_from(i).all_satisfy(|x| x % 2 == 1));
    }

    #[test]
    fn partition_when_second_part_is_empty() {
        let mut arr = [2, 4, 6];
        let i = arr.partition(|x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(arr.prefix_upto(i).all_satisfy(|x| x % 2 == 0));
        assert!(arr.suffix_from(i).all_satisfy(|x| x % 2 == 1));
    }

    #[test]
    fn stable_partition_when_both_parts_are_not_empty() {
        let mut arr = [1, 2, 3, 4, 5];
        let i = arr.stable_partition(|x| x % 2 == 1);
        assert_eq!(i, 2);
        assert!(arr.equals(&[2, 4, 1, 3, 5]));
    }

    #[test]
    fn stable_partition_when_first_part_is_empty() {
        let mut arr = [1, 3, 5];
        let i = arr.stable_partition(|x| x % 2 == 1);
        assert_eq!(i, 0);
        assert!(arr.equals(&[1, 3, 5]));
    }

    #[test]
    fn stable_partition_when_second_part_is_empty() {
        let mut arr = [2, 4, 6];
        let i = arr.stable_partition(|x| x % 2 == 1);
        assert_eq!(i, 3);
        assert!(arr.equals(&[2, 4, 6]));
    }
}
