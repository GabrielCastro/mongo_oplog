use std::sync::{Arc};
use mongo_driver::client::{ClientPool, Uri};

pub fn get_mongo() -> Arc<ClientPool> {
    let _ = Uri::new("mongodb://db:27017/").unwrap();
    let uri = Uri::new("mongodb://192.168.1.147:27017/?readPreference=secondaryPreferred&slaveOk=1").unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    pool
}
