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
2. Lazy composition of views

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
`[start, end)`. semi-open range representation is chosen because they enable
us to represent empty ranges.

However, it is not necessary that end position for a range would always be known
upfront.

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
more specialized range with enhanced capabilities and is called `BoundedRange`.

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

#### Range Internals

It is really important to understand these traits first before understanding range
internals:

```rust
pub trait SemiRegular: Eq {}
pub trait Regular: SemiRegular + Clone {}
```

The range definitions rely on these traits for their working. These traits are
ideas from Alex Stepanov. rs-stl provides default implementation of these traits
as well:

```rust
impl<T> SemiRegular for T where T: Eq {}
impl<T> Regular for T where T: SemiRegular + Clone {}
```

Internals about `InputRange` will help to learn about ranges in general, as it
is the most primitive range:

```rust
pub trait InputRange {
    type Position: SemiRegular;

    type Element;
    type ElementRef<'a>: std::ops::Deref<Target = Self::Element>
    where
        Self: 'a;

    fn start(&self) -> Self::Position;
    fn is_end(&self, i: &Self::Position) -> bool;
    fn after(&self, i: Self::Position) -> Self::Position;
    fn after_n(&self, mut i: Self::Position, mut n: usize) -> Self::Position {
        while n > 0 {
            i = self.after(i);
            n -= 1;
        }
        i
    }
    fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a>;
}
```

Every range has atlease 3 typedefs:

- `Position` defining the type of position in range.
- `Element` defining the type of element range contains.
- `ElementRef<'a>` defining the reference type of ElementRef. (`ElementRef` is
  discussed in detail in later section).

For supporting single-pass nature of `InputRange`, `Position` is `SemiRegular`
so that, position can't be copied for independent traversal.

InputRange also defines O(n) implementation for `after_n` method. If any struct
implementing `InputRange` have better implementation, it can override the
same for more efficient implementation. Usually, structs like `Vec` provide
O(1) implementation for the same.

This overriding of default methods become important when working with
`RandomAccessRange` as it requires these methods to work in O(1).

`ForwardRange` relaxes the `Position` to be `Regular` and thus positions can
be cloned and used for multi-pass traversal.

`OutputRange` also defines an extra typedef `ElementMutRef`:

```rust
pub trait OutputRange: SemiOutputRange {
    type ElementMutRef<'a>: std::ops::DerefMut<Target = Self::Element>
    where
        Self: 'a;

    fn at_mut<'a>(&'a mut self, i: &Self::Position) -> Self::ElementMutRef<'a>;
}
```

`ElementMutRef` is discussed in details in later section.

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

### ElementRef and ElementMutRef

As seen earlier, `InputRange` defines a typedef `ElementRef`:

```rust
type ElementRef<'a>: std::ops::Deref<Target = Self::Element>
where
    Self: 'a;
```

and `at(i)` method returns `ElementRef` value.

On the first sight it looks like `ElementRef` should always be `&Self::Element`.
This is true for most of data structures.

However, with introduction of views, this is not always true. For example,
consider `view::ints(0)`. This range doesn't contain any real element at all.
Thus, it is not possible to return `&Self::Element` from `at` method because
there is no such element itself.

For coping with the same, the concept of `ElementRef` is introduced.
`ElementRef` for these kind of cases can be used as **proxy references**.

The definition of `ElementRef` suggests:

1. `ElementRef` is bounded to `std::ops::Deref<Target = Self::Element>`, i.e.,
   dereferencing object of `ElementRef` would result in `Self::Element`.
2. Lifetime of `ElementRef` is bounded to lifetime of `Self`. Thus, `ElementRef`
   should be valid till `Self` is valid.

```rust
fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a>;
```

The defintion of `at` method suggests:

1. `at` method returns `Self::ElementRef` value whose lifetime is bounded to
   `self` lifetime.

The same story is there for `OutputRange` with `ElementMutRef` typedef and `at_mut`
method.

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
version import `rng::infix::*`. `algo::` version seems unnecessary, however it
is supported as it serves well while compsing eager algorithms to form new
algorithms.

See API Documentation `stl::algo` and `stl::rng` module for knowing about all algorithms rs-stl supports.

### Lazy composition of views

This operation is similar to what Rust Iterator provides, but way more
powerful than that.

It would be easy to understand with an example:

```rust
use stl::*;
use rng::infix::*;

let arr = [1, 2, 3];

let v = arr.view().map(|x| *x + 1);
assert!(v.equals(&[2, 3, 4]));
assert!(arr.equals(&[2, 3, 4])); // arr is not changed
```

In the above example, `v` is a view over `arr` with mapping elements to `ele + 1`.
However, that doesn't change arr at all.

The given function is not even applied on line:
`let v = arr.view().map(|x| *x + 1);`.
Rather, it would be applied when actually element is accessed using position,
that `equals` algorithm does. So, composition of views are lazy in nature whereas,
the algorithms provided by `algo::` or `rng::` module are eager in nature.

This allows to chain view adaptors to create a higher order view without
worrying about intermediate allocations as views are non-owning ranges. This can
be seen like functional programming approach.

Another advantage of non-ownership is, views can be used to represent infinite
ranges:

```rust
let v = view::ints(0); // 0, 1, 2, 3, ...
```

One can have immutable/mutable view to ranges.
This allows views to be used with all eager generic algorithms. And as result,
inplace mutation of data can be achieved using views.

For example, this is possible with views, but not with rust iterators:

```rust
use stl::*;
use rng::infix::*;

let mut arr = [(1, 2), (2, 1))];
arr.view_mut().map(|x| x.1).sort_range();
assert_eq!(arr, [(2, 1), (1, 2)]);
```

View adaptors like `map` operates over views and consumes the given view.

See API Documentation `stl::view` module for more information about views and all supported
view adaptors.
