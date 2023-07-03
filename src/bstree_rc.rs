#![allow(unused)]
use std::{cell::RefCell, rc::Rc};

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct BSTree<T> {
    root: Option<Link<T>>,
}

pub struct Node<T> {
    value: T,
    left: Option<Link<T>>,
    right: Option<Link<T>>,
    parent: Option<Link<T>>,
}

impl<T> From<Node<T>> for Link<T> {
    fn from(value: Node<T>) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_with_parent(value: T, parent: Link<T>) -> Self {
        Self {
            value,
            left: None,
            right: None,
            parent: Some(parent),
        }
    }
}

impl<T> BSTree<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match &self.root {
            Some(root) => Self::insert_fn(root.clone(), value),
            None => self.root = Some(Node::new(value).into()),
        }
    }

    fn insert_fn(node: Link<T>, value: T) {
        let node_value = node.borrow().value.clone();

        if value < node_value {
            let left = node.borrow().left.clone();
            match left {
                Some(node) => Self::insert_fn(node.clone(), value),
                None => {
                    node.borrow_mut().left = Some(Node::new_with_parent(value, node.clone()).into())
                }
            }
        } else if value > node_value {
            let right = node.borrow().right.clone();
            match right {
                Some(node) => Self::insert_fn(node.clone(), value),
                None => {
                    node.borrow_mut().right =
                        Some(Node::new_with_parent(value, node.clone()).into())
                }
            }
        }
    }

    pub fn traverse(&self) -> Vec<T> {
        let mut values = Vec::new();
        match &self.root {
            Some(root) => Self::traverse_fn(&mut values, root.clone()),
            None => (),
        }
        values
    }

    pub fn traverse_fn(values: &mut Vec<T>, current: Link<T>) {
        if let Some(left) = &current.borrow().left {
            Self::traverse_fn(values, left.clone());
        }

        values.push(current.borrow().value.clone());

        if let Some(right) = &current.borrow().right {
            Self::traverse_fn(values, right.clone());
        }
    }

    pub fn find(&self, value: &T) -> Option<Link<T>> {
        match self.root {
            Some(ref root) => Self::find_fn(root.clone(), value),
            None => None,
        }
    }

    fn find_fn(node: Link<T>, value: &T) -> Option<Link<T>> {
        if value == &node.borrow().value {
            return Some(node);
        } else if value < &node.borrow().value {
            return match node.borrow().left {
                Some(ref node) => Self::find_fn(node.clone(), value),
                None => None,
            };
        } else {
            return match node.borrow().right {
                Some(ref node) => Self::find_fn(node.clone(), value),
                None => None,
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn mock_tree<T: Ord + Clone>(nodes: Vec<T>) -> BSTree<T> {
        let mut tree = BSTree::new();
        for n in nodes {
            tree.insert(n);
        }
        tree
    }
    #[test]
    fn traverse_no_root() {
        let tree: BSTree<usize> = BSTree::new();
        assert_eq!(tree.traverse(), vec![]);
    }

    #[test]
    fn traverse_ok() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);
        assert_eq!(tree.traverse(), vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
    }

    #[test]
    fn find_no_root() {
        let tree: BSTree<usize> = BSTree::new();
        assert!(tree.find(&10).is_none());
    }
    #[test]
    fn find_ok() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);
        assert_eq!(tree.find(&3).unwrap().borrow().value, 3);
        assert_eq!(tree.find(&8).unwrap().borrow().value, 8);
        assert_eq!(tree.find(&14).unwrap().borrow().value, 14);
    }
}
