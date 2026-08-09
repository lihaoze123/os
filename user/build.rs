use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let linker_script = manifest_dir.join("src/linker.ld");

    println!("cargo::rerun-if-changed={}", linker_script.display());
    println!("cargo::rustc-link-arg-bins=-T{}", linker_script.display());
}
