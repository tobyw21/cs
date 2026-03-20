// non-panic refcell
use super::cell::Cell;
use std::cell::UnsafeCell;


// This is a state flag for reference state
#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Exclusive,
    Shared(i32),
}

// Similar to Cell, but with additional reference state to check borrowing with & and &mut
// during runtime
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    refcount: Cell<RefState>,
}
// same as Cell, Sync + Send are not implemented as it is not thread safe.

// Return wrapper values for borrow and borrow_mut
// https://doc.rust-lang.org/stable/std/cell/struct.Ref.html
// if we don't use this wrapper, in impl of RefCell, simply returning a Option<T>
// we are able to set and check the state of RefCell but never able to decrement the shared
// counter whenever the RefCell borrowed reference is dropped when reaching out of the scope. 
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

// must impl Drop trait for bookeeping the reference state
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.refcount.get() {
            RefState::Shared(1) => {
                // set refstate to unshared because we are the only shared reference
                self.refcell.refcount.set(RefState::Unshared);
            },
            RefState::Shared(n) => {
                // decrease refstate by 1, there are still other shared refs
                self.refcell.refcount.set(RefState::Shared(n - 1));
            },
            // not possible to be here
            // if a shared reference occurs, either other shared refs are somewhere else
            // or only 1 exclusive reference occurs and no other shared refs.
            _ => { /* not possible to reach here */ }
        }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.refcount.get() {
            RefState::Exclusive => {
                self.refcell.refcount.set(RefState::Unshared);
            }
            _ => { /* not possible to reach here */ }
        }
    }
}

// impl deref to able to dereference the Ref in order to get refcell value
impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

// need to get the exclusive reference from mutable borrow
impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            refcount: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.refcount.get() {
            RefState::Shared(n) => {
                self.refcount.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Unshared => {
                // not shared, make it to shared
                self.refcount.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            // already exclusive reference, give out none
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.refcount.get() {
            RefState::Exclusive => {
                // already &mut out there, can't have more &mut refs
                None
            }
            RefState::Shared(_) => {
                // already shared can't have &mut refs
                None
            }
            RefState::Unshared => {
                self.refcount.set(RefState::Exclusive);
                Some(RefMut { refcell: self })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_refcell() {
        let c = RefCell::new(1);

        let borrowed = c.borrow().unwrap();

        assert_eq!(*borrowed, 1);

        let borrowed2 = c.borrow().unwrap();
        assert_eq!(*borrowed2, 1);

        let mutborrowed = c.borrow_mut();
        assert!(mutborrowed.is_none());
    }

    #[test]
    fn test_refcell2() {
        let c = RefCell::new(1);

        let mut mutborrowed = c.borrow_mut().unwrap();
        *mutborrowed = 3;
        assert_eq!(*mutborrowed, 3);

        let borrowed = c.borrow();
        assert!(borrowed.is_none());
    }
}
