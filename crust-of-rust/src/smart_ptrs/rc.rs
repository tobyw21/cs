use std::clone::Clone;
use std::ops::Deref;
use std::ptr::NonNull;
use std::marker::PhantomData;
use super::cell::Cell;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>
}
struct Rc<T> {
    ptr: NonNull<RcInner<T>>,
    phantom: PhantomData<RcInner<T>>
}

impl<T> Rc<T> {
    fn new() -> Self {
        todo!()
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        
    }
}