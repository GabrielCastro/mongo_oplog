use std::error;
use std::fmt;
use std::convert;
use bson;
use mongo_driver;

#[derive(Debug)]
pub enum OpLogError {
    MongoError { cause: mongo_driver::MongoError },
    MalformedOplogEntry { cause: Box<fmt::Debug> },
    UnknownOpType,
    UNKNOWN,
}

impl error::Error for OpLogError {
    fn description(&self) -> &str {
        "OpLogError"
    }
}

impl fmt::Display for OpLogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpLogError")
    }
}

impl convert::From<bson::ValueAccessError> for OpLogError {
    fn from(e: bson::ValueAccessError) -> OpLogError {
        OpLogError::MalformedOplogEntry { cause: Box::new(e) }
    }
}

impl convert::From<mongo_driver::MongoError> for OpLogError {
    fn from(e: mongo_driver::MongoError) -> OpLogError {
        OpLogError::MongoError { cause: e }
    }
}
