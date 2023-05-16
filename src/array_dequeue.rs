#![allow(unused)]

pub struct Deque<T> {
    buf: Vec<Option<T>>,
    start: usize,
    len: usize,
}

impl<T> Deque<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buf = Vec::new();
        buf.resize_with(capacity, Default::default);
        Self {
            buf,
            start: 0,
            len: 0,
        }
    }
}
