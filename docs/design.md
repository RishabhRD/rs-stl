# rs-stl Design

rs-stl introduces the concept of collection into rust ecosystem. A collection
is linear multi-pass sequence of `elements`.

Every element has a `position` associated with it. There is a `start` position
in the collection which contains first element of collection. Also there is
a `end` position in the collection which is just after last element of collection.

```text
  _ _ _ _ _ _

  ^            ^
  |            |
start   -->   end
```

With abstraction of collections, rs-stl also provides generic algorithms to
operate over collections. This eliminates the need of implementing algorithms
on special types like `[T]`.

## Collection Hierarchy in terms of traversal

`Collection -> BidirectionalCollection -> RandomAccessCollection`.

### Collection

Collection is the base trait that formally defines the linear sequence of
elements. Elements are not necessarily needed to be stored in memory.

```rust
pub trait Collection {
    type Position: Regular;
    type Element;
    type ElementRef<'a>: std::ops::Deref<Target = Self::Element>
    type Whole: Collection<
        Position = Self::Position,
        Element = Self::Element,
        Whole = Self::Whole,
    >;

    /// Returns the position of first element in self.
    fn start(&self) -> Self::Position;
    /// Returns the position just after last element in collection.
    fn end(&self) -> Self::Position;
    /// Returns position immediately after i
    fn next(&self, i: Self::Position) -> Self::Position;
    /// Access element at position i.
    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_>;
    /// Returns a contiguous subrange of given collection.
    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole>;
}
```

Collection defines associated type Position and Element that determines type
of positions in collection and elements in collection. Every element has
an associated position.

To access an element at a position, `at` method can be used.

`start`, `end` and `next` method provides essential methods for forward traversal.

Every collection has provides a slice method, that returns slice of the collection.
A slice is a dependent collection which internally refers to original collection
and contains 0 or more contiguous original collection elements.

Associated type `Whole` always denotes the original collection. For types like
`Array`, `Whole = Self`. For types like `Slice<T>`, `Whole = T` as T here denotes
the original collection. This property is really important considering, `slice`
method returns `Slice<Whole>` as for `Slice<T>`, slice method would not
return `Slice<Slice<T>>` but return `Slice<T>`. This avoidance of nesting is
really necessary for slices being first class citizen and being used for
algorithm composition. This property is also visible in bounds on associated
type Whole.

### BidirectionalCollection

BidirectionalCollection provides `prior` method to support backward iteration.

```rust
pub trait BidirectionalCollection: Collection
where
    Self::Whole: BidirectionalCollection,
{
    /// Returns position immediately before i
    fn prior(&self, i: Self::Position) -> Self::Position;
}
```

### RandomAccessCollection

In actual defintion of `Collection`, it also provides `next_n` and `distance`
method with default implementation of O(n). Similarily, `BidirectionalCollection`
also provides `prior_n` method with default implementation of O(n).

RandomAccessCollection only mandates these methods to be O(1). For a data
structure to be RandomAccessCollection, it needs to provide override of above
methods with O(1) implementation. Also, it needs to explicitly provide impl for
RandomAccessCollection.

```rust
pub trait RandomAccessCollection:
    BidirectionalCollection<Position: Regular + Ord>
where
    Self::Whole: RandomAccessCollection,
{
}
```

## Collection Hierarchy in terms of mutation

`Collection -> ReorderableCollection -> MutableCollection`.

### ReorderableCollection

ReorderableCollection provides ability to reorder the elements collection
without any external mutation. For reordering it provides `swap_at` method.

```rust
pub trait ReorderableCollection: Collection
where
    Self::Whole: ReorderableCollection,
{
    /// Swaps element at position i with element at position j.
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position);

    /// Returns mutable slice of collection in positions [from, to).
    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::Whole>;
}
```

Also, `slice_mut` method provides access to slice that mutably references
original collection. This can be used for composition of mutating algorithms
that work on parts of collection.

### MutableCollection

```rust
pub trait MutableCollection: ReorderableCollection
where
    Self::Whole: MutableCollection,
{
    /// Mutably Access element at position i.
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;
}
```

MutableCollection provides mutable access to any element in collection for
supporting external mutation.

## Collection Hierarchy in terms of laziness

### LazyCollection

LazyCollection is a collection whose elements are computed on element access.
This suggests that the returned element are not actually stored in memory.

LazyCollection trait is mostly for optimization purposes where one might
need ownership of returned element, then it would avoid redundant copy.

```rust
pub trait LazyCollection: Collection
where
    Self::Whole: LazyCollection,
{
    /// Computes element at position `i`.
    fn compute_at(&self, i: &Self::Position) -> Self::Element;
}
```

## Algorithms

rs-stl provides multiple generic algorithms over these abstractions. These
algorithms are exposed as methods using extension traits like this:

```rust
pub trait CollectionExt: Collection {
    fn find_if<Pred>(&self, mut pred: Pred) -> Self::Position
    where
        Pred: FnMut(&Self::Element) -> bool,
    {
        let mut start = self.start();
        let end = self.end();
        while start != end {
            if pred(self.at(&start)) {
                return start;
            }
            start = self.next(start);
        }
        start
    }
}

impl<R> CollectionExt for R where R: Collection + ?Sized {}
```

Exposing algorithms as methods instead of free functions are helpful for:

1. Algorithms chaining
2. IDE support

Different extension traits are used for algorithms that require different
capabilities of collections like `RandomAccessCollectionExt`.

## Data Structures

Implementation for these traits are provided for standard library data structures.
Currently

- `[T; N]`
- `[T]`
- `Vec<T>`
- `Option<T>`
- `Range<T>`
- `RangeInclusive<T>`

is supported. Will support more data structures in future.

## Slice

Since `[T]` requires its elements to be contiguous in memory, it can't be used
for generic collection abstraction.

rs-stl introduces 2 slice types: `Slice<T>` and `SliceMut<T>` to represent
immutable and mutable slice to collection.

Slices in rs-stl itself is a collection such that slicing itself doesn't create
nested slice.

```rust
pub trait Collection {
    type Position: Regular;
    type Element;
    type Whole: Collection<
        Position = Self::Position,
        Element = Self::Element,
        Whole = Self::Whole,
    >;
    // other details...
    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole>;
}
```

`slice` method always returns `Slice<Self::Whole>`. `Whole` associated type
always represents the type of "full" collection. Thus for types like
`[T; N]`, `Whole = Self`. However, for types like `Slice<T>`, `Whole = T`.

This way slices never nest. This is very important for using slices for
recursive algorithms composition without any efficiency loss.

Similar properties are held for `SliceMut` type.

For convenience, `full`, `prefix_upto`, `suffix_from`, `full_mut`, `prefix_upto_mut`,
`suffix_from_mut` methods are also provided as algorithms in addition to `slice` and
`slice_mut` method for slicing.

## Language Limitations

There are some language limitations rs-stl suffer with. This leads to some
ugly corners in library that is unavoidable.

### Lack of yield-once coroutines

Swift's subscript operator provide ability to `yield` element. This is
really helpful to project `ephermal` parts of data structure. This enables
swift to have `Collection` trait that doesn't require `Element` in memory.

However, rs-stl needs to incorporate proxy references using `ElementRef` for the
same purpose. For example, `Range<i32>` can't be modelled as `Collection` without
`ElementRef` abstraction as `Range<i32>` doesn't actually contains `i32`.
`ElementRef` is ugly and hurts ergonomics of API.

With yield-once coroutine, `at` method can return reference to elements that
doesn't actually exist in memory and API would be really simple:

```rust
    fn at(&self, i: &Self::Position) -> &Self::Element;
```

This is simpler in terms of use and also symmetrical with `MutableCollection`.
**If rust community chooses to pick one thing from here, please pick yield-once
coroutines.**

### Lifetime GATs are useless right now

Similar to how collections exposes their custom `Iterator` type, collections
could expose their custom `Slice` type too. However, when working with
refinement like `BidirectionalCollection`, associated `Slice` type should also
need to conform to `BidirectionalCollection`. However, that would require
`for<...>` syntax on lifetimes of `Slice` associated type, that would require
lifetime of `Slice` associated type to be `'static`. This is a well known
language limitation currently.

To overcome the same, `Whole` associated type is exposed on which generic `Slice`
and `SliceMut` structs can be built.

The same bug is also the reason why rust currently not have `LendingIterator`
abstraction at all.

### Unable of handling recursive trait conditionals

For example this just not compiles:

```rust
pub trait LazyCollection: Collection<Whole: LazyCollection> {
    // details here...
}
```

So one needs to write `Whole: LazyCollection` all the time one might want to
depends on `LazyCollection` extension.

### Lack of extension keyword

Swift has really great extension keyword which enables to add methods to
traits/structs after the definition of trait/structs.

This enables Single Responsibility Principle as trait would only include
the core/primitive methods required for implementing the trait. Other algorithms
can be provided as an extension to trait/struct.

Currently for coping the same, `extension-traits` are used that requires
defining another trait like `CollectionExt: Collection` and implement it for
all `Collection`. This hurts ergonomics in terms of IDE perspective as end-user
doesn't need to know about `CollectionExt`.
