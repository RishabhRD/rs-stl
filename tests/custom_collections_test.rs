// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::{
        collections::{EmptyCollection, SingletonCollection},
        *,
    };

    #[test]
    fn collection_of_one() {
        let mut c = SingletonCollection::new(2);

        assert!(!c.start());
        assert!(c.end());
        assert_eq!(c.next(c.start()), c.end());
        assert_eq!(c.prior(c.end()), c.start());

        assert_eq!(*c.at(&c.start()), 2);
        *c.at_mut(&c.start()) = 3;
        assert_eq!(*c.at(&c.start()), 3);

        let c = SingletonCollection::new(2);
        assert!(!c.is_empty());

        let mut s = c.full();
        s.drop_first();
        assert!(s.is_empty());

        let mut i = c.start();
        let succ = c.form_next_n_limited_by(&mut i, 0, c.start());
        assert_eq!(i, c.start());
        assert!(succ);

        let mut i = c.start();
        let succ = c.form_next_n_limited_by(&mut i, 1, c.start());
        assert_eq!(i, c.start());
        assert!(!succ);

        let mut i = c.start();
        let succ = c.form_next_n_limited_by(&mut i, 1, c.end());
        assert_eq!(i, c.end());
        assert!(succ);

        let mut i = c.start();
        let succ = c.form_next_n_limited_by(&mut i, 2, c.end());
        assert_eq!(i, c.end());
        assert!(!succ);

        let mut i = c.start();
        let succ = c.form_prior_n_limited_by(&mut i, 0, c.start());
        assert_eq!(i, c.start());
        assert!(succ);

        let mut i = c.start();
        let succ = c.form_prior_n_limited_by(&mut i, 1, c.start());
        assert_eq!(i, c.start());
        assert!(!succ);

        let mut i = c.end();
        let succ = c.form_prior_n_limited_by(&mut i, 1, c.start());
        assert_eq!(i, c.start());
        assert!(succ);

        let mut i = c.end();
        let succ = c.form_prior_n_limited_by(&mut i, 2, c.start());
        assert_eq!(i, c.start());
        assert!(!succ);
    }

    #[test]
    fn empty_collection() {
        let c: EmptyCollection<i32> = EmptyCollection::new();

        let mut i = ();
        let succ = c.form_next_n_limited_by(&mut i, 0, ());
        assert!(succ);

        let mut i = ();
        let succ = c.form_next_n_limited_by(&mut i, 2, ());
        assert!(!succ);

        let mut i = ();
        let succ = c.form_prior_n_limited_by(&mut i, 0, ());
        assert!(succ);

        let mut i = ();
        let succ = c.form_prior_n_limited_by(&mut i, 2, ());
        assert!(!succ);
    }
}
