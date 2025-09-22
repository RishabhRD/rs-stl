// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn first_position_where() {
        let arr = [1, 2, 3, 4];
        let i = arr.first_position_where(|x| x % 2 == 1);
        assert_eq!(i, 0);

        let arr = [2, 4, 6];
        let i = arr.first_position_where(|x| x % 2 == 1);
        assert_eq!(i, 3);

        let arr = [];
        let i = arr.first_position_where(|x| x % 2 == 1);
        assert_eq!(i, 0);
    }

    #[test]
    fn first_position_of() {
        let arr = [1, 3, 3, 4];
        let i = arr.first_position_of(&3);
        assert_eq!(i, 1);

        let arr = [2, 4, 6];
        let i = arr.first_position_of(&3);
        assert_eq!(i, 3);

        let arr = [];
        let i = arr.first_position_of(&3);
        assert_eq!(i, 0);
    }

    #[test]
    fn last_position_where() {
        let arr = [1, 2, 3, 4];
        let i = arr.last_position_where(|x| x % 2 == 1);
        assert_eq!(i, 2);

        let arr = [2, 4, 6];
        let i = arr.last_position_where(|x| x % 2 == 1);
        assert_eq!(i, 3);

        let arr = [];
        let i = arr.last_position_where(|x| x % 2 == 1);
        assert_eq!(i, 0);
    }

    #[test]
    fn last_position_of() {
        let arr = [1, 3, 3, 4];
        let i = arr.last_position_of(&3);
        assert_eq!(i, 2);

        let arr = [2, 4, 6];
        let i = arr.last_position_of(&3);
        assert_eq!(i, 3);

        let arr = [];
        let i = arr.last_position_of(&3);
        assert_eq!(i, 0);
    }

    #[test]
    fn parallel_first_position_where() {
        let arr = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5];
        let i = arr.parallel_first_position_where(|x| *x == 5);
        assert_eq!(i, 0);

        let arr = [0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5];
        let i = arr.parallel_first_position_where(|x| *x == 5);
        assert_eq!(i, 6);

        let arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let i = arr.parallel_first_position_where(|x| *x == 5);
        assert_eq!(i, 11);
    }
}
