# The Drop Check
By default, compiler will assume dropping a type generic over T will access the T, if T impls Drop.

In order to ask compiler to not to assume the type T be accessed when dropping, a nightly feature `#![feature(dropck_eyepatch)]` and `unsafe impl<#[may_dangle] T> std::ops::Drop for Type<T>` is needed, note that this feature is nightly and there is no stable solution to this as of today.

However this will break if inner data is accessed, ie, a type `struct Foo(T)`, it impls Drop but access T, so when `struct Bar(Foo(T))` is dropped, compiler will throw error, it only tells compiler we guarantee not to touch the data to skip the check and it won't tell if we want to drop it.

## PhantomData
`std::marker::PhantomData` is a marker type that will ask compiler to check T's drop implementation.

`#[may_dangle]` is an assurance to compiler that I will not access data in the T, but it don't check the drop for T.

PhantomData ask compiler to check if `T::drop()` when the outer struct is dropped.

PhantomData also is used to mark the variance on type.