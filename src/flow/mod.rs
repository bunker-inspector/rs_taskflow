mod dag;

use dag::Dag;
use dag::node::Node;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

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

pub struct Flow<'a, 'b, T, U>
where T: Eq + Hash + Resolveable<U> + Display {
    dag: Dag<'a, 'b, T>,
    phantom: Option<PhantomData<U>>
}

impl<'a, 'b, T, U> Flow<'a, 'b, T, U>
where T: Eq + Hash + Resolveable<U> + Display {
    pub fn new_task(value: T) -> Node<'a, T> {
        Node::new(value)
    }

    pub fn dep(to: &'a Node<'a, T>, from:  &'a Node<'a, T>) {
        Dag::dep(&to, &from);
    }

    fn exec(task: &'a mut T) -> U {
        task.exec()
    }

    pub fn start(&mut self) {
    }

    pub fn build(tasks: Vec<&'a Node<'b, T>>) -> Flow<'a, 'b, T, U> {
        Flow{dag: Dag::build(tasks), phantom: None}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_test() {
        let a = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {1})));
        let b = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {2})));

        Flow::build(vec![&a, &b]);
    }
}
