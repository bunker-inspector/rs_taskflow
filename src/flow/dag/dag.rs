use crate::flow::dag::node::Node;

use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Eq, PartialEq, Debug)]
pub struct Dag<'a, 'b, T>
where
    T: Eq + Hash + Display,
{
    pub roots: HashSet<&'a Node<'b, T>>,
}

enum CycleCheckStatus {
    Initial,
    Processing,
    Processed,
}

impl<'a, 'b, T> Dag<'a, 'b, T>
where
    T: Eq + Hash + Display,
{
    pub fn node(value: T) -> Node<'b, T> {
        Node::new(value)
    }

    pub fn build(nodes: Vec<&'a Node<'b, T>>) -> Dag<'a, 'b, T> {
        let mut roots = HashSet::new();

        for node in nodes {
            if node.dependencies.borrow().is_empty() {
                roots.insert(node);
            }
        }

        Dag::check(Dag { roots })
    }

    pub fn insert(&mut self, new_node: &'a Node<'b, T>) {
        if new_node.dependencies.borrow().is_empty() {
            self.roots.insert(new_node);
        }

        Dag::_check(new_node, &mut HashMap::new());
    }

    pub fn dep(from: &'a Node<'a, T>, to: &'a Node<'a, T>) {
        from.add_dependency(to);
        to.add_dependant(from);
    }

    pub fn remove(&mut self, to_remove: &'a Node<'b, T>) {
        to_remove.remove();

        self.roots.remove(to_remove);

        for node in to_remove.dependants.borrow().iter() {
            if node.dependencies.borrow().is_empty() {
                self.roots.insert(&node);
            }
        }
    }

    fn check(dag: Dag<'a, 'b, T>) -> Dag<'a, 'b, T> {
        if dag.roots.is_empty() {
            panic!("No roots found. DAG is invalid!");
        }

        if dag
            .roots
            .iter()
            .all(|root| Dag::_check(&root, &mut HashMap::new()))
        {
            dag
        } else {
            panic!("Invalid DAG detected")
        }
    }

    fn _check(
        pt: &'a Node<'b, T>,
        visited: &mut HashMap<&'a Node<'b, T>, CycleCheckStatus>,
    ) -> bool {
        visited.insert(pt, CycleCheckStatus::Processing);

        let deps = pt.dependants.borrow();

        for dep in deps.iter() {
            println!("Visiting {}", dep);

            let status = match visited.get(dep) {
                Some(v) => v,
                None => &CycleCheckStatus::Initial,
            };

            match status {
                CycleCheckStatus::Initial => {
                    if !Dag::_check(dep, visited) {
                        return false;
                    }
                }
                CycleCheckStatus::Processing => return false,
                CycleCheckStatus::Processed => {}
            }
        }

        visited.insert(pt, CycleCheckStatus::Processed);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct MockStruct {
        id: char,
    }

    impl MockStruct {
        fn new(id: char) -> MockStruct {
            MockStruct { id }
        }
    }

    impl Display for MockStruct {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.id)
        }
    }

    #[test]
    #[should_panic]
    fn build_dag() {
        let a = Dag::node(MockStruct::new('A'));
        let b = Dag::node(MockStruct::new('B'));
        let c = Dag::node(MockStruct::new('C'));
        let d = Dag::node(MockStruct::new('D'));
        let e = Dag::node(MockStruct::new('E'));
        let f = Dag::node(MockStruct::new('F'));
        let g = Dag::node(MockStruct::new('G'));
        let h = Dag::node(MockStruct::new('H'));

        Dag::dep(&b, &a);
        Dag::dep(&c, &b);
        Dag::dep(&d, &c);
        Dag::dep(&e, &d);
        Dag::dep(&f, &d);
        Dag::dep(&g, &f);
        Dag::dep(&h, &f);
        Dag::dep(&b, &d); //causes circular dependency

        Dag::build(vec![&a, &b, &c, &d, &e, &f, &g, &h]);
    }

    #[test]
    fn remove_nodes() {
        let a = Dag::node(MockStruct::new('A'));
        let b = Dag::node(MockStruct::new('B'));

        Dag::dep(&b, &a);
        let mut dag = Dag::build(vec![&a, &b]);

        assert!(
            !b.dependencies.borrow().is_empty(),
            "Node was not successfully removed"
        );
        dag.remove(&a);
        assert!(
            b.dependencies.borrow().is_empty(),
            "Node was not successfully removed"
        );
    }

    #[test]
    fn node_hash() {
        let a = Node::new(MockStruct::new('A'));
        let b = Node::new(MockStruct::new('B'));
        let c = Node::new(MockStruct::new('C'));
        let d = Node::new(MockStruct::new('D'));
        let e = Node::new(MockStruct::new('E'));
        let f = Node::new(MockStruct::new('F'));
        let g = Node::new(MockStruct::new('G'));
        let h = Node::new(MockStruct::new('H'));

        let mut hash: HashSet<&Node<MockStruct>> = HashSet::new();

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
}
