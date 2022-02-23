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
    pub fn new(key: String, value: String) -> HashtableData {
        HashtableData {
            key,
            value,
            next: None,
        }
    }
}

impl Hashtable {
    pub fn new(bucket_size: u64) -> Hashtable {
        Hashtable {
            bucket_size,
            buckets: iter::repeat(None)
                .take(bucket_size as usize)
                .collect(),
        }
    }

    pub fn insert(&mut self, _key: &str, _value: &str) -> Result<(), ()> {
        // overwrites if same key
        // attaches to the bottom due to overwrite 
        todo!();
    }

    pub fn get(&self, key: &str) -> Result<&str, ()> {
        let hash = self.get_hash(key);
        let mut to_check = &self.buckets[hash as usize];
        loop {
            match to_check {
                Some(data) => {
                    if data.key == key {
                        return Ok(&data.value)
                    } else {
                        to_check = &data.next;
                    }
                }
                None => return Err(()),
            };
        }
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
