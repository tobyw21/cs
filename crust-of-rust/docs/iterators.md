# Iterators
## IntoIterator
`IntoIterator` trait contains when a type can be converted into iterator, like a vec, hashmap... it has a type `IntoIter` which is an alias of `Iterator<Item = Self::Item>`, is the iterator contains an item which is a data type.

## DoubleEndedIterator
similar to regular iterator, `DoubleEndedIterator` gives `next_back` and `rev` options to regular iterator which can iterate from the back. 

## Different types of `iter*`
- `iter()`: generate iterator with `&T` from collection, collection remains valid.
- `iter_mut()`: generator iterator with `&mut T` from collection, can change items, collection remains valid.
- `into_iter()`: 
    + if collection contains `T`, it takes ownership and return an iterator.
    + if collection contains `&T` it is the same as `iter()`.
    + if collection contains `&mut T` it is the same as `iter_mut()`.