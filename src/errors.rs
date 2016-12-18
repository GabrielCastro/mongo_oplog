//! Contains errors from this create
//!
use std::error;
use std::fmt;
use std::convert;
use bson;
use mongo_driver;
use backtrace::Backtrace;

#[derive(Debug)]
pub struct BacktraceCapture {
    traces: Vec<Backtrace>,
}

impl BacktraceCapture {
    fn new() -> BacktraceCapture {
        let mut new_instance = BacktraceCapture { traces: Vec::with_capacity(1) };
        new_instance.add_trace();
        new_instance
    }

    fn add_trace(&mut self) {
        let bt = Backtrace::new();
        self.traces.push(bt)
    }
}

///
/// All errors will be one of `OpLogError`
///
#[derive(Debug)]
pub enum OpLogError {
    /// An error in the mongo driver: includes connection issues
    MongoError {
        stack: BacktraceCapture,
        cause: mongo_driver::MongoError,
    },
    /// Found something the the oplog that cannot be parsed
    MalformedOplogEntry {
        stack: BacktraceCapture,
        cause: Box<fmt::Debug>,
    },
    /// Found an oplog entry with an `op` field that is unknown
    UnknownOpType {
        stack: BacktraceCapture,
        op_name: String,
    },
    /// any other generic error
    Unknown,
}

impl OpLogError {
    pub fn new_malformed_oplog_entry<T: fmt::Debug + 'static>(cause: T) -> OpLogError {
        OpLogError::MalformedOplogEntry {
            stack: BacktraceCapture::new(),
            cause: Box::new(cause),
        }
    }

    pub fn from_unknown_op(op_name: &str) -> OpLogError {
        OpLogError::UnknownOpType {
            stack: BacktraceCapture::new(),
            op_name: op_name.into(),
        }
    }

    fn description_str(&self) -> String {
        match self {
            &OpLogError::MalformedOplogEntry { ref cause, .. } => {
                format!("OpLogError::MalformedOplogEntry: {:?}", cause)
            }
            &OpLogError::MongoError { ref cause, .. } => {
                format!("OpLogError::MongoError: {:?}", cause)
            }
            &OpLogError::UnknownOpType { ref op_name, .. } => {
                format!("OpLogError::UnknownOpType: {:?} ", op_name)
            }
            &OpLogError::Unknown => format!("OpLogError::Unknown"),
        }
    }
}

impl error::Error for OpLogError {
    fn description(&self) -> &str {
        "OpLogError"
    }
}

impl fmt::Display for OpLogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = self.description_str();
        write!(f, "{}", desc)
    }
}

impl convert::From<bson::ValueAccessError> for OpLogError {
    fn from(e: bson::ValueAccessError) -> OpLogError {
        info!("found malformed entry from: {:?}", e);
        OpLogError::MalformedOplogEntry {
            stack: BacktraceCapture::new(),
            cause: Box::new(e),
        }
    }
}

impl convert::From<mongo_driver::MongoError> for OpLogError {
    fn from(e: mongo_driver::MongoError) -> OpLogError {
        warn!("Got a mongo error! \n{:?}", e);
        OpLogError::MongoError {
            stack: BacktraceCapture::new(),
            cause: e,
        }
    }
}
