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
    NoOp { ts: i64, h: i64 },
    Delete {
        ts: i64,
        h: i64,
        ns: String,
        _id: oid::ObjectId,
    },
    /// see: https://groups.google.com/forum/#!topic/mongodb-user/dTf5VEJJWvY
    ApplyOps {
        ts: i64,
        h: i64,
        ns: String,
        apply_ops: Vec<Op>,
    },
    Command {
        ts: i64,
        h: i64,
        ns: String,
        o: Option<Bson>,
    },
}

impl Op {
    pub fn get_ts(&self) -> &i64 {
        match self {
            &Op::Insert { ref ts, .. } => ts,
            &Op::Update { ref ts, .. } => ts,
            &Op::NoOp { ref ts, .. } => ts,
            &Op::Delete { ref ts, .. } => ts,
            &Op::ApplyOps { ref ts, .. } => ts,
            &Op::Command { ref ts, .. } => ts,
        }
    }

    pub fn get_ns(&self) -> Option<&String> {
        match self {
            &Op::Insert { ref ns, .. } => Some(ns),
            &Op::Update { ref ns, .. } => Some(ns),
            &Op::NoOp { .. } => None,
            &Op::Delete { ref ns, .. } => Some(ns),
            &Op::ApplyOps { ref ns, .. } => Some(ns),
            &Op::Command { ref ns, .. } => Some(ns),
        }
    }

    pub fn get_h(&self) -> &i64 {
        match self {
            &Op::Insert { ref h, .. } => h,
            &Op::Update { ref h, .. } => h,
            &Op::NoOp { ref h, .. } => h,
            &Op::Delete { ref h, .. } => h,
            &Op::ApplyOps { ref h, .. } => h,
            &Op::Command { ref h, .. } => h,
        }
    }

    fn get_common(doc: &Document) -> Result<(i64, i64, &str), OpLogError> {
        let ts = doc.get_time_stamp("ts")?;
        let h = doc.get_i64("h")?;
        let ns = doc.get_str("ns")?;
        Ok((ts, h, ns))
    }

    fn from_update(doc: &Document) -> Result<Op, OpLogError> {
        let (ts, h, ns) = Op::get_common(doc)?;

        let o = doc.get_document("o")?;
        let o2 = doc.get_document("o2")?;

        Ok(Op::Update {
            ts: ts,
            h: h,
            ns: ns.into(),
            o: o.clone(),
            o2: o2.clone(),
        })
    }

    fn from_insert(doc: &Document) -> Result<Op, OpLogError> {
        let (ts, h, ns) = Op::get_common(doc)?;

        let o = doc.get_document("o")?;

        Ok(Op::Insert {
            ts: ts,
            h: h,
            ns: ns.into(),
            o: o.clone(),
        })
    }

    fn from_noop(doc: &Document) -> Result<Op, OpLogError> {
        let ts = doc.get_time_stamp("ts")?;
        let h = doc.get_i64("h")?;
        Ok(Op::NoOp { ts: ts, h: h })
    }

    fn from_delete(doc: &Document) -> Result<Op, OpLogError> {
        let (ts, h, ns) = Op::get_common(doc)?;

        let o = doc.get_document("o")?;
        let _id = o.get_object_id("_id")?;

        Ok(Op::Delete {
            ts: ts,
            h: h,
            ns: ns.into(),
            _id: _id.clone(),
        })
    }

    fn from_command(doc: &Document) -> Result<Op, OpLogError> {
        let (ts, h, ns) = Op::get_common(doc)?;

        if !doc.contains_key("o") {
            return Ok(Op::Command {
                ts: ts,
                h: h,
                ns: ns.into(),
                o: None,
            });
        }

        let o = doc.get_document("o").unwrap();

        if !o.contains_key("applyOps") {
            return Ok(Op::Command {
                ts: ts,
                h: h,
                ns: ns.into(),
                o: Some(Bson::Document(o.clone())),
            });
        }

        let apply_ops = o.get_array("applyOps")?;

        let ops_result: Result<Vec<Op>, _> = apply_ops.into_iter()
            .map(|bson| Op::from_bson(bson))
            .collect();

        let ops_result = ops_result?;

        Ok(Op::ApplyOps {
            ts: ts,
            h: h,
            ns: ns.into(),
            apply_ops: ops_result,
        })
    }

    fn from_bson(bson_doc: &Bson) -> Result<Op, OpLogError> {
        match bson_doc {
            &Bson::Document(ref doc) => Op::from_doc(doc),
            _ => Err(OpLogError::new_malformed_oplog_entry("bson is not document")),
        }
    }

    ///
    /// Converts a bson::Document into an oplog entry
    ///
    pub fn from_doc(doc: &Document) -> Result<Op, OpLogError> {
        let op_name = doc.get_str("op")?;

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
