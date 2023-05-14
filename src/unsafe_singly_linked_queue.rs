#![allow(unused)]
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    next: Link<T>,
    value: T,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // TODO: Work in progress
    // pub fn push(&mut self, value: T) {
    //     let new_tail = Box::new(Node { next: None, value });
    // }
}
