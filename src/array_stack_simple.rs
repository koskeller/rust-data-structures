#![allow(unused)]

const DEFAULT_CAPACITY: usize = 4;

pub struct Stack<T>
where
    T: Clone,
{
    elements: Vec<Option<T>>,
    len: usize,
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Stack::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: vec![None; capacity],
            len: 0,
        }
    }

    pub fn set(&mut self, i: usize, value: T) -> Option<T> {
        if i >= self.capacity() {
            self.resize(i);
        }

        let old = self.elements[i].take();
        self.elements[i] = Some(value);
        // We increase len only if we replace None element.
        // If we replace existing element, we should not change len.
        if old.is_none() {
            self.len += 1;
        }
        old
    }

    pub fn get(&mut self, i: usize) -> Option<&T> {
        if i >= self.capacity() {
            return None;
        }
        match &self.elements[i] {
            Some(v) => Some(&v),
            None => None,
        }
    }

    /// Adds an element to stack. Dynamically resizes if necessary,
    /// shifts existing elements to make room for the new one.
    pub fn add(&mut self, i: usize, value: T) {
        if i >= self.capacity() {
            self.resize(i);
        }

        match self.elements[i] {
            Some(_) => {
                self.shift_right(i);
                self.elements[i] = Some(value);
            }
            None => self.elements[i] = Some(value),
        }
        self.len += 1;
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.capacity() {
            return None;
        }
        self.len -= 1;
        self.elements[i].take()
    }

    fn resize(&mut self, n: usize) {
        let mut new_size = self.capacity();
        while n >= new_size {
            new_size = std::cmp::max(new_size * 2, 1);
        }
        let mut old = std::mem::replace(&mut self.elements, vec![None; new_size]);
        self.elements.splice(..old.len(), old);
    }

    fn shift_right(&mut self, mut i: usize) {
        let mut tmp = None;
        loop {
            if i >= self.capacity() {
                self.resize(i);
            }

            std::mem::swap(&mut tmp, &mut self.elements[i]);
            i += 1;

            if let None = tmp {
                break;
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn add() {
        let mut stack: Stack<u8> = Stack::new();

        stack.add(0, 13);
        assert_eq!(stack.get(0), Some(&13));
        assert_eq!(stack.len(), 1);

        // Should resize backing array
        stack.add(8, 13);
        assert_eq!(stack.get(8), Some(&13));
        assert_eq!(stack.capacity(), 16);
        assert_eq!(stack.len(), 2);

        stack.add(100, 13);
        assert_eq!(stack.get(100), Some(&13));
        assert_eq!(stack.capacity(), 128);
    }

    #[test]
    fn set() {
        let mut stack: Stack<u8> = Stack::new();

        stack.add(0, 13);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.get(0), Some(&13));

        // Should overwrite
        stack.set(0, 69);
        assert_eq!(stack.get(0), Some(&69));
        assert_eq!(stack.len(), 1);

        // Should resize & set
        stack.set(100, 13);
        assert_eq!(stack.get(100), Some(&13));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn remove() {
        let mut stack: Stack<u8> = Stack::new();

        stack.add(0, 13);
        assert_eq!(stack.get(0), Some(&13));
        assert_eq!(stack.len(), 1);

        assert_eq!(stack.remove(0), Some(13));
        assert_eq!(stack.get(0), None);
        assert_eq!(stack.len(), 0);

        assert_eq!(stack.remove(100), None);
        assert_eq!(stack.get(100), None);
    }

    #[test]
    fn shift_right() {
        let mut stack: Stack<u8> = Stack::with_capacity(4);
        assert_eq!(stack.capacity(), 4);
        stack.set(0, 0);
        stack.set(1, 1);
        stack.set(2, 2);
        stack.set(3, 3);
        stack.add(0, 13);
        assert_eq!(stack.get(0), Some(&13));
        assert_eq!(stack.get(1), Some(&0));
        assert_eq!(stack.get(2), Some(&1));
        assert_eq!(stack.get(3), Some(&2));
        assert_eq!(stack.get(4), Some(&3));
        assert_eq!(stack.capacity(), 8);
        assert_eq!(stack.len(), 5);

        let mut stack: Stack<u8> = Stack::new();
        stack.set(2, 2);
        stack.add(2, 13);
        assert_eq!(stack.get(2), Some(&13));
        assert_eq!(stack.get(3), Some(&2));
    }

    #[test]
    fn edge_cases() {
        let mut stack: Stack<u8> = Stack::with_capacity(0);
        assert_eq!(stack.get(11), None);
        assert_eq!(stack.set(0, 13), None);
    }
}
