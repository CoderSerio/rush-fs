// #![deny(clippy::all)]
// use napi_derive::napi;

// #[napi]
// pub fn plus_100(input: u32) -> u32 {
//   input + 100
// }

// src/lib.rs
#![deny(clippy::all)]

// define modules
pub mod readdir;
pub mod rm;
pub mod types;
pub mod utils;

//export modules
pub use readdir::*;
pub use rm::*;
pub use types::*;
