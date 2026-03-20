// unsafe cell is the core of interior mutability
// https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html
use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// cell is not thread safe, it does not impl Send + Sync
// impl<T> !Sync for Cell<T> {}
// data race happens like regular types when this is used in thread
// no lock primitives are preventing data race.

// the idea of cell is to _copy_ out the value to shared reference holders
// the intuitive is that if there is no reference to the inner value wrapped by UnsafeCell
// therefore without concurrency, the value can be changed without doing any pointer validation

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// copy out the underlying value from unsafecell,
    /// hence get can only work when T impls Copy
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    /// set the value via UnsafeCell
    pub fn set(&self, value: T) {
        // safe: knowing no other threads holding the reference of this cell.
        unsafe { *self.value.get() = value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell() {
        let c = Cell::new(1);

        assert_eq!(c.get(), 1);

        c.set(2);

        assert_eq!(c.get(), 2);
    }
}
