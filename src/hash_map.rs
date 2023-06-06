#![allow(unused)]
use core::panic;
use std::hash::{Hash, Hasher};

const DEFAULT_CAPACITY: usize = 10;

pub struct HashMap<K, V>
where
    K: Hash,
{
    buckets: Vec<Option<(K, V)>>,
    len: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    /// Creates an empty `HashMap`.
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
        }
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
        let threshold = self.buckets.len() * 6 / 10;
        if self.buckets.is_empty() || self.len() >= threshold {
            self.grow()
        }

        let index = self.find_or_find_insert_slot(&k)?;
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
        let index = self.find_or_find_insert_slot(&k)?;
        self.buckets[index].as_ref().map(|elt| &elt.1)
    }

    /// Removes a key from themap, returning the value at the key if the key
    /// was previously in the map.
    pub fn remove(&mut self, k: K) -> Option<(K, V)> {
        let index = self.find_or_find_insert_slot(&k)?;
        if let Some(elt) = self.buckets[index].take() {
            self.len -= 1;
            Some(elt)
        } else {
            None
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            index: 0,
            buckets: &self.buckets,
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order,
    /// with owned values.
    /// The iterator element type is `(&'a K, &'a mut V)`.
    pub fn into_iter(self) -> IntoIter<K, V> {
        IntoIter {
            index: 0,
            buckets: self.buckets,
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, k: K) -> bool {
        self.get(k).is_some()
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry<'a>(&'a mut self, k: K) -> Entry<'a, K, V> {
        // Grow underlying Vec if length is at 60% of capacity.
        let threshold = self.buckets.len() * 6 / 10;
        if self.buckets.is_empty() || self.len() >= threshold {
            self.grow()
        }

        let index = self.find_or_find_insert_slot(&k).unwrap();
        match self.buckets[index] {
            Some(ref mut elt) => Entry::Occupied(OccupiedEntry { index, map: self }),
            None => Entry::Vacant(VacantEntry {
                key: k,
                index,
                map: self,
            }),
        }
    }

    fn grow(&mut self) {
        let size = match self.buckets.len() {
            0 => DEFAULT_CAPACITY,
            n => self.buckets.len() * 2,
        };

        let mut new_buckets = Vec::new();
        new_buckets.resize_with(size, Default::default);

        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);

        for elt in old_buckets {
            if let Some(elt) = elt {
                let (k, v) = elt;
                let index = self
                    .find_or_find_insert_slot(&k)
                    .expect("we allocated at least DEFAULT_CAPACITY, can't be empty");
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
    fn find_or_find_insert_slot(&self, k: &K) -> Option<usize> {
        if self.buckets.is_empty() {
            return None;
        }

        let hash = self.hash(k);
        let mut index = hash as usize % self.buckets.len();

        // TODO: handle case with infinity loop when no elemnt found.
        loop {
            match &self.buckets[index] {
                Some(elt) => {
                    if elt.0 == *k {
                        return Some(index);
                    } else {
                        index = (index + 1) % self.buckets.len();
                        continue;
                    }
                }
                None => return Some(index),
            }
        }
    }
}

pub struct OccupiedEntry<'a, K, V>
where
    K: Hash,
{
    index: usize,
    map: &'a mut HashMap<K, V>,
}

pub struct VacantEntry<'a, K, V>
where
    K: Hash,
{
    key: K,
    index: usize,
    map: &'a mut HashMap<K, V>,
}

impl<'a, K, V> VacantEntry<'a, K, V>
where
    K: Hash,
{
    pub fn insert(self, v: V) -> &'a mut V {
        self.map.buckets[self.index].replace((self.key, v));
        self.map.len += 1;
        &mut self.map.buckets[self.index].as_mut().unwrap().1
    }
}

pub enum Entry<'a, K, V>
where
    K: Hash,
{
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash,
{
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => &mut e.map.buckets[e.index].as_mut().unwrap().1,
            Entry::Vacant(e) => e.insert(value),
        }
    }

    pub fn or_insert_with<F>(self, maker: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(e) => &mut e.map.buckets[e.index].as_mut().unwrap().1,
            Entry::Vacant(e) => e.insert(maker()),
        }
    }

    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with(Default::default)
    }
}

pub struct Iter<'a, K, V> {
    index: usize,
    buckets: &'a Vec<Option<(K, V)>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.buckets.len() {
            match self.buckets[self.index] {
                Some((ref k, ref v)) => {
                    self.index += 1;
                    return Some((k, v));
                }
                None => {
                    self.index += 1;
                    continue;
                }
            }
        }
        None
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V>
where
    K: Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            index: 0,
            buckets: &self.buckets,
        }
    }
}

pub struct IntoIter<K, V> {
    index: usize,
    buckets: Vec<Option<(K, V)>>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.buckets.len() {
            match self.buckets[self.index].take() {
                Some(elt) => {
                    self.index += 1;
                    return Some(elt);
                }
                None => {
                    self.index += 1;
                    continue;
                }
            }
        }
        None
    }
}

impl<K, V> IntoIterator for HashMap<K, V>
where
    K: Hash,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            index: 0,
            buckets: self.buckets,
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
        assert_eq!(hash_map.capacity(), 0);
        assert_eq!(hash_map.get(1), None);
        assert_eq!(hash_map.remove(1), None);

        // Add element
        assert_eq!(hash_map.insert(1, 1), None);
        assert_eq!(hash_map.len(), 1);
        assert_eq!(hash_map.capacity(), DEFAULT_CAPACITY);
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

    #[test]
    fn string() {
        let mut hash_map = HashMap::new();

        hash_map.insert("Hello", "world");
        hash_map.insert(&"Hello", &"world");
    }

    #[test]
    fn iter() {
        let mut hash_map = HashMap::new();

        hash_map.insert("a", 1);
        hash_map.insert("b", 2);
        hash_map.insert("c", 3);

        assert_eq!(hash_map.iter().count(), 3);

        for (&k, &v) in &hash_map {
            match k {
                "a" => assert_eq!(v, 1),
                "b" => assert_eq!(v, 2),
                "c" => assert_eq!(v, 3),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn into_iter() {
        let mut hash_map = HashMap::new();

        hash_map.insert("a", 1);
        hash_map.insert("b", 2);
        hash_map.insert("c", 3);

        for (k, v) in hash_map {
            match k {
                "a" => assert_eq!(v, 1),
                "b" => assert_eq!(v, 2),
                "c" => assert_eq!(v, 3),
                _ => unreachable!(),
            }
        }
    }
}
