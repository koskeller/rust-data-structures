#![allow(unused)]

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

pub struct BloomFilter {
    size: usize,
    bytes: Vec<u8>,
    hashers: Vec<RandomState>,
}

impl BloomFilter {
    pub fn with_dimensions(size: usize, hashers_count: usize) -> Self {
        let bytes_count = size / 8 + if size % 8 > 0 { 1 } else { 0 };
        Self {
            size,
            bytes: vec![0; bytes_count],
            hashers: vec![RandomState::new(); hashers_count],
        }
    }

    pub fn from_estimate(n_of_items: usize, target_prob: f64) -> Self {
        // From https://en.wikipedia.org/wiki/Bloom_filter
        let size =
            (-(n_of_items as f64) * target_prob.ln() / (2.0_f64.ln().powi(2))).ceil() as usize;
        let hashers_count = ((size as f64 / n_of_items as f64) * 2.0_f64.ln()).ceil() as usize;
        Self::with_dimensions(size, hashers_count)
    }
}

impl BloomFilter {
    pub fn insert<T: Hash>(&mut self, item: T) {
        for hasher in &self.hashers {
            let mut hasher = hasher.build_hasher();
            item.hash(&mut hasher);
            let hash = hasher.finish();
            let index = hash % self.size as u64;

            let byte_index = index as usize / 8;
            let bit_index = (index % 8) as u8;

            self.bytes[byte_index] |= 1 << bit_index;
        }
    }

    pub fn contains<T: Hash>(&mut self, item: T) -> bool {
        for hasher in &self.hashers {
            let mut hasher = hasher.build_hasher();
            item.hash(&mut hasher);
            let hash = hasher.finish();
            let index = hash % self.size as u64;

            let byte_index = index as usize / 8;
            let bit_index = (index % 8) as u8;

            if self.bytes[byte_index] * (1 << bit_index) == 0 {
                return false;
            }
        }
        true
    }
}
