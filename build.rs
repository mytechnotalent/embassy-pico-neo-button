// build.rs
//
// Copies memory.x into OUT_DIR so the linker can find it, and
// emits the RP2040 linker‐script arguments for normal firmware.
// During `cargo test`, it also emits the `-Tembedded-test.x` flag.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // 1) Copy `memory.x` into OUT_DIR:
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // Tell the linker to search OUT_DIR for `memory.x`:
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Re‐run this script if memory.x changes:
    println!("cargo:rerun-if-changed=memory.x");

    // 2) Emit the four standard RP2040 link‐args (always for firmware):
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");

    // 3) If we are building tests, tell rustc to pass `-Tembedded-test.x`:
    //
    //    During `cargo test`, the PROFILE environment variable is "test".
    //    We want test executables to use embedded-test.x instead of link.x.
    if env::var("PROFILE").as_deref() == Ok("test") {
        println!("cargo:rustc-link-arg-tests=-Tembedded-test.x");
    }
}
