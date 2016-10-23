extern crate mongo_oplog;
#[macro_use]
extern crate log;
extern crate mongo_driver;
#[macro_use]
extern crate bson;

use mongo_oplog::op::Op;
use mongo_oplog::transform::{NsFilterTransform, OpTransform};

mod utils;

use utils::new_regex;

fn insert_into(ns: String) -> Op {
    let o = doc! {
        "apple"=> "bannana",
        "cat"=> false,
        "dog"=> true
    };

    Op::Insert {
        ts: 0i64,
        h: 1i64,
        ns: ns,
        o: o,
    }
}

#[test]
fn test_ns_filtering_removes_db() {
    let op = insert_into("mydb.mycoll".into());

    let allowed = new_regex(r"^some_other_db\..*$");
    let tf = NsFilterTransform::new(allowed);
    let tf: &OpTransform = &tf;

    let second = tf.transform(op);

    match second {
        Op::NoOp {..} => (),
        _ => panic!("was expecting second to be NoOp")
    }
}

#[test]
fn test_ns_filtering_keeps_db() {
    let op = insert_into("mydb.mycoll".into());

    let allowed = new_regex(r"^mydb\..*$");
    let tf = NsFilterTransform::new(allowed);
    let tf: &OpTransform = &tf;

    let second = tf.transform(op);

    match second {
        Op::Insert {..} => (),
        _ => panic!("was expecting second to be Insert")
    }
}
