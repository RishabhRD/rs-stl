// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use stl::*;

#[test]
fn start_position() {
    let vec = vec![10, 20, 30];
    assert_eq!(vec.start_position(), 0)
}

#[test]
fn end_position() {
    let vec = vec![10, 20, 30];
    assert_eq!(vec.end_position(), 3)
}

#[test]
fn position_after() {
    let vec = vec![10, 20, 30];
    assert_eq!(vec.position_after(vec.start_position()), 1)
}

#[test]
fn at() {
    let vec = vec![10, 20, 30];
    assert_eq!(*vec.at(&0), 10)
}

#[test]
fn position_before() {
    let vec = vec![10, 20, 30];
    assert_eq!(vec.position_before(3), 2)
}

#[test]
fn nth_position_after() {
    let vec = vec![10, 20, 30];
    assert_eq!(vec.nth_position_after(0, 2), 2)
}

#[test]
fn nth_position_before() {
    let vec = vec![10, 20, 30];
    assert_eq!(vec.nth_position_before(2, 2), 0)
}

#[test]
fn at_mut() {
    let mut vec = vec![10, 20, 30];
    *vec.at_mut(&0) = 2;
    assert_eq!(*vec.at(&0), 2)
}
