use rustbreak::{Database, Result};

lazy_static! {
    static ref DB: Database<String> = {
        Database::open("/tmp/more_data").unwrap()
    };
}

fn get_data(key: &str) -> Result<u64> {
    DB.retrieve(key)
}

fn set_data(key: &str, d: u64) -> Result<()> {
    let mut lock = try!(DB.lock());
    let old_data : u64 = try!(lock.retrieve(key));
    lock.insert(key, d + old_data)
}