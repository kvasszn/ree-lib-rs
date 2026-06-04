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

    /*let bindings = bindgen::Builder::default()
        .header("include/imgui.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg("-DIMGUI_USE_WCHAR32")
        .clang_arg("-DIMGUI_ENABLE_FREETYPE")
        .clang_arg("-DIMGUI_IMPL_WIN32_DISABLE_GAMEPAD")
        .clang_arg("-DIM_ASSERT(x)=((void)(x))")
        .layout_tests(false)
        .generate()
        .expect("Unable to generate ImGui bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("imgui_bindings.rs"))
            .expect("Couldn't write bindings!");

    cc::Build::new()
        .cpp(true)
        .file("third-party/imgui/imgui.cpp")
        .file("third-party/imgui/imgui_draw.cpp")
        .file("third-party/imgui/imgui_widgets.cpp")
        .file("third-party/imgui/imgui_tables.cpp")
        .define("IMGUI_USER_CONFIG", "\"../../../include/imconfig.h\"")
        .define("IMGUI_DISABLE_DEFAULT_ALLOCATORS", None)
        .flag("-std=c++17")
        .warnings(false)
        .compile("imgui");*/
}
