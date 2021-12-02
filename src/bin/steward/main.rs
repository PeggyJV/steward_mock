//! Main entry point for Steward

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use steward::application::APP;

/// Boot Steward
fn main() {
    abscissa_core::boot(&APP);
}
