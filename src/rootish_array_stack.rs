#![allow(unused)]

pub struct RootishStack<T> {
    len: usize,
    blocks: Vec<Vec<T>>,
}

impl<T> RootishStack<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            blocks: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) {
        let (block, index) = (self.get_block(), self.get_index());
        if index == 0 {
            self.blocks
                .push(Vec::with_capacity(std::cmp::max(block, 1)));
            self.blocks[block].push(value);
        } else {
            self.blocks[block].push(value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            let block = self.get_block();
            self.blocks[block].pop()
        }
    }

    fn get_block(&self) -> usize {
        ((-3.0 + f64::sqrt(9.0 + 8.0 * self.len as f64)) / 2.0).ceil() as usize
    }

    fn get_index(&self) -> usize {
        let block = self.get_block();
        self.len - block * (block + 1) / 2
    }
}

#[cfg(test)]
mod test {
    use super::RootishStack;

    #[test]
    fn basic() {
        let mut stack = RootishStack::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.get_block(), 0);
        assert_eq!(stack.get_index(), 0);

        stack.push(0);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.get_block(), 1);
        assert_eq!(stack.get_index(), 0);

        stack.push(1);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.get_block(), 1);
        assert_eq!(stack.get_index(), 1);

        stack.push(2);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.get_block(), 2);
        assert_eq!(stack.get_index(), 0);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), Some(0));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);
    }
}
