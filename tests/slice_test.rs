// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start_position() {
        let arr = [10, 20, 30];
        let s = &arr[0..];
        assert_eq!(s.start(), 0)
    }

    #[test]
    fn end_position() {
        let arr = [10, 20, 30];
        let s = &arr[0..];
        assert_eq!(s.end(), 3)
    }

    #[test]
    fn position_after() {
        let arr = [10, 20, 30];
        let s = &arr[0..];
        assert_eq!(s.after(s.start()), 1)
    }

    #[test]
    fn at() {
        let arr = [10, 20, 30];
        let s = &arr[1..];
        assert_eq!(*s.at(&0), 20)
    }

    #[test]
    fn position_before() {
        let arr = [10, 20, 30];
        let s = &arr[0..];
        assert_eq!(s.before(3), 2)
    }

    #[test]
    fn nth_position_after() {
        let arr = [10, 20, 30];
        let s = &arr[0..];
        assert_eq!(s.after_n(0, 2), 2)
    }

    #[test]
    fn nth_position_before() {
        let arr = [10, 20, 30];
        let s = &arr[0..];
        assert_eq!(s.before_n(2, 2), 0)
    }

    #[test]
    fn at_mut() {
        let mut arr = [10, 20, 30];
        let s = &mut arr[0..];
        *s.at_mut(&0) = 2;
        assert_eq!(*s.at(&0), 2);
        assert_eq!(*arr.at(&0), 2);
    }
}
