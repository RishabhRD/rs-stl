// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let arr = [10, 20, 30];
        assert_eq!(arr[..].start(), 0);
    }

    #[test]
    fn end() {
        let arr = [10, 20, 30];
        assert_eq!(arr[..].end(), 3);
    }

    #[test]
    fn after() {
        let arr = [10, 20, 30];
        assert_eq!(arr[..].after(0), 1);
    }

    #[test]
    fn after_n() {
        let arr = [10, 20, 30];
        assert_eq!(arr[..].after_n(0, 2), 2);
    }

    #[test]
    fn before() {
        let arr = [10, 20, 30];
        assert_eq!(arr[..].before(1), 0);
    }

    #[test]
    fn before_n() {
        let arr = [10, 20, 30];
        assert_eq!(arr[..].before_n(2, 2), 0);
    }

    #[test]
    fn distance() {
        let arr = [10, 20, 30];
        assert_eq!(arr[..].distance(0, 2), 2);
    }

    #[test]
    fn at() {
        let arr = [10, 20, 30];
        assert_eq!(*arr[..].at(&0), 10);
    }
}
