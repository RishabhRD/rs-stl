// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use algo::infix::*;
    use stl::*;
    #[test]
    fn find_if_when_element_is_present() {
        let arr = [1, 2, 3];
        assert_eq!(algo::find_if(&arr, |x| x % 2 == 0), 1);
        assert_eq!(arr.find_if(|x| x % 2 == 0), 1);
    }

    #[test]
    fn find_if_when_element_is_not_present() {
        let arr = [1, 3, 5];
        let i = arr.find_if(|x| x % 2 == 0);

        assert_eq!(i, 3);
    }

    #[test]
    fn find_if_when_range_is_empty() {
        let arr = [];
        let i = arr.find_if(|x| x % 2 == 0);
        assert_eq!(i, 0);
    }

    #[test]
    fn find_if_not_when_element_is_present() {
        let arr = [2, 3, 4];
        let i = arr.find_if_not(|x| x % 2 == 0);
        assert_eq!(i, 1);
    }

    #[test]
    fn find_if_not_when_element_is_not_present() {
        let arr = [2, 4, 6];
        let i = arr.find_if_not(|x| x % 2 == 0);
        assert_eq!(i, 3);
    }

    #[test]
    fn find_if_not_when_range_is_empty() {
        let arr = [];
        let i = arr.find_if_not(|x| x % 2 == 0);
        assert_eq!(i, 0);
    }

    #[test]
    fn find_when_element_is_present() {
        let arr = [2, 3, 4];
        let i = arr.find(&3);
        assert_eq!(i, 1);
    }

    #[test]
    fn find_when_element_is_not_present() {
        let arr = [2, 4, 6];
        let i = arr.find(&3);
        assert_eq!(i, 3);
    }

    #[test]
    fn find_when_range_is_empty() {
        let arr = [];
        let i = arr.find(&3);
        assert_eq!(i, 0);
    }
}
