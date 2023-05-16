#![allow(unused)]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// A doubly-linked list.
///
/// The `LinkedList` allows pushing and popping elements at either end
/// in constant time.
struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    element: T,
}

impl<T> LinkedList<T> {
    /// Creates an empty `LinkedList`.
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    /// Adds an element first in the list.
    pub fn push_front(&mut self, elt: T) {
        let new_head = Node::new(elt);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    /// Appends an element to the back of a list.
    pub fn push_back(&mut self, elt: T) {
        let new_node = Node::new(elt);
        match self.tail.take() {
            Some(old_node) => {
                old_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_node);
                self.tail = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    /// Removes the first element and returns it, or `None` if the list is
    /// empty.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().element
        })
    }

    /// Removes the last element from a list and returns it, or `None` if
    /// it is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().element
        })
    }

    /// Provides a reference to the front element, or `None` if the list is
    /// empty.
    pub fn front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.element))
    }

    /// Provides a reference to the back element, or `None` if the list is
    /// empty.
    pub fn back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.element))
    }

    /// Provides a mutable reference to the front element, or `None` if the list
    /// is empty.
    pub fn front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_mut()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.element))
    }

    /// Provides a mutable reference to the back element, or `None` if the list
    /// is empty.
    pub fn back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_mut()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.element))
    }

    /// Consumes the list into an iterator yielding elements by value.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            element: value,
            next: None,
            prev: None,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basic_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn basic_back() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert!(list.front().is_none());
        assert!(list.back().is_none());
        assert!(list.front_mut().is_none());
        assert!(list.back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        assert_eq!(&*list.front().unwrap(), &2);
        assert_eq!(&mut *list.front_mut().unwrap(), &mut 2);
        assert_eq!(&*list.back().unwrap(), &1);
        assert_eq!(&mut *list.back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
