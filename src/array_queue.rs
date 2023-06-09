#![allow(unused)]

const DEFAULT_CAPACITY: usize = 4;

pub struct Queue<T> {
    buf: Vec<Option<T>>,
    start: usize,
    len: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut buf = Vec::new();
        buf.resize_with(capacity, Default::default);
        Self {
            buf,
            start: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len >= self.capacity() {
            self.resize()
        }
        let next = (self.start + self.len) % self.capacity();
        self.buf[next] = Some(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let result = self.buf[self.start].take();
        self.len -= 1;
        self.start = (self.start + 1) % self.capacity();
        result
    }

    fn resize(&mut self) {
        self.buf.rotate_left(self.start);
        let new_capacity = std::cmp::max(self.capacity() * 2, 1);
        let mut buf = Vec::new();
        buf.resize_with(new_capacity, Default::default);
        let mut old = std::mem::replace(&mut self.buf, buf);
        self.buf.splice(..old.len(), old);
        self.start = 0;
    }

    fn capacity(&self) -> usize {
        self.buf.len()
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn basic() {
        let mut queue = Queue::new();
        assert_eq!(queue.pop(), None);

        queue.push(0);
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.pop(), Some(0));
        queue.push(3);
        queue.push(4);
        assert_eq!(queue.pop(), Some(1));
        queue.push(5);
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        queue.push(6);
        queue.push(7);
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), Some(6));
        assert_eq!(queue.pop(), Some(7));
        assert_eq!(queue.pop(), None);
    }
}
