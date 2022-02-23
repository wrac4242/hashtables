use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter;

pub struct Hashtable {
    bucket_size: u64,
    buckets: Vec<Option<Box<HashtableData>>>,
}

#[derive(Clone)]
struct HashtableData {
    pub key: String,
    pub value: String,
    pub next: Option<Box<HashtableData>>,
}

impl HashtableData {
    pub fn new() -> HashtableData {
        HashtableData {
            key: String::new(),
            value: String::new(),
            next: None,
        }
    }
}

impl Hashtable {
    pub fn new(bucket_size: u64) -> Hashtable {
        Hashtable {
            bucket_size,
            buckets: iter::repeat(Some(Box::new(HashtableData::new())))
                .take(bucket_size as usize)
                .collect(),
        }
    }

    pub fn insert(&mut self, _key: &str, _value: &str) -> Result<(), ()> {
        // overwrites if same key
        todo!();
    }

    pub fn get(&self, _key: &str) -> Result<&str, ()> {
        todo!();
    }

    pub fn remove(&mut self, _key: &str) -> Result<(), ()> {
        todo!();
    }

    fn get_hash(&self, key: &str) -> u64 {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish() % self.bucket_size
    }
}
