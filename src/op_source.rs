use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use mongo_driver::client::{ClientPool, Uri};
use mongo_driver::collection::TailOptions;
use mongo_driver::CommandAndFindOptions;

use bson::Bson;

use op;

fn bar(tx: Sender<op::Op>) {
    //    let uri = Uri::new("mongodb://db:27017/").unwrap();
    let uri = Uri::new("mongodb://192.168.1.147:27017/").unwrap();
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

    for res in cur {
        let res = res.expect("iter res ok");

        let op = op::Op::from_doc(&res).expect("is op");

        tx.send(op).unwrap();
    }
}

pub fn create_oplog_receiver() -> (Receiver<op::Op>, thread::JoinHandle)
{
    let (tx, rx) = channel::<op::Op>();

    let handle = thread::Builder::new()
        .name("oplog-read-thread".to_string())
        .spawn(move || {
            bar(tx);
        })
        .unwrap();
    (rx, handle)
}
