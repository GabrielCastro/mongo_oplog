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

    // so that we can drop rx and it's iterator
    {
        let rx_iter = rx.iter().take(10);
        for op in rx_iter {
            trace!("{:?}", op);
        }
    }

    drop(rx);

    if let Err(err) = join_handle.join() {
        panic!(err);
    }
}

/**
 *  this is here so that cargo test still compiles this module
 *  even though the above is ignored.
 */
#[test]
fn force_test_compile() {
    utils::log_init();
    debug!("force_test_compile of test_op_sources");
    utils::get_mongo();
    // this is here so that cargo test still compiles this module
    // even though the above is ignored.
    assert!(true, "this test module compiles");
}
