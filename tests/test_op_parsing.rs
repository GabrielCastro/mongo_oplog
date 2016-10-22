#[macro_use]
extern crate log;
#[macro_use(doc, bson)]
extern crate bson;
extern crate mongo_driver;
extern crate env_logger;
extern crate mongo_oplog;

use bson::Bson;
use bson::oid;

use mongo_oplog::op::Op;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;

fn log_init() {
    START.call_once(|| {
        env_logger::init().unwrap();
    });
}

#[test]
fn check_op_insert() {
    log_init();
    let insert = doc! {
        "op" => "i",
        "ns" => "foo.bar",
        "h" => 66i64,
        "ts" => (Bson::TimeStamp(0)),
        "o" => {
            "foo" => "bar",
            "fizz" => "buzz"
        }
    };

    let op = Op::from_doc(&insert).expect("from_doc");

    match op {
        Op::Insert { ref ts, ref h, ref ns, ref o } => {
            assert_eq!("foo.bar", ns);
            assert_eq!(&66i64, h);
            assert_eq!(&0i64, ts);
            assert_eq!(&doc! {
                "foo" => "bar",
                "fizz" => "buzz"
            },
                       o);
        }
        _ => panic!("op did not match insert"),
    };
}

#[test]
fn check_op_update() {
    log_init();

    let insert = doc! {
        "op" => "u",
        "ns" => "foo.bar",
        "h" => 66i64,
        "ts" => (Bson::TimeStamp(0)),
        "o" => {
            "foo" => "bar",
            "fizz" => "buzz"
        },
        "o2" => {
            "_id" => 77i32
        }
    };

    let op = Op::from_doc(&insert).expect("from_doc");

    match op {
        Op::Update { ref ts, ref h, ref ns, ref o, ref o2 } => {
            assert_eq!("foo.bar", ns);
            assert_eq!(&66i64, h);
            assert_eq!(&0i64, ts);
            assert_eq!(&doc! {
                "foo" => "bar",
                "fizz" => "buzz"
            }, o);
            assert_eq!(&doc! {
                "_id" => 77i32
            },
                       o2);
        }
        _ => panic!("op did not match update"),
    };
}

#[test]
fn check_op_noop() {
    log_init();

    let insert = doc! {
        "op" => "n",
        "h" => 66i64,
        "ts" => (Bson::TimeStamp(0))
    };

    let op = Op::from_doc(&insert).expect("from_doc");

    match op {
        Op::NoOp { ref ts, ref h } => {
            assert_eq!(&66i64, h);
            assert_eq!(&0i64, ts);
        }
        _ => panic!("op did not match NoOp"),
    };
}

#[test]
fn check_op_delete() {
    log_init();
    let id = oid::ObjectId::with_string("123456789012345678901234").unwrap();
    let insert = doc! {
        "op" => "d",
        "ns" => "foo.bar",
        "h" => 66i64,
        "ts" => (Bson::TimeStamp(0)),
        "o" => {
            "_id" => (Bson::ObjectId(id))
        }
    };

    let op = Op::from_doc(&insert).expect("from_doc");

    match op {
        Op::Delete { ref ts, ref h, ref ns, ref _id } => {
            let expected_id = oid::ObjectId::with_string("123456789012345678901234").unwrap();

            assert_eq!("foo.bar", ns);
            assert_eq!(&66i64, h);
            assert_eq!(&0i64, ts);
            assert_eq!(&expected_id, _id);
        }
        _ => panic!("op did not match insert"),
    };
}

#[test]
fn check_op_command() {
    log_init();

    let insert = doc! {
        "op" => "c",
        "h" => 66i64,
        "ts" => (Bson::TimeStamp(0)),
        "ns" => "test.$cmd"
    };

    let op = Op::from_doc(&insert).expect("from_doc");

    match op {
        Op::Command { ref ts, ref h, ref ns, ref apply_ops } => {
            assert_eq!(&0i64, ts);
            assert_eq!(&66i64, h);
            assert_eq!("test.$cmd", ns);
            assert!(apply_ops.is_none(), "apply_ops.is_none()");
        }
        _ => panic!("op did not match command"),
    };
}

#[test]
fn check_op_command_apply_ops() {
    log_init();

    let ops: bson::Array = vec![Bson::Document(doc! {
            "op" => "n",
            "h" => 66i64,
            "ts" => (Bson::TimeStamp(0))
        })];
    let insert = doc! {
        "op" => "c",
        "h" => 66i64,
        "ts" => (Bson::TimeStamp(0)),
        "ns" => "test.$cmd",
        "o" => {
            "applyOps" => ops
        }
    };

    let op = Op::from_doc(&insert).expect("from_doc");

    match op {
        Op::Command { ts, h, ns, mut apply_ops } => {
            assert_eq!(0i64, ts);
            assert_eq!(66i64, h);
            assert_eq!("test.$cmd", ns);
            assert!(apply_ops.is_some(), "apply_ops.is_some()");

            let ops = apply_ops.take().unwrap();

            assert_eq!(ops.len(), 1, "ops_size");

            let first_op = &ops[0];

            match first_op {
                Op::NoOp { .. } => (),
                _ => panic!("expected NoOp"),
            }
        }
        _ => panic!("op did not match command"),
    };
}


#[test]
fn check_op_unknown() {
    log_init();
    let insert = doc! {
        "op" => "?"
    };

    Op::from_doc(&insert).err().expect("error expected");
}
