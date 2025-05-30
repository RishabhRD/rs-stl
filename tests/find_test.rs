// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn find_if_element_found() {
        let arr = [1, 2, 3, 4];
        let i = arr.first_position_where(|x| *x == 2);
        assert_eq!(i, 1);

        let arr = ["Hello".to_string(), "Test".to_string()];
        let to_find = "Test".to_string();
        let i = arr.first_position_where(|x| *x == to_find);
        assert_eq!(i, 1);
    }

    #[test]
    fn find_if_element_not_found() {
        let arr = [1, 3, 4];
        let i = arr.first_position_where(|x| *x == 2);
        assert_eq!(i, 3);
    }

    #[test]
    fn find_if_empty() {
        let arr: [i32; 0] = [];
        let i = arr.first_position_where(|x| *x == 2);
        assert_eq!(i, 0);
    }

    #[test]
    fn find_element() {
        let arr = [1, 2, 3];
        assert_eq!(arr.first_position_of(&2), 1);
    }
}
