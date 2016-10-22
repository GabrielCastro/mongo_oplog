use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use mongo_driver::client::{Client, ClientPool};
use mongo_driver::collection::TailOptions;
use mongo_driver::CommandAndFindOptions;

use bson::Bson;

use op;
use errors;

///
/// Begins to tail the oplog for the given `client` and send those operations to
/// `tx`.
///
/// This function will return
/// `Ok(())` when the receiver for `tx` is dropped. Other wise it will continue
/// tailing the oplog until there is some error
///
fn tail_the_oplog(client: Client, tx: Sender<op::Op>) -> Result<(), errors::OpLogError> {
    try!(client.get_server_status(None));

    let db_name = "local";
    let coll_name = "oplog.rs";
    let gt = Bson::TimeStamp(0);

    info!("starting tail on {}.{} at {}", db_name, coll_name, gt);

    let coll = client.get_collection(db_name, coll_name);

    let query = doc! {
        "ts" => {
            "$gt" => gt
        }
    };

    let opts = CommandAndFindOptions::default();
    let tail_opts = TailOptions::default();

    let cur = coll.tail(query, Some(opts), Some(tail_opts));

    for res in cur {
        let res = try!(res);

        let op = try!(op::Op::from_doc(&res));

        if let Err(_) = tx.send(op) {
            info!("disconnected from tail since receiver has dropped");
            // no one is listening so we'll stop tailing
            break;
        }
    }

    Ok(())
}

///
/// Creates a receiver that will get sent oplog operations as they're tailed.
///
/// The created thread will panic if there is any error in tailing or finish when the receiver is dropped.
///
pub fn create_oplog_receiver(pool: Arc<ClientPool>) -> (Receiver<op::Op>, thread::JoinHandle<()>) {
    let (tx, rx) = channel::<op::Op>();

    let handle: thread::JoinHandle<()> = thread::Builder::new()
        .name("oplog-read-thread".to_string())
        .spawn(move || {
            let client = pool.pop();
            // panic here for now instead of trying to return a result
            let result = tail_the_oplog(client, tx);
            if result.is_err() {
                panic!("tailing ended early: {:?}", result.err().unwrap());
            }
            ()
        })
        .unwrap();
    (rx, handle)
}
