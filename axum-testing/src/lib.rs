#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::perf)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.me")]

mod state;
mod test_app;

/// AVRO assertion helpers
#[cfg(feature = "avro")]
pub mod avro;

pub use self::state::*;
pub use self::test_app::*;
