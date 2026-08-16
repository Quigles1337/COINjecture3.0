#![forbid(unsafe_code)]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(cj3_testnet)");
}
