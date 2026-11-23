//! cargo-fuzz target: one property of `cj3_fuzz`, driven by libFuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| cj3_fuzz::header_codec(data));
