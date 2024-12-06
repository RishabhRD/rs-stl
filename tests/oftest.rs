// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use rng::infix::*;
use stl::*;

#[test]
fn all_of_for_true() {
    let vecp = vec![2, 3, 5];
    let vec = vec![1, 3, 5];
    let p1 = algo::all_of(&vecp, vecp.after(vecp.start()), vecp.end(), |x| {
        x % 2 == 1
    });
    let p2 = rng::all_of(&vec, |x| x % 2 == 1);
    let p3 = vec.all_of(|x| x % 2 == 1);

    assert_eq!(p1, true);
    assert_eq!(p2, true);
    assert_eq!(p3, true);
}

#[test]
fn all_of_for_false() {
    let vecp = vec![2, 4, 5];
    let vec = vec![1, 4, 5];
    let p1 = algo::all_of(&vecp, vecp.after(vecp.start()), vecp.end(), |x| {
        x % 2 == 1
    });
    let p2 = rng::all_of(&vec, |x| x % 2 == 1);
    let p3 = vec.all_of(|x| x % 2 == 1);

    assert_eq!(p1, false);
    assert_eq!(p2, false);
    assert_eq!(p3, false);
}

#[test]
fn all_of_for_empty() {
    let vecp = vec![2, 4, 5];
    let vec = vec![];
    let p1 = algo::all_of(&vecp, vecp.start(), vecp.start(), |x| x % 2 == 1);
    let p2 = rng::all_of(&vec, |x| x % 2 == 1);
    let p3 = vec.all_of(|x| x % 2 == 1);

    assert_eq!(p1, true);
    assert_eq!(p2, true);
    assert_eq!(p3, true);
}

#[test]
fn any_of_for_true() {
    let vecp = vec![2, 3, 4];
    let vec = vec![1, 3, 4];
    let p1 = algo::any_of(&vecp, vecp.after(vecp.start()), vecp.end(), |x| {
        x % 2 == 1
    });
    let p2 = rng::any_of(&vec, |x| x % 2 == 1);
    let p3 = vec.any_of(|x| x % 2 == 1);

    assert_eq!(p1, true);
    assert_eq!(p2, true);
    assert_eq!(p3, true);
}

#[test]
fn any_of_for_false() {
    let vec = vec![2, 4, 6];
    let p1 =
        algo::any_of(&vec, vec.after(vec.start()), vec.end(), |x| x % 2 == 1);
    let p2 = rng::any_of(&vec, |x| x % 2 == 1);
    let p3 = vec.any_of(|x| x % 2 == 1);

    assert_eq!(p1, false);
    assert_eq!(p2, false);
    assert_eq!(p3, false);
}

#[test]
fn any_of_for_empty() {
    let vecp = vec![2, 4, 6];
    let vec = vec![];
    let p1 = algo::any_of(&vecp, vecp.start(), vecp.start(), |x| x % 2 == 1);
    let p2 = rng::any_of(&vec, |x| x % 2 == 1);
    let p3 = vec.any_of(|x| x % 2 == 1);

    assert_eq!(p1, false);
    assert_eq!(p2, false);
    assert_eq!(p3, false);
}

#[test]
fn none_of_for_true() {
    let vecp = vec![1, 4, 6];
    let vec = vec![2, 4, 6];
    let p1 = algo::none_of(&vecp, vecp.after(vecp.start()), vecp.end(), |x| {
        x % 2 == 1
    });
    let p2 = rng::none_of(&vec, |x| x % 2 == 1);
    let p3 = vec.none_of(|x| x % 2 == 1);

    assert_eq!(p1, true);
    assert_eq!(p2, true);
    assert_eq!(p3, true);
}

#[test]
fn none_of_for_false() {
    let vecp = vec![2, 4, 5];
    let vec = vec![2, 4, 5];
    let p1 = algo::none_of(&vecp, vecp.after(vecp.start()), vecp.end(), |x| {
        x % 2 == 1
    });
    let p2 = rng::none_of(&vec, |x| x % 2 == 1);
    let p3 = vec.none_of(|x| x % 2 == 1);

    assert_eq!(p1, false);
    assert_eq!(p2, false);
    assert_eq!(p3, false);
}

#[test]
fn none_of_for_empty() {
    let vecp = vec![2, 4, 5];
    let vec = vec![];
    let p1 = algo::none_of(&vecp, vecp.start(), vecp.start(), |x| x % 2 == 1);
    let p2 = rng::none_of(&vec, |x| x % 2 == 1);
    let p3 = vec.none_of(|x| x % 2 == 1);

    assert_eq!(p1, true);
    assert_eq!(p2, true);
    assert_eq!(p3, true);
}
