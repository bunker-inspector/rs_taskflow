mod dag;

use dag::Dag;
use dag::node::Node;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Display;
use std::collections::HashSet;
use std::cell::RefCell;

pub struct Task<'a, T>
where T: Eq + Hash + Resolveable<T> + Display {
    t: Node<'a, T>
}

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

impl<'a, T> DefaultResolveable<'a, T> {
    pub fn new(task: &'a Fn() -> T) -> DefaultResolveable<T> {
        DefaultResolveable{task, started: false, complete: false}
    }
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

pub struct Flow<'a, 'b, T>
where T: Eq + Hash + Resolveable<T> + Display {
    dag: Dag<'a, 'b, T>
}

impl<'a, 'b, T> Flow<'a, 'b, T>
where T: Eq + Hash + Resolveable<T> + Display {
    pub fn new_task(value: T) -> Task<'a, T> {
        Task {
            t: Node{
                value,
                dependencies: RefCell::new(HashSet::new()),
                dependants: RefCell::new(HashSet::new())
            }
        }
    }

    pub fn dep(to: &'a Task<'a, T>, from:  &'a Task<'a, T>) {
        Dag::dep(&to.t, &from.t);
    }

    pub fn start(&mut self) {
    }
}
 
