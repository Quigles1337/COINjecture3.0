//! Build script of `cj3-types`: one arithmetic back end (Gate G0-F ruling, clause 4.5c).
//!
//! `curve25519-dalek` chooses its arithmetic back end in its own build script, from the
//! configuration value `curve25519_dalek_backend`, which cargo hands to build scripts as
//! `CARGO_CFG_CURVE25519_DALEK_BACKEND`. This script reads the same variable, for the
//! same target, and reports anything other than exactly `serial` to the crate, which
//! then refuses to compile (`src/lib.rs`).
//!
//! The comparison is exact on purpose. A configuration value can hold several values at
//! once, and cargo joins them with commas. `serial,simd` contains `serial`, yet the
//! library does not choose the serial back end for it.

#![forbid(unsafe_code)]

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(curve25519_dalek_backend, values(\"serial\"))");
    println!("cargo::rustc-check-cfg=cfg(cj3_arithmetic_backend_not_serial)");

    let backend = std::env::var("CARGO_CFG_CURVE25519_DALEK_BACKEND");
    if backend.as_deref() != Ok("serial") {
        println!("cargo::rustc-cfg=cj3_arithmetic_backend_not_serial");
    }
}
