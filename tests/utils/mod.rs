//! This modules contains code that is to be reused from many tests
//! The test here is mostly to silence used code warnings, and to force all utils
//! to be compiles and are NOT for testing this module
//!
extern crate env_logger;
extern crate regex;


use std::sync::{Once, ONCE_INIT};
use std::sync::Arc;
use mongo_driver::client::{ClientPool, Uri};

static LOG_INIT_ONCE: Once = ONCE_INIT;

///
/// Initializes an env logger so out tests can have more output
///     Should be called at least once, but can be called many times.
///
pub fn log_init() {
    LOG_INIT_ONCE.call_once(|| {
        env_logger::init().unwrap();
        debug!("logging initialized");
    });
}

///
/// Should be used to acquire a client to the db we're using for testing
///
pub fn get_mongo() -> Arc<ClientPool> {
    let _ = "mongodb://db:27017/";
    let uri_str = "mongodb://192.168.1.147:27017/?readPreference=secondaryPreferred&slaveOk=1";
    info!("creating client pool for {}", uri_str);
    let uri = Uri::new(uri_str).unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    pool
}

pub fn new_regex(pattren: &str) -> regex::Regex {
    regex::Regex::new(pattren).unwrap()
}

///
/// Forces everything in this mod to be compiled and removes unused code warnings
///
#[test]
fn test_self() {
    log_init();
    get_mongo();
    new_regex(r".*");
}
