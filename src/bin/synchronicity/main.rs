//! Main entry point for Synchronicity

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use synchronicity::application::APPLICATION;

/// Boot Synchronicity
fn main() {
    abscissa_core::boot(&APPLICATION);
}
