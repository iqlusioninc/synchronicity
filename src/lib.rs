//! Synchronicity
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, unused_lifetimes, unused_qualifications)]

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;
