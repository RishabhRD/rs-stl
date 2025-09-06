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
    fn form_next_n_limited_by() {
        let c = Some(2);

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

        let c: Option<i32> = None;

        let mut i = c.start();
        let succ = c.form_next_n_limited_by(&mut i, 0, c.start());
        assert_eq!(i, c.start());
        assert!(succ);

        let mut i = c.start();
        let succ = c.form_next_n_limited_by(&mut i, 1, c.end());
        assert_eq!(i, c.end());
        assert!(!succ);
    }

    #[test]
    fn form_prior_n_limited_by() {
        let c = Some(2);

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

        let c: Option<i32> = None;

        let mut i = c.end();
        let succ = c.form_next_n_limited_by(&mut i, 0, c.end());
        assert_eq!(i, c.start());
        assert!(succ);

        let mut i = c.end();
        let succ = c.form_next_n_limited_by(&mut i, 1, c.start());
        assert_eq!(i, c.start());
        assert!(!succ);
    }

    #[test]
    fn none() {
        let c: Option<i32> = None;

        assert!(c.start());
        assert!(c.end());
    }
}
