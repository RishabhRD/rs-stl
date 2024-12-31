# How to use

## Ranges

### Basic Traversal

Let's first understand, how to traverse range with accessing elements:

immutable:

```rust
use stl::*;

let arr = [1, 2, 3];

let start = arr.start();
while arr.is_end(&start) {
    println!("{}", arr.at(&start));
}

// If end position is known one can also do:
let start = arr.start();
let end = arr.end();
while start != end {
    println!("{}", arr.at(&start));
}
```

mutable:

```rust
let mut arr = [1, 2, 3];

let start = arr.start();
while arr.is_end(&start) {
    arr.at_mut(&start) = 3;
}
```

### Algorithms

rs-stl provides 3 ways to call an algorithm:

1. `algo::`
2. `rng::`
3. infix

#### algo

`algo::` version accepts `start` and `end` positions explictly, to work on given
subrange of range. For example:

```rust
use stl::*;

let arr = [1, 2, 3, 4, 5];
let i = arr.after_n(arr.start(), 2);
let j = arr.end();
let cnt = algo::count_if(&arr, i, j, |x| x % 2 == 1);
assert_eq!(cnt, 2)
```

The above algorithms work on `[i, j)` positions of `arr` rather than working
on full arr.

This version is quite useful while writing a new generic algorithms with help
of other already existing algorithms.

#### rng

`rng::` version just accepts the given range for working, and work over full
range. For example:

```rust
use stl::*;

let arr = [1, 2, 3, 4, 5];
let cnt = rng::count_if(&arr, |x| x % 2 == 1);
assert_eq!(cnt, 3)
```

#### infix

infix version comes under `stl::rng::infix` module, that enables algorithms
to be used as methods. All `rng` algorithms doesn't support infix version and
thus look into `stl::rng` module's algorithm document to know if that supports
infix version.

```rust
use stl::*;
use rng::infix::*;

let arr = [1, 2, 3, 4, 5];
let cnt = arr.count_if(|x| x % 2 == 1);
assert_eq!(cnt, 3)
```

## Views

### Creation

#### From ranges

For creating view from a given range, one can use `.view()` method or `.view_mut()`
method.

immutable view:

```rust
let arr = [1, 2, 3];
let v = arr.view();
```

mutable view:

```rust
let mut arr = [1, 2, 3];
let v = arr.view_mut();
```

Here v doesn't consume arr, but do immutable and mutable borrow of arr respectively.

#### Using factories

View factories are functions that returns a view without taking a range/view
as argument.

```rust
let ints = view::seq(0, |x| *x + 1); // 0, 1, 2, 3, ...
```

#### Using view adaptors

View adaptors are functions that accept view as an argument by value and
returns a new view by consuming given view. For example:

```rust
let int_3 = view::seq(0, |x| *x + 1)
                .take(3); // 0, 1, 2
```

In above example, take is an adaptor.

### With Algorithms

As views are also ranges, they can be used with the algorithms described above:

```rust
use stl::*;
use rng::infix::*;

let mut arr = [(1, 2), (2, 1)];
arr
  .view()
  .map(|x| x.1)
  .sort_range();
assert_eq!(arr, [(2, 1), (1, 2)]);
```

In above example, `.view().map(...)` gives a view that is passed to sort_range
algorithm. As this view mutably borrows arr, arr is sorted.

This enables inplace mutation of ranges with functional style programming.
