use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::fmt::{Display, Formatter, Result};
use std::cmp::PartialEq;

pub trait Resolveable {
    fn start(&mut self) -> ();
    fn is_resolved(&self) -> bool;
    fn is_started(&self) -> bool;
}

#[derive(Eq, Debug)]
pub struct Node<'a, T>
where T: Eq + Resolveable + Hash + Display {
    pub value: T,
    pub dependants: RefCell<HashSet<&'a Node<'a, T>>>,
    pub dependencies: RefCell<HashSet<&'a Node<'a, T>>>,
    pub cycle_safe: bool
}

impl<'a, T> PartialEq for Node<'a, T>
where T: Eq + Resolveable + Hash + Display {
    fn eq(&self, other: &Node<'a, T>) -> bool {
        self.value == other.value
    }
}

impl<'a, T> Display for Node<'a, T>
where T: Eq + Resolveable + Hash + Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(Node: {})", self.value)
    }
}

impl<'a, T> Hash for Node<'a, T>
where T: Eq + Resolveable + Hash + Display {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<'a, T> Node<'a, T>
where T: Eq + Resolveable + Hash + Display {
    pub fn new(v: T) -> Node<'a, T> {
        Node{
            value: v,
            dependants: RefCell::new(HashSet::new()),
            dependencies: RefCell::new(HashSet::new()),
            cycle_safe: false
        }
    }

    pub fn add_dependency(&self, dep: &'a Node<'a, T>) {
        self.dependencies.borrow_mut().insert(dep);
    }

    pub fn add_dependant(&self, dep: &'a Node<'a, T>) {
        self.dependants.borrow_mut().insert(dep);
    }

    pub fn resolved(&self) -> bool {
        self.value.is_resolved()
    }
}
