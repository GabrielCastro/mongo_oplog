use op;
use regex;

pub trait OpTransform {
    fn transform(&self, op: op::Op) -> op::Op;
}

pub struct NsFilterTransform {
    allowed_pattern: regex::Regex
}

impl NsFilterTransform {
    pub fn new(patten: regex::Regex) -> NsFilterTransform {
        NsFilterTransform {
            allowed_pattern: patten
        }
    }
}

impl OpTransform for NsFilterTransform {
    fn transform(&self, op: op::Op) -> op::Op {
        // TODO: there's got to be a better way
        let mut is_match = false;
        if let Some(ns) = op.get_ns() {
            is_match = self.allowed_pattern.is_match(ns);
        }
        if is_match {
            return op;
        }
        op::Op::NoOp {
            ts: op.get_ts().clone(),
            h: op.get_h().clone(),
        }
    }
}
