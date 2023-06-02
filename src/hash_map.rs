#![allow(unused)]
use core::panic;
use std::hash::Hasher;

const DEFAULT_CAPACITY: usize = 10;

pub struct HashMap<K, V>
where
    K: std::hash::Hash,
{
    buckets: Vec<Option<(K, V)>>,
    len: usize,
}

impl<K, V> HashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    /// Creates an empty `HashMap`.
    pub fn new() -> Self {
        HashMap::with_capacity(DEFAULT_CAPACITY)
    }

    /// Creates an empty `HashMap` with space for at least `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buckets = Vec::new();
        buckets.resize_with(capacity, Default::default);
        Self { buckets, len: 0 }
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the capacity of the map.
    pub fn capacity(&self) -> usize {
        self.buckets.len()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // Grow underlying Vec if length is at 60% of capacity.
        let threshold = (self.buckets.len() * 6) / 10;
        if self.len() >= threshold {
            self.grow()
        }

        let index = self.find_or_find_insert_slot(&k);
        let old = std::mem::replace(&mut self.buckets[index], Some((k, v)));
        if let Some(elt) = old {
            Some(elt.1)
        } else {
            self.len += 1;
            None
        }
    }

    /// Returns the value corresponding to the supplied key.
    pub fn get(&self, k: K) -> Option<&V> {
        let index = self.find_or_find_insert_slot(&k);
        self.buckets[index].as_ref().map(|elt| &elt.1)
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    pub fn remove(&mut self, k: K) -> Option<(K, V)> {
        let index = self.find_or_find_insert_slot(&k);
        if let Some(elt) = self.buckets[index].take() {
            self.len -= 1;
            Some(elt)
        } else {
            None
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    pub fn iter(&self) {
        todo!()
    }

    /// An iterator visiting all key-value pairs in arbitrary order,
    /// with mutable references to the values.
    /// The iterator element type is `(&'a K, &'a mut V)`.
    pub fn iter_mut(&mut self) {
        todo!()
    }

    fn grow(&mut self) {
        let mut new_buckets = Vec::new();
        new_buckets.resize_with(self.buckets.len() * 2, Default::default);

        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);

        for elt in old_buckets {
            if let Some(elt) = elt {
                let (k, v) = elt;
                let index = self.find_or_find_insert_slot(&k);
                let old = std::mem::replace(&mut self.buckets[index], Some((k, v)));
            }
        }
    }

    fn hash(&self, k: &K) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        k.hash(&mut hasher);
        hasher.finish()
    }

    /// Searches for an index of element in the table, or a potential slot
    /// where that element could be inserted.
    fn find_or_find_insert_slot(&self, k: &K) -> usize {
        let hash = self.hash(k);
        let mut index = hash as usize % self.buckets.len();

        // TODO: handle case with infinity loop when no elemnt found.
        loop {
            match &self.buckets[index] {
                Some(elt) => {
                    if elt.0 == *k {
                        return index;
                    } else {
                        index = (index + 1) % self.buckets.len();
                        continue;
                    }
                }
                None => return index,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut hash_map = HashMap::new();

        // Make sure empty state is correct
        assert_eq!(hash_map.len(), 0);
        assert_eq!(hash_map.capacity(), DEFAULT_CAPACITY);
        assert_eq!(hash_map.get(1), None);
        assert_eq!(hash_map.remove(1), None);

        // Add element
        assert_eq!(hash_map.insert(1, 1), None);
        assert_eq!(hash_map.len(), 1);
        assert_eq!(hash_map.get(1), Some(&1));

        // Owerwrite
        assert_eq!(hash_map.insert(1, 2), Some(1));
        assert_eq!(hash_map.get(1), Some(&2));

        // Delete
        assert_eq!(hash_map.remove(1), Some((1, 2)));
        assert_eq!(hash_map.get(1), None);

        // Add multiple elements
        for i in 0..11 {
            assert_eq!(hash_map.insert(i, i), None);
        }
        assert_eq!(hash_map.len(), 11);
        assert_eq!(hash_map.capacity(), DEFAULT_CAPACITY * 2);
    }
}
