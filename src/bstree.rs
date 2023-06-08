#![allow(unused)]
use std::collections::VecDeque;

pub struct Tree<T>
where
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

    pub fn get(&self, value: T) -> Option<&T> {
        match self.root {
            Some(ref node) => Self::find(value, node).map(|node| &node.value),
            None => None,
        }
    }

    fn find(value: T, node: &Box<Node<T>>) -> Option<&Box<Node<T>>> {
        if node.value == value {
            return Some(node);
        } else if node.value > value {
            match node.left {
                Some(ref n) => Self::find(value, n),
                None => None,
            }
        } else {
            match node.right {
                Some(ref n) => Self::find(value, n),
                None => None,
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => Tree::insert_recursive_fn(node, value),
            None => self.root = Node::new(value).into(),
        }
    }

    fn insert_recursive_fn(node: &mut Box<Node<T>>, value: T) {
        if value < node.value {
            match node.left {
                Some(ref mut child) => Tree::insert_recursive_fn(child, value),
                None => node.left = Node::new(value).into(),
            }
        } else if value > node.value {
            match node.right {
                Some(ref mut child) => Tree::insert_recursive_fn(child, value),
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

        let mut stack: Vec<&mut Box<Node<T>>> = Vec::new();
        stack.push(root);

        while let Some(node) = stack.pop() {
            if value < node.value {
                match node.left {
                    Some(ref mut n) => stack.push(n),
                    None => node.left = Node::new(value.clone()).into(),
                }
            } else if value > node.value {
                match node.right {
                    Some(ref mut n) => stack.push(n),
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

    pub fn traverse_inorder_recursive(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }
        let mut result = Vec::new();
        if let Some(ref node) = self.root {
            Tree::traverse_recursive_fn(&mut result, node);
        }
        result
    }

    fn traverse_recursive_fn(values: &mut Vec<T>, node: &Box<Node<T>>) {
        // For preorder traversal, uncomment:
        // values.push(node.value.clone());

        if let Some(ref node) = node.left {
            Tree::traverse_recursive_fn(values, &node);
        }

        // For inorder traversal, uncomment:
        values.push(node.value.clone());

        if let Some(ref node) = node.right {
            Tree::traverse_recursive_fn(values, &node);
        }

        // For post order traversal, uncomment:
        // values.push(node.value.clone());
    }

    pub fn traverse_inorder_iterative(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = self.root.as_ref();

        while !stack.is_empty() || current.is_some() {
            while let Some(node) = current {
                stack.push(node);
                current = node.left.as_ref();
            }

            if let Some(node) = stack.pop() {
                result.push(node.value.clone());
                current = node.right.as_ref();
            }
        }

        result
    }

    pub fn traverse_pre_order_iteratively(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut stack = Vec::new();
        stack.push(self.root.as_ref().expect("guarded by root.is_none()"));

        while let Some(node) = stack.pop() {
            result.push(node.value.clone());
            if let Some(ref node) = node.right {
                stack.push(node);
            }
            if let Some(ref node) = node.left {
                stack.push(node);
            }
        }

        result
    }

    pub fn iter(&self) -> Iter<T> {
        let current = self.root.as_ref().map(|r| r);
        Iter {
            stack: Vec::new(),
            current,
        }
    }
}

pub struct Iter<'a, T> {
    stack: Vec<&'a Box<Node<T>>>,
    current: Option<&'a Box<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.current, &mut self.stack) {
            (None, q) if q.is_empty() => None,
            (None, q) => {
                let node = q.pop().expect("guarded by q.is_empty() before");
                self.current = node.right.as_ref();
                Some(&node.value)
            }
            (Some(node), q) => {
                self.stack.push(node);
                self.current = node.left.as_ref();
                self.next()
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a Tree<T>
where
    T: Ord + Clone,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let current = self.root.as_ref();
        Iter {
            stack: Vec::new(),
            current,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn mock_tree<T: Ord + Clone>(nodes: Vec<T>) -> Tree<T> {
        let mut tree = Tree::new();
        for n in nodes {
            tree.insert(n);
        }
        tree
    }

    #[test]
    fn traverse_level_order() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes.clone());
        assert_eq!(tree.traverse_level_order(), nodes);
    }

    #[test]
    fn traverse_inorder_recursive() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);
        assert_eq!(
            tree.traverse_inorder_recursive(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn traverse_inorder_iterative() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);
        assert_eq!(
            tree.traverse_inorder_iterative(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn traverse_pre_order() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);
        assert_eq!(
            tree.traverse_pre_order_iteratively(),
            vec![8, 3, 1, 6, 4, 7, 10, 14, 13]
        );
    }

    #[test]
    fn iter() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);
        let nums = vec![1, 3, 4, 6, 7, 8, 10, 13, 14];
        let want: Vec<&i32> = nums.iter().collect();
        assert_eq!(tree.iter().collect::<Vec<&i32>>(), want);
    }

    #[test]
    fn into_iter() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);

        let mut got = Vec::new();
        for n in &tree {
            got.push(n);
        }

        let nums = vec![1, 3, 4, 6, 7, 8, 10, 13, 14];
        let want: Vec<&i32> = nums.iter().collect();
        assert_eq!(got, want);
    }

    #[test]
    fn find() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);
        assert!(tree.get(8).is_some());
        assert!(tree.get(13).is_some());
        assert!(tree.get(0).is_none());
        assert!(tree.get(99).is_none());
    }
}
