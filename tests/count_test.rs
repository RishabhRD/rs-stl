// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn count_if_for_element_exist() {
        let vec = vec![1, 2, 3, 4, 5];
        let p = rng::count_if(&vec, |x| x % 2 == 1);
        assert_eq!(p, 3);

        let p = vec.count_if(|x| x % 2 == 1);
        assert_eq!(p, 3);
    }

    #[test]
    fn count_if_for_element_not_exist() {
        let vec = vec![2, 4, 6, 8];
        let p = rng::count_if(&vec, |x| x % 2 == 1);
        assert_eq!(p, 0);

        let p = vec.count_if(|x| x % 2 == 1);
        assert_eq!(p, 0);
    }

    #[test]
    fn count_if_for_empty_range() {
        let vec = vec![];
        let p = rng::count_if(&vec, |x| x % 2 == 1);
        assert_eq!(p, 0);

        let p = vec.count_if(|x| x % 2 == 1);
        assert_eq!(p, 0);
    }

    #[test]
    fn count_for_element_exist() {
        let vec = vec![2, 4, 2, 8];
        let p = rng::count(&vec, &2);
        assert_eq!(p, 2);

        let p = vec.count(&2);
        assert_eq!(p, 2);
    }
}
