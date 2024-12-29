// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    pub fn adjacent_find_with_element_found() {
        let vec = vec![1, 2, 2, 3];
        let i = rng::adjacent_find_if(&vec, |x, y| x == y);
        assert_eq!(i, 1);
        let i = vec.adjacent_find_if(|x, y| x == y);
        assert_eq!(i, 1);
    }

    #[test]
    pub fn adjacent_find_with_element_not_found() {
        let vec = vec![1, 2, 3, 4];
        let i = rng::adjacent_find_if(&vec, |x, y| x == y);
        assert_eq!(i, 4);
        let i = vec.adjacent_find_if(|x, y| x == y);
        assert_eq!(i, 4);
    }

    #[test]
    pub fn adjacent_find_with_singleton_list() {
        let vec = vec![1];
        let i = rng::adjacent_find_if(&vec, |x, y| x == y);
        assert_eq!(i, 1);
        let i = vec.adjacent_find_if(|x, y| x == y);
        assert_eq!(i, 1);
    }

    #[test]
    pub fn adjacent_find_with_empty_list() {
        let vec: Vec<u32> = vec![];
        let i = rng::adjacent_find_if(&vec, |x, y| x == y);
        assert_eq!(i, 0);
        let i = vec.adjacent_find_if(|x, y| x == y);
        assert_eq!(i, 0);
    }
}
