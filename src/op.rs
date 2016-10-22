use bson;
use bson::{Bson, Document, oid};
use errors::OpLogError;

///
/// Represents an entry from the oplog
///
/// see: https://docs.mongodb.com/v3.2/core/replica-set-oplog/
///
#[derive(Debug)]
pub enum Op {
    Insert {
        ts: i64,
        h: i64,
        ns: String,
        o: bson::Document,
    },
    Update {
        ts: i64,
        h: i64,
        ns: String,
        o: bson::Document,
        o2: bson::Document,
    },
    NoOp {
        ts: i64,
        h: i64
    },
    Delete {
        ts: i64,
        h: i64,
        ns: String,
        _id: oid::ObjectId,
    },
    Command {
        ts: i64,
        h: i64,
        ns: String,
        apply_ops: Option<Vec<Op>>,
    },
}

impl Op {
    
    fn get_common(doc: &Document) -> Result<(i64, i64, &str), OpLogError> {
        let ts = try!(doc.get_time_stamp("ts"));
        let h = try!(doc.get_i64("h"));
        let ns = try!(doc.get_str("ns"));
        Ok((ts, h, ns))
    }

    fn from_update(doc: &Document) -> Result<Op, OpLogError> {
        let (ts, h, ns) = try!(Op::get_common(doc));

        let o = try!(doc.get_document("o"));
        let o2 = try!(doc.get_document("o2"));

        Ok(Op::Update {
            ts: ts,
            h: h,
            ns: ns.into(),
            o: o.clone(),
            o2: o2.clone(),
        })
    }

    fn from_insert(doc: &Document) -> Result<Op, OpLogError> {
        let (ts, h, ns) = try!(Op::get_common(doc));

        let o = try!(doc.get_document("o"));

        Ok(Op::Insert {
            ts: ts,
            h: h,
            ns: ns.into(),
            o: o.clone(),
        })
    }

    fn from_noop(doc: &Document) -> Result<Op, OpLogError> {
        let ts = try!(doc.get_time_stamp("ts"));
        let h = try!(doc.get_i64("h"));
        Ok(Op::NoOp { ts: ts, h: h })
    }

    fn from_delete(doc: &Document) -> Result<Op, OpLogError> {
        let (ts, h, ns) = try!(Op::get_common(doc));

        let o = try!(doc.get_document("o"));
        let _id = try!(o.get_object_id("_id"));

        Ok(Op::Delete {
            ts: ts,
            h: h,
            ns: ns.into(),
            _id: _id.clone(),
        })
    }

    fn from_command(doc: &Document) -> Result<Op, OpLogError> {
        let (ts, h, ns) = try!(Op::get_common(doc));

        let mut op = Op::Command {
            ts: ts,
            h: h,
            ns: ns.into(),
            apply_ops: None,
        };

        // see: https://groups.google.com/forum/#!topic/mongodb-user/dTf5VEJJWvY
        if doc.contains_key("o") {
            let o = try!(doc.get_document("o"));
            if o.contains_key("applyOps") {
                let apply_ops = try!(o.get_array("applyOps"));

                let ops_result: Result<Vec<Op>, _> = apply_ops.into_iter()
                    .map(|bson| Op::from_bson(bson))
                    .collect();

                let ops_result = Some(try!(ops_result));

                if let Op::Command { ref mut apply_ops, .. } = op {
                    *apply_ops = ops_result;
                }
            }
        }

        Ok(op)
    }

    fn from_bson(bson_doc: &Bson) -> Result<Op, OpLogError> {
        match bson_doc {
            &Bson::Document(ref doc) => Op::from_doc(doc),
            _ => Err(OpLogError::MalformedOplogEntry { cause: Box::new("bson is not document") }),
        }
    }

    ///
    /// Converts a bson::Document into an oplog entry
    /// 
    pub fn from_doc(doc: &Document) -> Result<Op, OpLogError> {
        let op_name = try!(doc.get_str("op"));

        match op_name {
            "u" => Op::from_update(doc),
            "i" => Op::from_insert(doc),
            "n" => Op::from_noop(doc),
            "d" => Op::from_delete(doc),
            "c" => Op::from_command(doc),
            _ => Err(OpLogError::from_unknown_op(op_name)),
        }
    }
}
