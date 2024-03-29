#![allow(unused)]
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct BSTree<T> {
    root: Option<Link<T>>,
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

    pub fn min(&self) -> Option<Rc<RefCell<Node<T>>>> {
        match &self.root {
            Some(root) => {
                let mut current = root.clone();
                loop {
                    // Borrow checker fight
                    let temp = {
                        let borrow = current.borrow();
                        borrow.left.clone()
                    };
                    match temp {
                        Some(node) => current = node.clone(),
                        None => return Some(current.clone()),
                    }
                }
            }
            None => None,
        }
    }

    // Breath first traversal
    pub fn height(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }

        let mut height = 0;
        let mut deq = VecDeque::new();
        deq.push_front(self.root.as_ref().unwrap().clone());

        while !deq.is_empty() {
            height += 1;
            for _ in 0..deq.len() {
                if let Some(node) = deq.pop_front() {
                    if let Some(ref left) = node.borrow().left {
                        deq.push_back(left.clone());
                    }
                    if let Some(ref right) = node.borrow().right {
                        deq.push_back(right.clone());
                    }
                }
            }
        }

        height
    }

    pub fn is_balanced(&self) -> bool {
        let (balanced, height) = Self::check_balanced(self.root.clone());
        balanced
    }

    fn check_balanced(node: Option<Link<T>>) -> (bool, i32) {
        match node {
            Some(node) => {
                let (left_balanced, left_height) = Self::check_balanced(node.borrow().left.clone());
                if !left_balanced {
                    return (false, 0);
                }

                let (right_balanced, right_height) =
                    Self::check_balanced(node.borrow().right.clone());
                if !right_balanced {
                    return (false, 0);
                }

                (
                    (left_height - right_height).abs() < 2,
                    std::cmp::max(left_height, right_height) + 1,
                )
            }
            None => (true, -1),
        }
    }

    pub fn is_valid(&self) -> bool {
        match &self.root {
            Some(node) => Self::check_valid(node.clone()),
            None => true,
        }
    }

    fn check_valid(node: Link<T>) -> bool {
        if let Some(left) = &node.borrow().left {
            if left.borrow().value >= node.borrow().value {
                return false;
            }

            let left = Self::check_valid(left.clone());
            if left == false {
                return false;
            }
        }

        if let Some(right) = &node.borrow().right {
            if right.borrow().value <= node.borrow().value {
                return false;
            }

            let right = Self::check_valid(right.clone());
            if right == false {
                return false;
            }
        }

        return true;
    }
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

use super::linked_list_simple::List;

fn merge_to_ll<T: Ord + Clone>(tree1: &BSTree<T>, tree2: &BSTree<T>) -> List<T> {
    let mut list: List<T> = List::new();

    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();
    let mut current1 = tree1.root.as_ref().map(Clone::clone);
    let mut current2 = tree2.root.as_ref().map(Clone::clone);

    while !stack1.is_empty() || current1.is_some() {
        while let Some(node) = current1 {
            stack1.push(node.clone());
            current1 = node.borrow().left.as_ref().map(Clone::clone);
        }
        while let Some(node) = current2 {
            stack2.push(node.clone());
            current2 = node.borrow().left.as_ref().map(Clone::clone);
        }

        match stack1.get(stack1.len() - 1) {
            Some(node1) => match stack2.get(stack2.len() - 1) {
                Some(node2) => {
                    if node1.borrow().value < node2.borrow().value {
                        if let Some(node) = stack1.pop() {
                            list.push(node.borrow().value.clone());
                            current1 = node.borrow().right.as_ref().map(Clone::clone);
                        }
                    } else {
                        if let Some(node) = stack2.pop() {
                            current2 = node.borrow().right.as_ref().map(Clone::clone);
                            list.push(node.borrow().value.clone());
                        }
                    }
                }
                None => {
                    if let Some(node) = stack1.pop() {
                        current1 = node.borrow().right.as_ref().map(Clone::clone);
                        list.push(node.borrow().value.clone());
                    }
                }
            },
            None => {
                if let Some(node) = stack2.pop() {
                    current2 = node.borrow().right.as_ref().map(Clone::clone);
                    list.push(node.borrow().value.clone());
                }
            }
        }
    }

    while !stack2.is_empty() || current2.is_some() {
        while let Some(node) = current2 {
            stack2.push(node.clone());
            current2 = node.borrow().left.as_ref().map(Clone::clone);
        }

        if let Some(node) = stack2.pop() {
            current2 = node.borrow().right.as_ref().map(Clone::clone);
            list.push(node.borrow().value.clone());
        }
    }

    list
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

    #[test]
    fn min() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let tree = mock_tree(nodes);
        assert_eq!(tree.min().unwrap().borrow().value, 1);
    }

    #[test]
    fn min_is_root() {
        let nodes = vec![1, 3];
        let tree = mock_tree(nodes);
        assert_eq!(tree.min().unwrap().borrow().value, 1);
    }

    #[test]
    fn height_0() {
        let nodes = vec![1];
        let tree = mock_tree(nodes);
        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn height_3() {
        let nodes = vec![3, 2, 1];
        let tree = mock_tree(nodes);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn height_1() {
        let nodes = vec![1, 3];
        let tree = mock_tree(nodes);
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn height_5() {
        let nodes = vec![1, 2, 3, 4, 5, 6];
        let tree = mock_tree(nodes);
        assert_eq!(tree.height(), 6);
    }

    #[test]
    fn merge_to_linked_list() {
        let tree1 = mock_tree(vec![4, 2, 1]);
        let tree2 = mock_tree(vec![5, 3, 6]);

        let mut list = merge_to_ll(&tree1, &tree2);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn check_balanced_empty() {
        let nodes: Vec<usize> = vec![];
        let tree = mock_tree(nodes);
        assert_eq!(BSTree::check_balanced(tree.root.clone()).0, true);
    }

    #[test]
    fn check_balanced_balanced() {
        let nodes = vec![2, 1, 2];
        let tree = mock_tree(nodes);
        assert_eq!(BSTree::check_balanced(tree.root.clone()).0, true);
    }

    #[test]
    fn check_balanced_not_perfect() {
        let nodes = vec![4, 3, 2, 1, 5];
        let tree = mock_tree(nodes);
        assert_eq!(BSTree::check_balanced(tree.root.clone()).0, false);
    }

    #[test]
    fn check_balanced_right_heavy() {
        let nodes = vec![1, 2, 3, 4, 5, 6];
        let tree = mock_tree(nodes);
        assert_eq!(BSTree::check_balanced(tree.root.clone()).0, false);
    }

    #[test]
    fn is_valid_empty() {
        let nodes: Vec<usize> = vec![];
        let tree = mock_tree(nodes);
        assert_eq!(tree.is_valid(), true);
    }

    #[test]
    fn is_valid_ok() {
        let nodes = vec![1, 2, 3, 4, 5, 6];
        let tree = mock_tree(nodes);
        assert_eq!(tree.is_valid(), true);
    }

    #[test]
    fn is_valid_not_valid() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let mut tree = mock_tree(nodes);
        tree.root.as_mut().unwrap().borrow_mut().value = 0;
        assert_eq!(tree.is_valid(), false);
    }

    #[test]
    fn is_valid_not_valid_left() {
        let nodes = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let mut tree = mock_tree(nodes);
        tree.root
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left
            .as_mut()
            .unwrap()
            .borrow_mut()
            .value = 0;
        assert_eq!(tree.is_valid(), false);
    }
}
