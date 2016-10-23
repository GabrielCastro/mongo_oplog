use op;

pub trait OpTransform {
    fn transform(&self, op: op::Op) -> op::Op;
}

pub struct NsFilterTransform;

impl NsFilterTransform {
    pub fn new() -> NsFilterTransform {
        NsFilterTransform {}
    }
}

impl OpTransform for NsFilterTransform {
    fn transform(&self, op: op::Op) -> op::Op {
        op::Op::NoOp {
            ts: op.get_ts().clone(),
            h: op.get_h().clone(),
        }
    }
}
