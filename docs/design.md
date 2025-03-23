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

```rust
pub trait Collection {
    type Position: Regular;
    type Element;
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
    fn after(&self, i: Self::Position) -> Self::Position;
    /// Access element at position i.
    fn at(&self, i: &Self::Position) -> &Self::Element;
    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<Self::Whole>;
}
```

Collection defines associated type Position and Element that determines type
of positions in collection and elements in collection. Every element has
an associated position.

To access an element at a position, `at` method can be used.

`start`, `end` and `after` method provides essential methods for forward traversal.

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

BidirectionalCollection provides `before` method to support backward iteration.

```rust
pub trait BidirectionalCollection: Collection
where
    Self::Whole: BidirectionalCollection,
{
    /// Returns position immediately before i
    fn before(&self, i: Self::Position) -> Self::Position;
}
```

### RandomAccessCollection

In actual defintion of `Collection`, it also provides `after_n` and `distance`
method with default implementation of O(n). Similarily, `BidirectionalCollection`
also provides `before_n` method with default implementation of O(n).

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
    ) -> SliceMut<Self::Whole>;
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

## Algorithms

rs-stl provides multiple generic algorithms over these abstractions. These
algorithms are exposed as methods using extension traits like this:

```rust
pub trait CollectionExt: Collection {
    fn find_if<Pred>(&self, pred: Pred) -> Self::Position
    where
        Pred: Fn(&Self::Element) -> bool,
    {
        let mut start = self.start();
        let end = self.end();
        while start != end {
            if pred(self.at(&start)) {
                return start;
            }
            start = self.after(start);
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

is supported. Will support more data structures in future.
