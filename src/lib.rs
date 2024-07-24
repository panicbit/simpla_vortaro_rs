extern crate reqwest;
#[macro_use]
extern crate serde;
extern crate percent_encoding;

#[doc(inline)]
pub use v1::*;

pub mod v1;

pub mod cli;

pub const USER_AGENT: &'static str = concat!(
    "simpla_vortaro/",
    env!("CARGO_PKG_VERSION"),
    " (github.com/panicbit/simpla_vortaro_rs)",
);
