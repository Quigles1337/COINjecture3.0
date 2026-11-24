//! Build script of `cj3-types`: one arithmetic back end, at one word size (Gate G0-F
//! ruling, clause 4.5c; ruling "P-101 TO ITS FRAME", clauses 2.2 and 4.3a).
//!
//! `curve25519-dalek` chooses its arithmetic in its own build script, from two
//! configuration values: `curve25519_dalek_backend` and `curve25519_dalek_bits`. cargo
//! hands them to build scripts as `CARGO_CFG_CURVE25519_DALEK_BACKEND` and
//! `CARGO_CFG_CURVE25519_DALEK_BITS`. This script reads the same two variables, for the
//! same target. It reports a back end that is anything other than exactly `serial`, and a
//! word size that is anything other than exactly `64`, to the crate, which then refuses
//! to compile (`src/lib.rs`).
//!
//! Both comparisons are exact on purpose. A configuration value can hold several values
//! at once, and cargo joins them with commas. `serial,simd` contains `serial`, yet the
//! library does not choose the serial back end for it. And the library treats any word
//! size it does not recognise, `32,64` included, as if none were set: it then chooses by
//! the target's pointer width.

#![forbid(unsafe_code)]

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(curve25519_dalek_backend, values(\"serial\"))");
    println!("cargo::rustc-check-cfg=cfg(curve25519_dalek_bits, values(\"64\"))");
    println!("cargo::rustc-check-cfg=cfg(cj3_arithmetic_backend_not_serial)");
    println!("cargo::rustc-check-cfg=cfg(cj3_arithmetic_bits_not_64)");

    let backend = std::env::var("CARGO_CFG_CURVE25519_DALEK_BACKEND");
    if backend.as_deref() != Ok("serial") {
        println!("cargo::rustc-cfg=cj3_arithmetic_backend_not_serial");
    }

    let bits = std::env::var("CARGO_CFG_CURVE25519_DALEK_BITS");
    if bits.as_deref() != Ok("64") {
        println!("cargo::rustc-cfg=cj3_arithmetic_bits_not_64");
    }
}
