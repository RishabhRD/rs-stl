// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    #[test]
    fn find_if_when_element_is_present() {
        let vec = vec![1, 2, 3];
        let p2 = rng::find_if(&vec, |x| x % 2 == 0);
        let p3 = vec.find_if(|x| x % 2 == 0);

        assert_eq!(p2, 1);
        assert_eq!(p3, 1);
    }

    #[test]
    fn find_if_when_element_is_not_present() {
        let vec = vec![1, 3, 5];
        let p2 = rng::find_if(&vec, |x| x % 2 == 0);
        let p3 = vec.find_if(|x| x % 2 == 0);

        assert_eq!(p2, 3);
        assert_eq!(p3, 3);
    }

    #[test]
    fn find_if_when_range_is_empty() {
        let vec = vec![];
        let p2 = rng::find_if(&vec, |x| x % 2 == 0);
        let p3 = vec.find_if(|x| x % 2 == 0);

        assert_eq!(p2, 0);
        assert_eq!(p3, 0);
    }

    #[test]
    fn find_if_not_when_element_is_present() {
        let vec = vec![2, 3, 4];
        let p2 = rng::find_if_not(&vec, |x| x % 2 == 0);
        let p3 = vec.find_if_not(|x| x % 2 == 0);

        assert_eq!(p2, 1);
        assert_eq!(p3, 1);
    }

    #[test]
    fn find_if_not_when_element_is_not_present() {
        let vec = vec![2, 4, 6];
        let p2 = rng::find_if_not(&vec, |x| x % 2 == 0);
        let p3 = vec.find_if_not(|x| x % 2 == 0);

        assert_eq!(p2, 3);
        assert_eq!(p3, 3);
    }

    #[test]
    fn find_if_not_when_range_is_empty() {
        let vec = vec![];
        let p2 = rng::find_if_not(&vec, |x| x % 2 == 0);
        let p3 = vec.find_if_not(|x| x % 2 == 0);

        assert_eq!(p2, 0);
        assert_eq!(p3, 0);
    }

    #[test]
    fn find_when_element_is_present() {
        let vec = vec![2, 3, 4];
        let p2 = rng::find(&vec, &3);
        let p3 = vec.find(&3);

        assert_eq!(p2, 1);
        assert_eq!(p3, 1);
    }

    #[test]
    fn find_when_element_is_not_present() {
        let vec = vec![2, 4, 6];
        let p2 = rng::find(&vec, &3);
        let p3 = vec.find(&3);

        assert_eq!(p2, 3);
        assert_eq!(p3, 3);
    }

    #[test]
    fn find_when_range_is_empty() {
        let vec = vec![];
        let p2 = rng::find(&vec, &3);
        let p3 = vec.find(&3);

        assert_eq!(p2, 0);
        assert_eq!(p3, 0);
    }
}
