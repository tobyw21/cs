use super::cell::Cell;
use std::clone::Clone;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

// Need RcInner to adjust refcount
// Rc needs to be cloned so it is impossible to put it in Rc
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

impl<T> RcInner<T> {
    fn new(value: T) -> Self {
        RcInner {
            value,
            refcount: Cell::new(1),
        }
    }

}

pub struct Rc<T> {
    ptr: NonNull<RcInner<T>>,
    // why do we need phantom?
    // read this: https://doc.rust-lang.org/nomicon/dropck.html 
    phantom: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner::new(value));
        
        Rc {
            // Safety: It is safe here because we can make sure the pointer is always
            // not null
            ptr: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            phantom: PhantomData,
        }   
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Self { ptr: self.ptr, phantom: PhantomData }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Get &T from Rc, NonNull providers as_ref which returns &RcInner
        &unsafe { self.ptr.as_ref() }.value
    }
}

impl<T> DerefMut for Rc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut unsafe { self.ptr.as_mut() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        // Get inner from a Rc, if refcount == 1, no other references are elsewhere
        // drop the inner and consume the raw pointer
        // otherwise, just set refcount - 1
        let inner = unsafe { self.ptr.as_ref() };
        let c = inner.refcount.get();
        if inner.refcount.get() == 1 {
            let _ = inner;
            let _ = unsafe { Box::from_raw(self.ptr.as_ptr()) };
        } else {
            inner.refcount.set(c - 1);
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        let a = Rc::new(1);
        
        {
            let _ = a.clone();
        }
        let b = a.clone();
        let mut c = a.clone();
        *c = 4;
        assert_eq!(*a, 4);
        assert_eq!(*b, 4);

    }
}