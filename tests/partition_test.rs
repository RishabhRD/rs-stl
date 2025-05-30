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
}
