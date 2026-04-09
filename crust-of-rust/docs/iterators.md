# Iterators
## IntoIterator
`IntoIterator` trait contains when a type can be converted into iterator, like a vec, hashmap... it has a type `IntoIter` which is an alias of `Iterator<Item = Self::Item>`, is the iterator contains an item which is a data type.

# DoubleEndedIterator
similar to regular iterator, `DoubleEndedIterator` gives `next_back` and `rev` options to regular iterator which can iterate from the back. 