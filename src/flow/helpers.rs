use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

pub trait Resolveable<T, E> {
    fn exec(&mut self) -> Result<T, E>;
    fn resolved(&self) -> bool;
    fn started(&self) -> bool;
}

pub struct DefaultResolveable<'a, T, E> {
    task: &'a Fn() -> Result<T, E>,
    started: bool,
    complete: bool,
}

impl<'a, T, E> Resolveable<T, E> for DefaultResolveable<'a, T, E> {
    fn exec(&mut self) -> Result<T, E> {
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

impl<'a, T, E> PartialEq for DefaultResolveable<'a, T, E> {
    fn eq(&self, other: &DefaultResolveable<'a, T, E>) -> bool {
        (self.task as *const Fn() -> Result<T, E>) == (other.task as *const Fn() -> Result<T, E>)
            && self.started == other.started
            && self.complete == other.complete
    }
}

impl<'a, T, E> Eq for DefaultResolveable<'a, T, E> {}

impl<'a, T, E> Display for DefaultResolveable<'a, T, E> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "DefaultResolveable: {:?}",
            self as *const DefaultResolveable<'a, T, E>
        )
    }
}

impl<'a, T, E> Hash for DefaultResolveable<'a, T, E>
where
    T: ,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.task as *const Fn() -> Result<T, E>).hash(state);
        self.started.hash(state);
        self.complete.hash(state);
    }
}

impl<'a, T, E> DefaultResolveable<'a, T, E> {
    pub fn new(task: &'a Fn() -> Result<T, E>) -> DefaultResolveable<'a, T, E> {
        DefaultResolveable {
            task,
            started: false,
            complete: false,
        }
    }
}

pub struct RefCellWrapper<T>
where
    T: Eq,
{
    pub c: RefCell<T>,
}

impl<T> RefCellWrapper<T>
where
    T: Eq,
{
    pub fn new(t: T) -> RefCellWrapper<T> {
        RefCellWrapper { c: RefCell::new(t) }
    }
}

impl<'a, T> PartialEq for RefCellWrapper<T>
where
    T: Eq,
{
    fn eq(&self, other: &RefCellWrapper<T>) -> bool {
        *self.c.borrow() == *other.c.borrow()
    }
}

impl<T> Eq for RefCellWrapper<T> where T: Eq {}

impl<T> Display for RefCellWrapper<T>
where
    T: Eq + Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "RefCellWrapper: {}", *self.c.borrow())
    }
}

impl<T> Hash for RefCellWrapper<T>
where
    T: Eq + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.c.borrow_mut()).hash(state);
    }
}

impl<T, U, E> Resolveable<U, E> for RefCellWrapper<T>
where
    U: Eq,
    T: Eq + Resolveable<U, E>,
{
    fn exec(&mut self) -> Result<U, E> {
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
