//! Contains errors from this create
//!
use std::error;
use std::fmt;
use std::convert;
use bson;
use mongo_driver;

///
/// All errors will be one of OpLogError
///
#[derive(Debug)]
pub enum OpLogError {
    /// An error in the mongo driver: includes connection issues
    MongoError { cause: mongo_driver::MongoError },
    /// Found something the the oplog that cannot be parsed
    MalformedOplogEntry { cause: Box<fmt::Debug> },
    /// Found an oplog entry with an `op` field that is unknown
    UnknownOpType { op_name: String },
    /// any other generic error
    Unknown,
}

impl OpLogError {
    pub fn from_unknown_op(op_name: &str) -> OpLogError {
        OpLogError::UnknownOpType { op_name: op_name.into() }
    }


    fn description_str(&self) -> String {
        match self {
            &OpLogError::MalformedOplogEntry { ref cause } => {
                format!("OpLogError::MalformedOplogEntry: {:?}", cause)
            }
            &OpLogError::MongoError { ref cause } => format!("OpLogError::MongoError: {:?}", cause),
            &OpLogError::UnknownOpType { ref op_name } => {
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
        OpLogError::MalformedOplogEntry { cause: Box::new(e) }
    }
}

impl convert::From<mongo_driver::MongoError> for OpLogError {
    fn from(e: mongo_driver::MongoError) -> OpLogError {
        warn!("Got a mongo error! \n{:?}", e);
        OpLogError::MongoError { cause: e }
    }
}
