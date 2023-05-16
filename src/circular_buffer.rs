#![allow(unused)]

const DEFAULT_CAPACITY: usize = 5;

pub struct Buffer<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Buffer::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut buf = Vec::new();
        buf.resize_with(capacity, Default::default);
        Self {
            buf,
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let mut next = self.head + 1;
        if next >= self.capacity() {
            next = 0;
        }

        self.buf[self.head] = Some(value);
        self.head = next;

        if next == self.tail {
            // buffer if full, move the tail
            self.tail += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head == self.tail {
            return None;
        }

        let result = self.buf[self.tail].take();

        let mut next = self.tail + 1;
        if next >= self.capacity() {
            next = 0;
        }
        self.tail = next;

        result
    }

    fn capacity(&self) -> usize {
        self.buf.len()
    }
}

#[cfg(test)]
mod test {
    use super::Buffer;

    #[test]
    fn basic() {
        let mut buf = Buffer::new();
        assert_eq!(buf.capacity(), 5);

        assert_eq!(buf.pop(), None);
        buf.push(1);
        assert_eq!(buf.pop(), Some(1));
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        buf.push(5); // <- should overwrite 1, making tail == 2
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), Some(4));
        assert_eq!(buf.pop(), Some(5));
        assert_eq!(buf.pop(), None);
        buf.push(1);
        assert_eq!(buf.pop(), Some(1));
    }
}
