#![allow(unused)]

use std::collections::VecDeque;

pub struct Tree<T>
where
    // TODO: remove Clone
    T: Ord + Clone,
{
    root: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> From<Node<T>> for Option<Box<Node<T>>> {
    fn from(node: Node<T>) -> Self {
        Some(Box::new(node))
    }
}

impl<T> Tree<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => Tree::insert_recursive(node, value),
            None => self.root = Node::new(value).into(),
        }
    }

    pub fn insert_recursive(node: &mut Box<Node<T>>, value: T) {
        if value < node.value {
            match node.left {
                Some(ref mut child) => Tree::insert_recursive(child, value),
                None => node.left = Node::new(value).into(),
            }
        } else if value > node.value {
            match node.right {
                Some(ref mut child) => Tree::insert_recursive(child, value),
                None => node.right = Node::new(value).into(),
            }
        }
    }

    pub fn insert_iterative(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Node::new(value).into();
            return;
        }

        let root = self.root.as_mut().expect("checked by root.is_none()");

        let mut q: Vec<&mut Box<Node<T>>> = Vec::new();
        q.push(root);

        while let Some(node) = q.pop() {
            if value < node.value {
                match node.left {
                    Some(ref mut n) => q.push(n),
                    None => node.left = Node::new(value.clone()).into(),
                }
            } else if value > node.value {
                match node.right {
                    Some(ref mut n) => q.push(n),
                    None => node.right = Node::new(value.clone()).into(),
                }
            }
        }
    }

    pub fn traverse_level_order(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut deque: VecDeque<&Box<Node<T>>> = VecDeque::new();
        let root = self.root.as_ref().expect("checked by root.is_none()");
        result.push(root.value.clone());
        deque.push_back(root);

        while !deque.is_empty() {
            for _ in 0..deque.len() {
                if let Some(node) = deque.pop_front() {
                    if let Some(ref node) = node.left {
                        result.push(node.value.clone());
                        deque.push_back(node);
                    }
                    if let Some(ref node) = node.right {
                        result.push(node.value.clone());
                        deque.push_back(node);
                    }
                }
            }
        }

        result
    }

    pub fn traverse_inorder(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }
        let mut result = Vec::new();
        if let Some(ref node) = self.root {
            Tree::traverse_inorder_recursive(&mut result, node);
        }
        result
    }

    pub fn traverse_inorder_recursive(values: &mut Vec<T>, node: &Box<Node<T>>) {
        // For preorder traversal, uncomment:
        // values.push(node.value.clone());

        if let Some(ref node) = node.left {
            Tree::traverse_inorder_recursive(values, &node);
        }

        // For inorder traversal, uncomment:
        values.push(node.value.clone());

        if let Some(ref node) = node.right {
            Tree::traverse_inorder_recursive(values, &node);
        }

        // For post order traversal, uncomment:
        // values.push(node.value.clone());
    }

    pub fn traverse_inorder_iterative(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut queue = Vec::new();
        let mut current = self.root.as_ref();

        while !queue.is_empty() || current.is_some() {
            while let Some(node) = current {
                queue.push(node);
                current = node.left.as_ref();
            }

            if let Some(node) = queue.pop() {
                result.push(node.value.clone());
                current = node.right.as_ref();
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn traverse_level_order() {
        let mut tree = Tree::new();
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        for n in nodes.clone() {
            tree.insert(n);
        }
        assert_eq!(tree.traverse_level_order(), nodes);
    }

    #[test]
    fn traverse_inorder() {
        let mut tree = Tree::new();
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        for n in nodes {
            tree.insert(n);
        }
        assert_eq!(tree.traverse_inorder(), vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
    }

    #[test]
    fn traverse_inorder_iterative() {
        let mut tree = Tree::new();
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        for n in nodes {
            tree.insert(n);
        }
        assert_eq!(
            tree.traverse_inorder_iterative(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }
}
