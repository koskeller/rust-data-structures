//! A double-ended queue (deque) implemented with a growable ring buffer.
//!
//! This queue has *O*(1) amortized inserts and removals from both ends of the
//! container. It also has *O*(1) indexing like a vector. The contained elements
//! are not required to be copyable, and the queue will be sendable if the
//! contained type is sendable.
#![allow(unused)]

const DEFAULT_CAPACITY: usize = 4;

/// A double-ended queue implemented with a growable ring buffer.
pub struct VecDeque<T> {
    head: usize,
    len: usize,
    buf: Vec<Option<T>>,
}

impl<T> VecDeque<T> {
    /// Creates an empty deque.
    pub fn new() -> Self {
        VecDeque::with_capacity(DEFAULT_CAPACITY)
    }

    /// Creates an empty deque with space for at least `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buf = Vec::new();
        buf.resize_with(capacity, Default::default);
        Self {
            head: 0,
            len: 0,
            buf,
        }
    }

    /// Returns the number of elements the deque can hold without
    /// reallocating.    
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Appends an element to the back of the deque.
    pub fn push_back(&mut self, value: T) {
        if self.is_full() {
            self.grow();
        }

        let next = (self.head + self.len) % self.capacity();
        self.buf[next] = Some(value);

        self.len += 1;
    }

    /// Prepends an element to the deque.
    pub fn push_front(&mut self, value: T) {
        if self.is_full() {
            self.grow();
        }

        self.head = if self.head == 0 {
            self.capacity() - 1
        } else {
            self.head - 1
        };

        self.buf[self.head] = Some(value);

        self.len += 1;
    }

    /// Removes the last element from the deque and returns it, or `None` if
    /// it is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            let i = (self.head + self.len) % self.capacity();
            self.buf[i].take()
        }
    }

    /// Removes the first element and returns it, or `None` if the deque is
    /// empty.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let old_head = self.buf[self.head].take();
            self.head = (self.head + 1) % self.capacity();
            self.len -= 1;
            old_head
        }
    }

    /// Returns `true` if the buffer is at full capacity.
    fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    // Double the buffer size.
    fn grow(&mut self) {
        self.buf.rotate_left(self.head);

        let new_capacity = std::cmp::max(self.capacity() * 2, 1);
        let mut new_buf = Vec::new();
        new_buf.resize_with(new_capacity, Default::default);

        let mut old_buf = std::mem::replace(&mut self.buf, new_buf);
        self.buf.splice(..old_buf.len(), old_buf);
        self.head = 0;
    }
}

#[cfg(test)]
mod test {
    use super::VecDeque;

    #[test]
    fn basic() {
        let mut deque = VecDeque::new();
        assert_eq!(deque.pop_front(), None);

        deque.push_back(1);
        assert_eq!(deque.pop_back(), Some(1));
        deque.push_back(1);
        assert_eq!(deque.pop_front(), Some(1));

        deque.push_front(1);
        assert_eq!(deque.pop_back(), Some(1));
        deque.push_front(1);
        assert_eq!(deque.pop_front(), Some(1));

        deque.push_front(1);
        deque.push_back(2);
        deque.push_front(0);
        assert_eq!(deque.pop_front(), Some(0));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
    }

    #[test]
    fn grow() {
        let mut deque = VecDeque::with_capacity(3);
        assert_eq!(deque.capacity(), 3);

        deque.push_back(0);
        deque.push_back(1);
        deque.pop_front();
        deque.push_back(2);
        deque.push_back(3);
        deque.push_back(4);
        assert_eq!(deque.capacity(), 6);
        assert_eq!(deque.buf[0], Some(1));
        assert_eq!(deque.buf[1], Some(2));
        assert_eq!(deque.buf[2], Some(3));
        assert_eq!(deque.buf[3], Some(4));
        assert_eq!(deque.buf[4], None);
    }
}
