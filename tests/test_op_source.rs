#[macro_use]
extern crate log;
extern crate mongo_oplog;
extern crate mongo_driver;
mod utils;

use mongo_oplog::op_source;

#[ignore]
#[test]
fn test_op_source() {
    utils::log_init();

    let pool = utils::get_mongo();

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

#[test]
fn force_test_compile() {
    utils::log_init();
    debug!("force_test_compile of test_op_sources");
    utils::get_mongo();
    // this is here so that cargo test still compiles this module
    // even though the above is ignored.
    assert!(true, "this test module compiles");
}
