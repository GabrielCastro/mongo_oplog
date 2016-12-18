//! TODO: a top level description
//!
#![cfg_attr(feature = "clippy", allow(unstable_features))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use(doc, bson)]
extern crate bson;
extern crate mongo_driver;
#[macro_use]
extern crate log;
extern crate regex;
extern crate backtrace;

mod errors;
pub mod op;
pub mod op_source;
pub mod transform;
