use crate::dag::node::Node;
use crate::dag::node::Resolveable;

use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
struct Dag<'a, T>
where T: Eq + Hash + Resolveable + Display {
    nodes: HashSet<&'a Node<'a, T>>,
    leaves: HashSet<&'a Node<'a, T>>
}

enum CycleCheckStatus {
    Initial,
    Processing,
    Processed
}

impl<'a, T> Dag<'a, T>
where T: Eq + Hash + Resolveable + Display {
    pub fn new() -> Dag<'a, T> {
        Dag{
            nodes: HashSet::new(),
            leaves: HashSet::new()
        }
    }

    pub fn add(&mut self, node: &'a Node<'a, T>) -> &'a mut Dag<T> {
        self.nodes.insert(node);
        self
    }

    pub fn dep(&'a mut self, from: &'a Node<'a, T>, to: &'a Node<'a, T>) -> &'a mut Dag<T> {
        if !self.nodes.contains(from) {
            panic!("Cannot add edge connecting to node that has not been added to the graph. Node: {}", from)
        }

        if !self.nodes.contains(to) {
            panic!("Cannot add edge connecting to node that has not been added to the graph. Node: {}", to)
        }

        if !self.check_cycle(from, to) {
            panic!("Attempted edge insertion would cause cycle containing: {}. Aborting.", from)
        }

        to.add_dependant(from);
        from.add_dependency(to);
        self
    }

    pub fn build(&mut self) {
    }

    fn check_cycle(&mut self, start: &'a Node<'a, T>, pt: &'a Node<'a, T>) -> bool {
        let mut visited: HashMap<&'a Node<'a, T>, CycleCheckStatus> = HashMap::new();
        visited.insert(start, CycleCheckStatus::Processing);

        return self._check_cycle(pt, &mut visited)
    }

    fn _check_cycle(&mut self,
                    pt: &'a Node<'a, T>,
                    visited: &mut HashMap<&'a Node<'a, T>, CycleCheckStatus>) -> bool {
        visited.insert(pt, CycleCheckStatus::Processing);

        let deps = pt.dependencies.borrow();

        if deps.is_empty() {
            self.leaves.insert(pt);
        }

        for dep in deps.iter() {
            let status = match visited.get(dep) {
                Some(v) => v,
                None    => &CycleCheckStatus::Initial
            };

            match status {
                CycleCheckStatus::Initial    => {
                    if !self._check_cycle(dep, visited) {
                        return false;
                    }
                },
                CycleCheckStatus::Processing => return false,
                CycleCheckStatus::Processed  => {}
            }
        }

        visited.insert(pt, CycleCheckStatus::Processed);
        true
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dag::node::Resolveable;

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct MockResolveable {
        id: char,
        done: bool
    }

    impl MockResolveable {
        fn new(id: char) -> MockResolveable {
            MockResolveable{id, done: false}
        }
    }

    impl Display for MockResolveable {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.id)
        }
    }

    impl Resolveable for MockResolveable {
        fn start(&mut self) { self.done = true }
        fn is_started(&self) -> bool { self.done }
        fn is_resolved(&self) -> bool { self.done }
    }

    #[test]
    #[should_panic]
    fn build_dag() {
        let a = Node::new(MockResolveable::new('A'));
        let b = Node::new(MockResolveable::new('B'));
        let c = Node::new(MockResolveable::new('C'));
        let d = Node::new(MockResolveable::new('D'));
        let e = Node::new(MockResolveable::new('E'));
        let f = Node::new(MockResolveable::new('F'));
        let g = Node::new(MockResolveable::new('G'));
        let h = Node::new(MockResolveable::new('H'));

        let mut dag = Dag::new();

        dag.add(&a)
            .add(&b)
            .add(&c)
            .add(&d)
            .add(&e)
            .add(&f)
            .add(&g)
            .add(&h)
            .dep(&b, &a)
            .dep(&c, &b)
            .dep(&d, &b)
            .dep(&d, &c)
            .dep(&e, &d)
            .dep(&f, &d)
            .dep(&g, &f)
            .dep(&h, &f)
            .dep(&c, &f) //causes cycle, panicks
            .build();
    }

    #[test]
    fn node_hash() {
        let a = Node::new(MockResolveable::new('A'));
        let b = Node::new(MockResolveable::new('B'));
        let c = Node::new(MockResolveable::new('C'));
        let d = Node::new(MockResolveable::new('D'));
        let e = Node::new(MockResolveable::new('E'));
        let f = Node::new(MockResolveable::new('F'));
        let g = Node::new(MockResolveable::new('G'));
        let h = Node::new(MockResolveable::new('H'));

        let mut hash: HashSet<&Node<MockResolveable>> = HashSet::new();

        hash.insert(&a);
        hash.insert(&b);
        hash.insert(&c);
        hash.insert(&d);
        hash.insert(&e);
        hash.insert(&f);
        hash.insert(&g);
        hash.insert(&h);

        assert!(hash.contains(&a), "Node did not hash properly");
        assert!(hash.contains(&b), "Node did not hash properly");
        assert!(hash.contains(&c), "Node did not hash properly");
        assert!(hash.contains(&d), "Node did not hash properly");
        assert!(hash.contains(&e), "Node did not hash properly");
        assert!(hash.contains(&f), "Node did not hash properly");
        assert!(hash.contains(&g), "Node did not hash properly");
        assert!(hash.contains(&h), "Node did not hash properly");
    }


    #[test]
    #[should_panic]
    fn dep_without_node() {
        let a = Node::new(MockResolveable::new('A'));
        let b = Node::new(MockResolveable::new('B'));

        let mut dag = Dag::new();

        dag.dep(&b, &a);
    }
}
