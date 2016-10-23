extern crate mongo_oplog;
#[macro_use]
extern crate bson;

use mongo_oplog::op::Op;
use mongo_oplog::transform::{NsFilterTransform, OpTransform};

#[test]
fn test_ns_filtering() {
    let o = doc! {
        "apple"=> "bannana",
        "cat"=> false,
        "dog"=> true
    };

    let first = Op::Insert {
        ts: 0i64,
        h: 1i64,
        ns: "my_database.my_collection".into(),
        o: o,
    };


    let tf = NsFilterTransform::new();
    let tf: &OpTransform = &tf;

    let second = tf.transform(first);
}
