# Interior Mutability and Smart Pointers - (Cell, RefCell, Rc, Arc, NonNull, PhantomData, Cow...)

## Cell
Cell allocates a space on stack to allow interior mutability with (`UnsafeCell`)[https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html], 
like a `Cell::new(v: T) where T: Copy + Clone`, which uses `set()` and `get()` functions to change the interior value, it can only hold value impls Copy and Clone,
imagine if you are always returning a copy of its value, it never get any problem of 
having multiple exclusive reference, and because of this, it knowing no other thread is holding any types of reference of this value, so it is safe to write a new value. It is **NOT** thread safe.

## RefCell
Refcell allocates a space on stack with `UnsafeCell`, different from `Cell`, it has a reference count to track if multiple references occurs at the same time, it is either ONE exclusive reference with NO shared reference, or MULTIPLE shared reference and ZERO exclusive reference. Returns a `Ref` or `RefMut` to track the reference count, if `Ref` or `RefMut` is not returned, the refcount is never going to be decreased when a refcell reference by `borrow()` or `borrow_mut()` is dropped. It is **NOT** thread safe, no sync primitives are used.

## Rc
Rc allocates a space on heap with `Box`, it holds the pointer to the allocated space which has a value and a refcount. When the pointer gets cloned, it returns a new `Rc` points to the same region of memory and increase the refcount, and when cloned `Rc` gets dropped, decrease refcount until only 1 `Rc` copy left. 

## Arc
A thread safe version of Rc, Atomic Rc uses (`Atomic`)[https://doc.rust-lang.org/std/sync/atomic/type.Atomic.html] to assue the sync between threads.

## NonNull

## CoW

## PhantomData
In `Rc` implementation, we wrapped `RcInner` inside `Rc`, when dropping `Rc`, pointers and space inside `RcInner` won't be dropped by compiler, it could cause memleak and dangling pointer.

(dropcheck)[https://doc.rust-lang.org/nomicon/dropck.html]
