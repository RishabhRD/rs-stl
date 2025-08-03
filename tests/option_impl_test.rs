// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn some() {
        let mut c = Some(2);

        assert!(!c.start());
        assert!(c.end());
        assert_eq!(c.next(c.start()), c.end());
        assert_eq!(c.prior(c.end()), c.start());

        assert_eq!(*c.at(&c.start()), 2);
        *c.at_mut(&c.start()) = 3;
        assert_eq!(*c.at(&c.start()), 3);
    }

    #[test]
    fn none() {
        let c: Option<i32> = None;

        assert!(c.start());
        assert!(c.end());
    }

    #[test]
    fn iter_full_some() {
        let c = Some(5);
        let sum = Collection::iter(&c).sum::<i32>();
        assert_eq!(sum, 5)
    }

    #[test]
    fn iter_full_none() {
        let c: Option<i32> = None;
        let sum = Collection::iter(&c).sum::<i32>();
        assert_eq!(sum, 0)
    }

    #[test]
    fn iter_sliced_some() {
        let c = Some(5);
        let mut s = c.full();
        s.drop_first();
        let sum = s.iter().sum::<i32>();
        assert_eq!(sum, 0)
    }
}
