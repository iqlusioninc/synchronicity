//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.

#![forbid(unsafe_code)]
#![warn(missing_docs, trivial_casts, unused_qualifications)]

use abscissa_core::testing::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RUNNER: CmdRunner = CmdRunner::default();
}

#[test]
fn start_no_args() {
    let mut runner = RUNNER.clone();
    let cmd = runner.arg("start").run();
    cmd.wait().unwrap().expect_success();
}
