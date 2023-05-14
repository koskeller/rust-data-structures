#![allow(unused)]
// Use Arc for multi-threaded cases.
use std::sync::Arc;

// Or use Rc instead for single-threaded.
// use std::rc::Rc;

type Link<T> = Option<Arc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    next: Link<T>,
    value: T,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&self, value: T) -> Self {
        Self {
            head: Some(Arc::new(Node {
                next: self.head.clone(),
                value,
            })),
        }
    }

    pub fn pop(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.peek(), None);

        let list = list.push(1).push(2).push(3);
        assert_eq!(list.peek(), Some(&3));

        let list = list.pop();
        assert_eq!(list.peek(), Some(&2));

        let list = list.pop();
        assert_eq!(list.peek(), Some(&1));

        let list = list.pop();
        assert_eq!(list.peek(), None);

        let list = list.pop();
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().push(1).push(2).push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
