// ree-plugin/build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed=include/API.h");

    let bindings = bindgen::Builder::default()
        .header("include/API.h")
        // Make it easy to read if you ever need to inspect it
        .layout_tests(false) 
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate REFramework bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("reframework_bindings.rs"))
        .expect("Couldn't write bindings!");
}
