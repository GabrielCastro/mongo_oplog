extern crate mongo_oplog;
extern crate mongo_driver;

use std::sync::Arc;
use mongo_oplog::op_source;
use mongo_driver::client::{ClientPool, Uri};


#[ignore]
#[test]
fn test_op_source() {

    //    let uri = Uri::new("mongodb://db:27017/").unwrap();
    let uri = Uri::new("mongodb://192.168.1.147:27017/").unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));

    let (rx, join_handle) = op_source::create_oplog_receiver(pool);

    for _ in 1..10 {
        let op = rx.recv();
        if op.is_err() {
            println!("{:?}", op.err().unwrap());
            break;
        }
        let op = op.expect("to receive op");

        println!("{:?}", op);
    }

    drop(rx);

    match join_handle.join() {
        Err(err) => println!("{:?}", err),
        Ok(_) => ()
    }
}
