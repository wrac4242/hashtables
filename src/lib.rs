pub struct Hashtable {
    bucket_size: u32,
}

impl Hashtable {
    pub fn new(bucket_size: u32) -> Hashtable {
        Hashtable {
            bucket_size: bucket_size,
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Result<(), ()> {
        todo!();
    }

    pub fn get(&self, key: &str) -> Result<&str, ()> {
        todo!();
    }

    pub fn remove(&mut self, key: &str) -> Result<(), ()> {
        todo!();
    }
}
