mod dag;
mod helpers;

use dag::Dag;
use dag::node::Node;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;
use helpers::{DefaultResolveable, Resolveable, RefCellWrapper};
use std::collections::{VecDeque, HashSet, HashMap};

pub struct Flow<'a, 'b, T, U, E>
where T: Eq + Hash + Resolveable<U, E> + Display {
    dag: Dag<'a, 'b, RefCellWrapper<T>>,
    ready: VecDeque<&'a Node<'b, RefCellWrapper<T>>>,
    errors: HashMap<&'a Node<'b, RefCellWrapper<T>>,E>,
    phantom: Option<PhantomData<U>>
}

impl<'a, 'b, T, U, E> Flow<'a, 'b, T, U, E>
where T: Eq + Hash + Resolveable<U, E> + Display {
    pub fn new_task(value: T) -> Node<'a, RefCellWrapper<T>> {
        Node::new(RefCellWrapper::new(value))
    }

    pub fn dep(to: &'a Node<'a, T>, from:  &'a Node<'a, T>) {
        Dag::dep(&to, &from);
    }

    pub fn start(&mut self) {
        loop {
            match self.ready.pop_front() {
                Some(ref node) => {
                    self.dag.remove(node);

                    node.value.c.borrow_mut().exec();

                    for dependant in node.dependants.borrow().iter() {
                        if dependant.dependencies.borrow().is_empty() {
                            self.ready.push_back(dependant);
                        }
                    }
                },
                None => { break; }
            }
        }
    }

    pub fn build(tasks: Vec<&'a Node<'b, RefCellWrapper<T>>>) -> Flow<'a, 'b, T, U, E> {
        let mut ready = VecDeque::new();

        for task in tasks.iter() {
            if task.dependencies.borrow().is_empty() {
                ready.push_back(*task);
            }
        }

        Flow{dag: Dag::build(tasks), ready, phantom: None, errors: HashMap::new()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_test() {
        let a = Flow::new_task(DefaultResolveable::new(
            &(|| -> i32 {1}),
            &(|| -> i32 {-1})));
        let b = Flow::new_task(DefaultResolveable::new(
            &(|| -> i32 {2}),
            &(|| -> i32 {-2})));
        let c = Flow::new_task(DefaultResolveable::new(
            &(|| -> i32 {3}),
            &(|| -> i32 {-3})));

        Flow::dep(&a, &b);
        Flow::dep(&b, &c);
        let mut flow = Flow::build(vec![&a, &b, &c]);

        flow.start();
    }

    #[test]
    fn flow_test_2() {
        let a = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {1}), &(|| -> i32 {-1})));
        let b = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {2}), &(|| -> i32 {-2})));

        Flow::dep(&a, &b);
        let mut flow = Flow::build(vec![&a, &b]);

        assert!(&a.dependencies.borrow().contains(&b));
        assert!(&b.dependants.borrow().contains(&a));
    }

    #[test]
    fn hash_test() {
        let a = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {1}), &(|| -> i32 {-1})));
        let b = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {2}), &(|| -> i32 {-2})));

        let mut foo = HashSet::new();

        foo.insert(&a);
        foo.insert(&b);
        assert!(foo.len() == 2);

        foo.remove(&a);
        foo.remove(&a);
        assert!(foo.len() == 1)
    }
}
