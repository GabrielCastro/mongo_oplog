extern crate mongo_oplog;

use mongo_oplog::op_source;

#[ignore]
#[test]
fn test_op_source() {
    let (rx, join_handle) = op_source::create_oplog_receiver();

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
