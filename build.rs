extern crate cbindgen;

use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let header_path = Path::new(&crate_dir).join("target").join("rust-interop.h");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::Cxx)
        .with_include_guard("RUST_INTEROP_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);
}