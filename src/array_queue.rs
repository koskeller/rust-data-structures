#![allow(unused)]

const DEFAULT_CAPACITY: usize = 4;

pub struct Queue<T>
where
    T: Clone,
{
    elements: Vec<Option<T>>,
    next: usize,
    len: usize,
}

impl<T> Queue<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Queue::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: vec![None; capacity],
            next: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        match self.elements[self.next] {
            Some(_) => {
                self.elements.rotate_left(self.next);
                self.next = self.elements.len();
                self.resize();
                self.elements[self.next] = Some(value);
            }
            None => {
                self.elements[self.next] = Some(value);
                self.next = self.get_next();
            }
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        match self.elements[self.next] {
            Some(_) => {
                self.len -= 1;
                self.elements[self.next].take()
            }
            None => {
                let index = (self.next + self.capacity() - self.len) % self.capacity();
                self.len -= 1;
                self.elements[index].take()
            }
        }
    }

    fn get_next(&self) -> usize {
        (self.next + 1) % self.elements.len()
    }

    fn resize(&mut self) {
        let new_capacity = std::cmp::max(self.elements.len() * 2, 1);
        let old = std::mem::replace(&mut self.elements, vec![None; new_capacity]);
        self.elements.splice(..old.len(), old);
    }

    fn capacity(&self) -> usize {
        self.elements.len()
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
