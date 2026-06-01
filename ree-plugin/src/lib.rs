#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/reframework_bindings.rs"));
}

use ffi::*;

/*use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static LOG_FILE: Lazy<Mutex<std::fs::File>> = Lazy::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ree_plugin.log")
        .expect("Failed to open log file");
    Mutex::new(file)
});

macro_rules! log_to_file {
    ($($arg:tt)*) => {
        if let Ok(mut file) = LOG_FILE.lock() {
            let _ = writeln!(file, $($arg)*);
            let _ = file.flush();
        }
    };
}*/

static mut REFRAMEWORK_API: Option<*const REFrameworkPluginInitializeParam> = None;

#[unsafe(no_mangle)]
pub extern "C" fn reframework_plugin_required() -> bool {
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn reframework_plugin_initialize(
    param: *const REFrameworkPluginInitializeParam,
) -> bool {
    //log_to_file!("Inside plugin intialize");
    unsafe {
        REFRAMEWORK_API = Some(param);

        if let Some(api) = REFRAMEWORK_API {
            let functions = (*api).functions;

            let msg = std::ffi::CString::new("Hello from ree-lib Rust Plugin!").unwrap();
            if let Some(log_info) = (*functions).log_info {
                log_info(msg.as_ptr());
            }

        }
    }
    true
}
