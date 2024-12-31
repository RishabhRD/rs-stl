# rs-stl Design

The document explains the design for rs-stl in details. For understanding the
design of any system, following information is necessary:

1. Entities in the system
2. Possible Operations with entities

rs-stl contains 2 kind of entities:

1. Range
2. Views

And supports following operations:

1. Algorithms over ranges
2. Composition of views

## Entities

### Range

A range models a linear sequence of elements, where every element has a `Position`
associated with it. One can access element in range with use of the
range and position of the element. `Position` can be thought as generalization
of indexes in arrays.

Similar to arrays, one can traverse a range by the use of `Positions`. A range
have a `start` Position and optionally have an `end` Position. A range would
not have an `end` position, if that range is an infinite range.

One can visualize range as follows:

```text
  _ _ _ _ _ _

  ^            ^
  |            |
start   -->   end
```

- Start Position is the position of first element in the range.
- End Position is position immediately after last element in the range.

Thus, rs-stl consistently treats range as semi-open range with positions
`[start, end)`. semi-open range representation is chosen because they enables
us to represent empty ranges.

However, it is not necessary that end position is known upfront, when one
starts using the range, but that doesn't mean the range is empty.

For example:

```rust
let arr = [1, 2, 3, 4, 5, 6, 7, 8];
let v = arr.view().take(5);
```

In above example, v is a range that defines end criteria with first 5 elements.
However, it's not necessary that `end` position would be known upfront.

For representing the same a more general notation for range positions would
be `[start, is_end)`, where is_end is a method of range that accepts the
position and returns whether that position is the end position or not. rs-stl
takes this model of representing positions as default model as it is the more
general one.

If any range has end position known upfront, that range would be treated as
more specialized range with enhanced capabilities.

rs-stl defines traits, that one struct has to implement to adopt the range
behavior. rs-stl defines following traits based on capabilities of the range:

- `InputRange`: is the most primitive range, that supports on single-pass
  traversal.
- `BoundedRange`: is an input range, in which end position is known upfront.
- `ForwardRange`: is an input range, that supports multi-pass traversal. i.e.,
  one can clone and save the position for initiating another totally
  independent traversal
- `BidirectionalRange`: is a forward range, that supports backward traversal as well.
- `RandomAccessRange`: is bidirectional range, which supports jumping from any
  position to any position in O(1) time.
- `SemiOutputRange`: is a mutable forward range, that supports just reordering
  of range elements.
- `OutputRange`: is semi-output range, that supports changing any element in range.

For more information about any traits, plese read API doc for stl traits.

In general, these methods can be used for traversing a range with element access.
If `rng` is a range, then:

- `rng.start()` returns position of first element.
- `rng.is_end(&i)` checks if i is the end position.
- `rng.at(&i)` to access element at position i.
- `rng.after(i)` to get position immediately after ith position.
- `rng.end()` to get the end position, **if range is BoundedRange**.
- `rng.before(i)` to get position immediately before ith position.
- `rng.swap_at(&i, &j)` to swap elements at position i and j **if range is SemiOutputRange**.
- `rng.at_mut(&i)` to access element at position i mutably **if range is OutputRange**.

### Views

`Views` are intended to be used as non-owning range. That means view itself is
a range and can be used in any place that requires a range.

For distinguishing `view` from a normal range, a marker trait `View` is defined:

```rust
pub trait View: InputRange{}
```

Any struct that implements `View` trait would be a View. Because View is an
extension of InputRange, that struct would atleast be an `InputRange`.

Because of non-owning nature of view, a view can be struct with one of following
properties:

1. Containing an immutable/mutable reference to another range.
2. Containing another view itself (own that view).
3. A view over elements that doesn't exist in memory itself.

To obtain an immutable view of a range use `.view()` method:

```rust
use stl::*;
use rng::infix::*;

let arr = [1, 2, 3];
let v = arr.view();
v.for_each(|x| println!("{}", x))
```

To obtain a mutable view of a range use `.view_mut()` method:

```rust
use stl::*;
use rng::infix::*;

let arr = [1, 2, 3];
let v = arr.view();
v.sort_range();
```

## Operations with entities

### Algorithms

rs-stl supports multiple useful algorithms over different kind of ranges.
These are generic algorithms and are not dependent on any specific data-structures.
This is something, currently rust lacks in their standard library.

Another goal for algorithms, is to provide performance with support for inplace
mutation in a generic way.

There are 3 ways to express an algorithm in general:

1. `algo::`: explicitly accepts 2 positions to work with.
2. `rng::`: just need to pass range.
3. infix: use algorithms as methods.

Let's see an example of `count_if` for the same:

```rust
use stl::*;
use rng::infix::*;

let arr = [1, 2, 3];

let n = algo::count_if(&arr, arr.start(), arr.end(), |x| x % 2 == 1);
assert_eq!(n, 1);

let n = rng::count_if(&arr, |x| x % 2 == 1);
assert_eq!(n, 1);

let n = arr.count_if(|x| x % 2 == 1); // infix version
assert_eq!(n, 1);
```

For using `algo::` and `rng::` version just import `stl::*`. For using infix
version import `rng::infix::*`.

See `stl::algo` and `stl::rng` module for knowing about all algorithms rs-stl supports.

### View Composition
