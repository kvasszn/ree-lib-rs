use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    let is_windows = target.contains("windows");

    let mut build = cc::Build::new();
    build.cpp(true);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut freetype_includes = Vec::new();

    if is_windows {
        let freetype_lib_dir = format!("{}/thirdparty/freetype/lib", manifest_dir);
        let freetype_include_dir = format!("{}/thirdparty/freetype/include", manifest_dir);

        println!("cargo:rustc-link-search=native={}", freetype_lib_dir);
        println!("cargo:rustc-link-lib=static=freetype");

        freetype_includes.push(PathBuf::from(freetype_include_dir));
    } else {
        let freetype = pkg_config::Config::new()
            .atleast_version("2.0")
            .probe("freetype2")
            .expect("Could not find freetype2 via pkg-config");
        
        freetype_includes = freetype.include_paths;
    }

    build.include("thirdparty/imgui");
    build.include("thirdparty/cimgui");
    for path in &freetype_includes {
        build.include(path);
    }

    let files = [
        "thirdparty/imgui/imgui.cpp",
        "thirdparty/imgui/imgui_draw.cpp",
        "thirdparty/imgui/imgui_tables.cpp",
        "thirdparty/imgui/imgui_widgets.cpp",
        "thirdparty/imgui/misc/freetype/imgui_freetype.cpp",
        "thirdparty/cimgui/cimgui.cpp",
        "thirdparty/imgui/imgui_demo.cpp",
    ];

    for file in &files {
        build.file(file);
    }

    build.define("IMGUI_ENABLE_FREETYPE", None);
    build.define("IMGUI_ENABLE_STB_TRUETYPE", "1");
    build.define("IMGUI_USER_CONFIG", "\"re2_imconfig.hpp\""); 
    
    build.compile("cimgui");

    println!("cargo:rerun-if-changed=wrapper.h");
    
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("--target={}", target))
        .clang_arg("-Ithirdparty/cimgui")
        .clang_arg("-Ithirdparty/imgui")
        .clang_arg("-DIMGUI_ENABLE_FREETYPE")
        .clang_arg("-DIMGUI_ENABLE_STB_TRUETYPE=1")
        .clang_arg("-DIMGUI_USER_CONFIG=\"re2_imconfig.hpp\"")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for path in &freetype_includes {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
