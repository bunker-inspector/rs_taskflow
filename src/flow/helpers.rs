use std::cell::RefCell;
use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

pub trait Resolveable<T> {
    fn exec(&mut self) -> T;
    fn resolved(&self) -> bool;
    fn started(&self) -> bool;
}

pub struct DefaultResolveable<'a, T> {
    task: &'a Fn() -> T,
    started: bool,
    complete: bool
}

impl<'a, T> Resolveable<T> for DefaultResolveable<'a, T> {
    fn exec(&mut self) -> T {
        self.started = true;
        let result = (self.task)();
        self.complete = true;

        result
    }

    fn resolved(&self) -> bool {
        self.complete
    }

    fn started(&self) -> bool {
        self.started
    }

}

impl<'a, T> PartialEq for DefaultResolveable<'a, T> {
    fn eq(&self, other: &DefaultResolveable<'a, T>) -> bool {
        (self.task as *const Fn() -> T) == (other.task as *const Fn() -> T)
            && self.started == other.started
            && self.complete == other.complete
    }
}

impl<'a, T> Eq for DefaultResolveable<'a, T> {}

impl<'a, T> Display for DefaultResolveable<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "DefaultResolveable: {:?}", self as *const DefaultResolveable<'a, T>)
    }
}

impl<'a, T> Hash for DefaultResolveable<'a, T>
where T: {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.task as *const Fn() -> T).hash(state);
        self.started.hash(state);
        self.complete.hash(state);
    }
}

impl<'a, T> DefaultResolveable<'a, T> {
    pub fn new(task: &'a Fn() -> T) -> DefaultResolveable<T> {
        DefaultResolveable{task, started: false, complete: false}
    }
}

pub struct RefCellWrapper<T>
where T: Eq {
    pub c: RefCell<T>
}

impl<T> RefCellWrapper<T>
where T: Eq {
    pub fn new(t: T) -> RefCellWrapper<T> {
        RefCellWrapper{c: RefCell::new(t)}
    }
}

impl<'a, T> PartialEq for RefCellWrapper<T>
where T: Eq {
    fn eq(&self, other: &RefCellWrapper<T>) -> bool {
        *self.c.borrow() == *other.c.borrow()
    }
}

impl<T> Eq for RefCellWrapper<T> where T: Eq {}

impl<T> Display for RefCellWrapper<T>
where T: Eq + Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RefCellWrapper: {}", *self.c.borrow())
    }
}


impl<T> Hash for RefCellWrapper<T>
where T: Eq + Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.c.borrow_mut()).hash(state);
    }
}

impl<T, U> Resolveable<U> for RefCellWrapper<T>
where U: Eq, T: Eq + Resolveable<U> {
    fn exec(&mut self) -> U {
        (*self.c.borrow_mut()).exec()
    }
    
    fn resolved(&self) -> bool {
        (*self.c.borrow()).resolved()
    }

    fn started(&self) -> bool {
        (*self.c.borrow()).started()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolveable_hash_test() {
        let foo = RefCellWrapper::new(1);
        let mut bar = HashSet::new();

        bar.insert(foo);

        assert!(bar.len() == 1);

        bar.remove(&RefCellWrapper::new(1));

        assert!(bar.is_empty());
    }
}
