#![allow(unused)]
use std::collections::VecDeque;

pub struct BSTree<T>
where
    T: Ord + Clone,
{
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
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

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl<T> From<Node<T>> for Option<Box<Node<T>>> {
    fn from(node: Node<T>) -> Self {
        Some(Box::new(node))
    }
}

impl<T> BSTree<T>
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

    // This code uses workaround for rust borrow checker issue discussed here:
    // https://github.com/rust-lang/rust/issues/54663
    // TODO this version won't work if parent is Tree itself.
    fn find_parent_mut(&mut self, value: T) -> Option<&mut Box<Node<T>>> {
        if self.root.is_none() {
            return None;
        }

        let mut node = self.root.as_mut().expect("checked by root.is_none()");
        loop {
            if value < node.value {
                if node.left.is_some() {
                    if value == node.left.as_ref().unwrap().value {
                        return Some(node);
                    } else {
                        node = node.left.as_mut().unwrap();
                        continue;
                    }
                } else {
                    return None;
                }
            } else if value > node.value {
                if node.right.is_some() {
                    if value == node.right.as_ref().unwrap().value {
                        return Some(node);
                    } else {
                        node = node.right.as_mut().unwrap();
                        continue;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        None
    }

    pub fn insert(&mut self, value: T) {
        let node = Node::new(value).into();
        match self.root {
            Some(ref mut current) => BSTree::insert_recursive(current, node),
            None => self.root = Some(node),
        }
    }

    pub fn delete(&mut self, value: &T) -> Option<T> {
        match self.root {
            Some(ref mut root) if &root.value == value => {
                let left = root.left.take();
                let right = root.right.take();
                let root = self.root.take().unwrap();

                if left.is_none() && right.is_none() {
                    self.root = None;
                } else if left.is_some() && right.is_none() {
                    self.root = left;
                } else if left.is_none() && right.is_some() {
                    self.root = right;
                } else {
                    self.root = right;
                    BSTree::insert_recursive(self.root.as_mut().unwrap(), left.unwrap());
                }
                return Some(root.value);
            }
            _ => (),
        }

        if let Some(parent) = self.find_parent_mut(value.clone()) {
            match parent.left {
                Some(ref mut target) if &target.value == value => {
                    let left = target.left.take();
                    let right = target.right.take();
                    let target = parent.left.take().unwrap();

                    if left.is_some() && right.is_none() {
                        parent.left = left;
                    } else if left.is_none() && right.is_some() {
                        parent.left = right;
                    } else if left.is_some() && right.is_some() {
                        parent.left = right;
                        BSTree::insert_recursive(parent.left.as_mut().unwrap(), left.unwrap());
                    }
                    return Some(target.value);
                }
                _ => (),
            }
            match parent.right {
                Some(ref mut target) if &target.value == value => {
                    let left = target.left.take();
                    let right = target.right.take();
                    let target = parent.right.take().unwrap();

                    if left.is_some() && right.is_none() {
                        parent.right = left;
                    } else if left.is_none() && right.is_some() {
                        parent.right = right;
                    } else if left.is_some() && right.is_some() {
                        parent.right = right;
                        BSTree::insert_recursive(parent.right.as_mut().unwrap(), left.unwrap());
                    }
                    return Some(target.value);
                }
                _ => (),
            }
        }
        None
    }

    fn insert_recursive(current: &mut Box<Node<T>>, node: Box<Node<T>>) {
        if node.value < current.value {
            match current.left {
                Some(ref mut child) => BSTree::insert_recursive(child, node),
                None => current.left = Some(node),
            }
        } else if node.value > current.value {
            match current.right {
                Some(ref mut child) => BSTree::insert_recursive(child, node),
                None => current.right = Some(node),
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
            BSTree::traverse_recursive_fn(&mut result, node);
        }
        result
    }

    fn traverse_recursive_fn(values: &mut Vec<T>, node: &Box<Node<T>>) {
        // For preorder traversal, uncomment:
        // values.push(node.value.clone());

        if let Some(ref node) = node.left {
            BSTree::traverse_recursive_fn(values, &node);
        }

        // For inorder traversal, uncomment:
        values.push(node.value.clone());

        if let Some(ref node) = node.right {
            BSTree::traverse_recursive_fn(values, &node);
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

impl<'a, T> IntoIterator for &'a BSTree<T>
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

    fn mock_tree<T: Ord + Clone>(nodes: Vec<T>) -> BSTree<T> {
        let mut tree = BSTree::new();
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

    #[test]
    fn find_parent() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let mut tree = mock_tree(nodes);

        assert!(tree.find_parent_mut(8).is_none());
        assert_eq!(tree.find_parent_mut(3).unwrap().value, 8);
        assert_eq!(tree.find_parent_mut(10).unwrap().value, 8);
        assert_eq!(tree.find_parent_mut(1).unwrap().value, 3);
        assert_eq!(tree.find_parent_mut(6).unwrap().value, 3);
        assert_eq!(tree.find_parent_mut(14).unwrap().value, 10);
        assert_eq!(tree.find_parent_mut(4).unwrap().value, 6);
        assert_eq!(tree.find_parent_mut(7).unwrap().value, 6);
        assert_eq!(tree.find_parent_mut(13).unwrap().value, 14);
    }

    #[test]
    fn delete_single_root() {
        let nodes = vec![8];
        let mut tree = mock_tree(nodes);
        assert_eq!(tree.delete(&8), Some(8));
        assert_eq!(tree.traverse_inorder_recursive(), vec![]);
    }

    #[test]
    fn delete_root_left() {
        let nodes = vec![8, 1];
        let mut tree = mock_tree(nodes);
        assert_eq!(tree.delete(&8), Some(8));
        assert_eq!(tree.traverse_inorder_recursive(), vec![1]);
    }

    #[test]
    fn delete_root_right() {
        let nodes = vec![8, 11];
        let mut tree = mock_tree(nodes);
        assert_eq!(tree.delete(&8), Some(8));
        assert_eq!(tree.traverse_inorder_recursive(), vec![11]);
    }

    #[test]
    fn delete_root() {
        let nodes = vec![8, 1, 11, 9];
        let mut tree = mock_tree(nodes);
        assert_eq!(tree.delete(&8), Some(8));
        assert_eq!(tree.traverse_inorder_recursive(), vec![1, 9, 11]);
        assert_eq!(tree.find_parent_mut(1).unwrap().value, 9);
    }

    #[test]
    fn delete() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let mut tree = mock_tree(nodes);

        assert_eq!(tree.delete(&8), Some(8));
        assert_eq!(
            tree.traverse_inorder_recursive(),
            vec![1, 3, 4, 6, 7, 10, 13, 14]
        );
        assert_eq!(tree.root.as_ref().unwrap().value, 10);

        assert_eq!(tree.delete(&1), Some(1));
        assert_eq!(
            tree.traverse_inorder_recursive(),
            vec![3, 4, 6, 7, 10, 13, 14]
        );
        assert_eq!(tree.delete(&14), Some(14));
        assert_eq!(tree.traverse_inorder_recursive(), vec![3, 4, 6, 7, 10, 13]);
    }
}
