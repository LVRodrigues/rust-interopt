use std::{env, path::Path};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let header_name = format!("{}.h", env::var("CARGO_PKG_NAME").unwrap());
    let header_guard = format!("{}_H", env::var("CARGO_PKG_NAME").unwrap())
        .replace("-", "_")
        .to_uppercase();
    let header_path = Path::new(&crate_dir)
        .join("target")
        .join("include")
        .join(header_name);

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_language(cbindgen::Language::Cxx)
        .with_include_guard(header_guard)
        .generate()
        .map_or_else(
            |error| match error {
                cbindgen::Error::ParseSyntaxError { .. } => {}
                e => panic!("{:?}", e),
            },
            |bindings| {
                bindings.write_to_file(header_path);
            },
        );
}
