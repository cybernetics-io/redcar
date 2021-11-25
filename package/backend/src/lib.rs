mod lmdb;
mod rocksdb;

pub trait Backend {
    fn insert(&self, k: Vec<u8>, v: Vec<u8>) -> (Vec<u8>, Vec<u8>);
    fn search(&self, key: Vec<u8>) -> Vec<u8>;
}

pub enum Type {
    LMDB,
    RocksDB,
}

pub struct DB;
impl DB {
    pub fn new(t: &Type, path: String) -> Box<dyn Backend + Send + Sync> {
        match t {
            Type::LMDB => Box::new(lmdb::LMDB::new(path)),
            Type::RocksDB => Box::new(rocksdb::RocksDB::new()),
            _ => Box::new(lmdb::LMDB::new(path)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
