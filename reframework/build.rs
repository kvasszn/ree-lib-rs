use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=include/API.h");

    let bindings = bindgen::Builder::default()
        .header("include/API.h")
        .layout_tests(false) 
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate REFramework bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("reframework_bindings.rs"))
        .expect("Couldn't write bindings!");
}
