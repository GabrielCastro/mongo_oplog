extern crate env_logger;

use std::sync::{Arc};
use mongo_driver::client::{ClientPool, Uri};

use std::sync::{Once, ONCE_INIT};

static LOG_INIT_ONCE: Once = ONCE_INIT;

pub fn log_init() {
    LOG_INIT_ONCE.call_once(|| {
        env_logger::init().unwrap();
        debug!("logging initialized");
    });
}


pub fn get_mongo() -> Arc<ClientPool> {
    let _ = "mongodb://db:27017/";
    let uri_str = "mongodb://192.168.1.147:27017/?readPreference=secondaryPreferred&slaveOk=1";
    info!("creating client pool for {}", uri_str);
    let uri = Uri::new(uri_str).unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    pool
}

#[test]
fn test_self() {
    log_init();
    get_mongo();
}
