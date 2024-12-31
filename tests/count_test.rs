// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn count_if_for_element_exist() {
        let vec = vec![1, 2, 3, 4, 5];
        let p1 = algo::count_if(&vec, vec.after(vec.start()), vec.end(), |x| {
            x % 2 == 1
        });
        let p2 = rng::count_if(&vec, |x| x % 2 == 1);
        let p3 = vec.count_if(|x| x % 2 == 1);

        assert_eq!(p1, 2);
        assert_eq!(p2, 3);
        assert_eq!(p3, 3);
    }

    #[test]
    fn count_if_for_element_not_exist() {
        let vec = vec![2, 4, 6, 8];
        let p1 = algo::count_if(&vec, vec.after(vec.start()), vec.end(), |x| {
            x % 2 == 1
        });
        let p2 = rng::count_if(&vec, |x| x % 2 == 1);
        let p3 = vec.count_if(|x| x % 2 == 1);

        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
        assert_eq!(p3, 0);
    }

    #[test]
    fn count_if_for_empty_range() {
        let vec = vec![];
        let p1 = algo::count_if(&vec, vec.start(), vec.end(), |x| x % 2 == 1);
        let p2 = rng::count_if(&vec, |x| x % 2 == 1);
        let p3 = vec.count_if(|x| x % 2 == 1);

        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
        assert_eq!(p3, 0);
    }

    #[test]
    fn count_for_element_exist() {
        let vec = vec![2, 4, 2, 8];
        let p1 = algo::count(&vec, vec.after(vec.start()), vec.end(), &2);
        let p2 = rng::count(&vec, &2);
        let p3 = vec.count(&2);

        assert_eq!(p1, 1);
        assert_eq!(p2, 2);
        assert_eq!(p3, 2);
    }
}
