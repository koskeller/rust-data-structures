#![allow(unused)]

// Enum will need to store some integer to indicate which variant of the enum it
// represents (D1, D2, .. Dn). This is the tag of the enum. It will also need
// enough space to store the largest of T1, T2, .. Tn (plus some extra space
// to satisfy alignment requirements).
//
// The big takeaway here is that even though Empty is a single bit of information,
// it necessarily consumes enough space for a pointer and an element, because it
// has to be ready to become an Elem at any time. Therefore the first layout heap
// allocates an extra element that's just full of junk, consuming a bit more space
// than the second layout.
// enum List<T> {
//     Empty,
//     Elem(T, Box<List<T>>),
// }

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new = Box::new(Node {
            value,
            next: self.head.take(),
            // next: std::mem::replace(&mut self.head, None),
        });
        self.head = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
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
        let mut cur_node = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_node {
            cur_node = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        // Can't do that now:
        // assert_eq!(list.pop(), Some(3));
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        assert_eq!(list.pop(), Some(3));
    }
}
