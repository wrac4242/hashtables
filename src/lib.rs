use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Hashtable {
    bucket_size: u64,
}

struct HashtableData<'a> {
    key: &'a str,
    value: &'a str,
    next: Option<&'a HashtableData<'a>>,
}

impl Hashtable {
    pub fn new(bucket_size: u64) -> Hashtable {
        Hashtable { bucket_size }
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Result<(), ()> {
        // overwrites if same key
        todo!();
    }

    pub fn get(&self, key: &str) -> Result<&str, ()> {
        todo!();
    }

    pub fn remove(&mut self, key: &str) -> Result<(), ()> {
        todo!();
    }

    fn get_hash(&self, key: &str) -> u64 {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish() % self.bucket_size
    }
}
