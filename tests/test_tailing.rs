#[macro_use(doc, bson)]
extern crate bson;
extern crate env_logger;
extern crate mongo_driver;
extern crate mongo_oplog;

use std::sync::Arc;
use std::sync::{Once, ONCE_INIT};
use bson::Bson;
use mongo_driver::client::{ClientPool, Uri};
use mongo_driver::collection::TailOptions;
use mongo_driver::CommandAndFindOptions;

use mongo_oplog::op::Op;

static START: Once = ONCE_INIT;

fn log_init() {
    START.call_once(|| {
        env_logger::init().unwrap();
    });
}

#[ignore]
#[test]
fn test_tail() {
    log_init();

    let uri = Uri::new("mongodb://db:27017/").unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));

    let client = pool.pop();
    client.get_server_status(None).unwrap();


    let coll = client.get_collection("local", "oplog.rs");

    let query = doc! {
        "ts" => {
            "$gt" => (Bson::TimeStamp(0))
        }
    };
    let opts = CommandAndFindOptions::default();
    let tail_opts = TailOptions::default();

    let cur = coll.tail(query, Some(opts), Some(tail_opts));

    let mut i = 0;
    for res in cur {
        let res = res.expect("iter res ok");

        let op = Op::from_doc(&res).expect("is op");

        i = i + 1;

        if i > 6000 {
            break;
        }

        println!("The op is {:?}", op);
    }
}
