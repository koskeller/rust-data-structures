#![allow(unused)]
/// Stack  that supports .push(x), .pop(), and .min(),
/// which returns the minimum element of S. All operations run in constant time.

pub struct Stack<T> {
    stack: Vec<T>,
    min_stack: Vec<T>,
}

impl<T> Stack<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value.clone());
        match self.min_stack.last() {
            Some(last) if &value <= last => self.min_stack.push(value),
            Some(last) => (),
            None => self.min_stack.push(value),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let value = self.stack.pop();
        if value == self.min_stack.last().cloned() {
            self.min_stack.pop();
        }
        value
    }

    pub fn min(&self) -> Option<&T> {
        self.min_stack.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);

        assert_eq!(stack.min(), Some(&3));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);

        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.min(), Some(&3));

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.min(), Some(&5));

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.min(), None);

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_min() {
        let mut stack = Stack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);

        assert_eq!(stack.min(), Some(&3));

        stack.pop();
        assert_eq!(stack.min(), Some(&3));

        stack.pop();
        assert_eq!(stack.min(), Some(&5));

        stack.push(2);
        assert_eq!(stack.min(), Some(&2));
    }
}
