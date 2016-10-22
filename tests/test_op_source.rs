extern crate mongo_oplog;

use mongo_oplog::op_source;

#[ignore]
#[test]
fn test_op_source() {

    let rx = op_source::create_oplog_receiver();

    let mut i = 0;
    loop {
        let op = rx.recv().unwrap();
        i = i + 1;
        if i % 50 == 0 {
            println!("{:?}", op);
        }
    }

}
