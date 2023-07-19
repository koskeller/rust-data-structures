#![allow(unused)]

/// Complexity:
/// insert - O(logn)
/// pop - O(logn)
pub struct MinHeap<T: Ord> {
    heap: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        self.heap.push(value);
        self.bubble_up(self.heap.len() - 1);
    }

    fn bubble_up(&mut self, index: usize) {
        let mut child_index = index;
        let mut parent_index = (child_index.saturating_sub(1)) / 2;

        while child_index > 0 && self.heap[parent_index] > self.heap[child_index] {
            self.heap.swap(parent_index, child_index);
            child_index = parent_index;
            parent_index = (child_index.saturating_sub(1)) / 2;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let min_value = self.heap.swap_remove(0);
        self.buble_down(0);
        Some(min_value)
    }

    fn buble_down(&mut self, index: usize) {
        let len = self.heap.len();
        let mut parent_index = index;

        loop {
            let left_index = 2 * parent_index + 1;
            let right_index = 2 * parent_index + 2;
            let mut smallest_index = parent_index;

            if left_index < len && self.heap[left_index] < self.heap[parent_index] {
                smallest_index = left_index;
            }

            if right_index < len && self.heap[right_index] < self.heap[smallest_index] {
                smallest_index = right_index;
            }

            if smallest_index != parent_index {
                self.heap.swap(smallest_index, parent_index);
                parent_index = smallest_index;
            } else {
                break;
            }
        }
    }
}

impl<T> From<Vec<T>> for MinHeap<T>
where
    T: Ord,
{
    fn from(value: Vec<T>) -> Self {
        let mut heap = Self::new();
        for v in value {
            heap.insert(v);
        }
        heap
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_single() {
        let mut heap: MinHeap<_> = vec![0].into();
        assert_eq!(heap.heap, vec![0]);
    }

    #[test]
    fn insert_simple() {
        let mut heap: MinHeap<_> = vec![2, 1].into();
        assert_eq!(heap.heap, vec![1, 2]);
    }

    #[test]
    fn insert_negative() {
        let mut heap: MinHeap<_> = vec![2, 1, -2, -5, 0].into();
        assert_eq!(heap.heap, vec![-5, -2, 1, 2, 0]);
    }

    #[test]
    fn insert_many() {
        let mut heap: MinHeap<_> = vec![8, 3, 10, 1, 6, 14, 4, 7, 13].into();
        assert_eq!(heap.heap, vec![1, 3, 4, 7, 6, 14, 10, 8, 13]);
    }

    #[test]
    fn min_empty() {
        let mut heap: MinHeap<usize> = vec![].into();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn min_many() {
        let mut heap: MinHeap<_> = vec![8, 3, -1, 0, 10, 1].into();
        assert_eq!(heap.pop(), Some(-1));
        assert_eq!(heap.pop(), Some(0));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), None);
    }
}
