#![allow(unused)]

#[derive(Debug)]
pub struct LinkedList<T: PartialOrd>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self(None)
    }

    pub fn prepend(&mut self, value: T) {
        let old_head = self.0.take();
        self.0 = Some((value, Box::new(LinkedList(old_head))))
    }

    pub fn append(&mut self, value: T) {
        match self.0 {
            Some((_, ref mut next)) => next.append(value),
            None => self.0 = Some((value, Box::new(LinkedList(None)))),
        }
    }

    pub fn insert_sorted(&mut self, value: T) {
        todo!()
    }

    pub fn find(&self, value: T) -> Option<&LinkedList<T>> {
        match &self.0 {
            Some((v, ref next)) => {
                if *v == value {
                    Some(self)
                } else {
                    next.find(value)
                }
            }
            None => None,
        }
    }
}
