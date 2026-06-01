//! Smart pointers: Box, Rc, Arc, RefCell.

use std::cell::RefCell;
use std::rc::Rc;

/// A recursive data structure requiring Box for heap allocation.
#[derive(Debug, PartialEq)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Nil
    }

    pub fn prepend(self, value: T) -> Self {
        List::Cons(value, Box::new(self))
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared ownership with Rc + interior mutability with RefCell.
#[derive(Debug)]
pub struct SharedCounter {
    inner: Rc<RefCell<i64>>,
}

impl SharedCounter {
    pub fn new(initial: i64) -> Self {
        Self {
            inner: Rc::new(RefCell::new(initial)),
        }
    }

    /// Creates another handle to the same counter.
    pub fn clone_handle(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }

    pub fn increment(&self) {
        *self.inner.borrow_mut() += 1;
    }

    pub fn get(&self) -> i64 {
        *self.inner.borrow()
    }

    /// Returns the number of strong references.
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_recursive_list() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        match &list {
            List::Cons(1, tail) => match tail.as_ref() {
                List::Cons(2, _) => {}
                other => panic!("unexpected: {other:?}"),
            },
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn rc_refcell_shared_mutation() {
        let counter = SharedCounter::new(0);
        let handle = counter.clone_handle();

        assert_eq!(counter.strong_count(), 2);

        counter.increment();
        handle.increment();
        handle.increment();

        assert_eq!(counter.get(), 3);
        assert_eq!(handle.get(), 3);
    }
}
