// A nightly feature telling compiler to not assume drop will access data
#![feature(dropck_eyepatch)]

use std::{marker::PhantomData, ptr::NonNull};

struct Boks<T> {
    // Using *mut T will make Boks invariant, and the std Box is covariant
    // ptr: *mut T,
    // NonNull is naturally covariant.
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
    // PhantomData<fn() -> T> is also covariant but can't be used here
    // this will not subject to the drop check
    // it will bring back the bug that 
    // it will not check the drop if T is a struct with T
}

// Not related from here to deref_mut...
impl <T> Boks<T> {
    fn ny(t: T) -> Self {
        Boks {
            // Safety: Box never creates a null pointer
            ptr: unsafe { 
                NonNull::new_unchecked(Box::into_raw(Box::new(t)))
            }, 
            _marker: PhantomData 
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr.as_mut() }
    }
}
// Not related section ends here.

// #[may_dangle] on T telling compiler we guarantee not to access T when dropping.
// It allows Boks to contain references that live shorter than the lifetime of Boks.
unsafe impl<#[may_dangle] T> std::ops::Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety: ptr was constructed by a box at the first place, and has not been free'd
        // since self still exists (otherwise drop would not be called).
        let _ = unsafe { Box::from_raw(self.ptr.as_mut()) }; 
    }
}

struct Foo<T: std::fmt::Display>(T);

impl<T: std::fmt::Display> Drop for Foo<T> {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn main() {
    let mut a = 42;
    let b = Boks::ny(&mut a);
    println!("{}", a);


    let mut a = 42;
    let b = Boks::ny(Foo(&mut a));
    
    // This is not working,
    // either impl Drop for Foo with #[may_dangle]
    // or do not impl Drop at all
    // println!("{}", a);

    // b gets dropped here 
    // drop(b)
    // and drop access Foo's field, it creates interleaving exclusive and shared references.


    // Check variance of Boks
    // std Box is covariant, if we use *mut T, it is invariant because of mut
    // then Boks<&'a str> = Boks<&'static str> will not work
    let s = String::new();
    let mut boks1 = Boks::ny(&*s);
    let ss: &'static str = "hello";
    let boks2: Boks<&'static str> = Boks::ny(ss);
    boks1 = boks2;
    
}