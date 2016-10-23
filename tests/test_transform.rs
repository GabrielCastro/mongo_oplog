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


    let allowed = new_regex(r"^some_other_db\\..*$");
    let tf = NsFilterTransform::new(allowed);
    let tf: &OpTransform = &tf;

    let second = tf.transform(first);

    match second {
        Op::NoOp {..} => (),
        _ => panic!("was expecting second to be NoOp")
    }
}
