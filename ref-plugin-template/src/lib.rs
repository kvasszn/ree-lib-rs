use std::ffi::CString;

use reframework::{prelude::*};
use imgui_sys;

#[unsafe(no_mangle)]
pub extern "C" fn reframework_plugin_required_version(
    version: *mut reframework::sys::REFrameworkPluginVersion,
) {
    unsafe {
        (*version).major = sys::REFRAMEWORK_PLUGIN_VERSION_MAJOR as i32;
        (*version).minor = sys::REFRAMEWORK_PLUGIN_VERSION_MINOR as i32;
        (*version).patch = sys::REFRAMEWORK_PLUGIN_VERSION_PATCH as i32;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reframework_plugin_required() -> bool {
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn reframework_plugin_initialize(
    param: *const sys::REFrameworkPluginInitializeParam,
) -> bool {
    // initialize the Api first
    unsafe {
        if let Err(e) = Api::initialize(param) {
            log_to_file!("Initializatino error: {e}");
            return false
        }
    }

    // initialize the logger
    // by default takes the name of the crate
    initialize_logging!(); // initialize_logging!("ref-plugin-template");

    // you don;t have to do this like this, you can write it however you want :D
    let _ = entry().is_some();
    true
}

// should probably be a Result here but you do you
pub fn entry() -> Option<()> {
    log::info!("Entry from Rust! 🚀");
    let _ = foo();
    log::info!("Exit from Rust");
    Some(())
}

pub fn foo() -> Option<()>{
    log::debug!("call foo");

    let res = reframework::on_imgui_draw_ui(Some(on_imgui_draw_ui));
    log::debug!("exit foo");
    Some(())
}

// Until I make a cleaner wrapper for ImGui, it will all have to be in unsafe blocks
pub unsafe extern "C" fn on_imgui_draw_ui(data: *mut sys::REFImGuiFrameCbData) {
    unsafe {
        if data.is_null() { return; }
        let data = &*data;
        use imgui_sys::*;

        // set the rust imgui context on each callback
        igSetCurrentContext(data.context as *mut imgui_sys::ImGuiContext);
        let alloc_fn: imgui_sys::ImGuiMemAllocFunc = std::mem::transmute(data.malloc_fn);
        let free_fn: imgui_sys::ImGuiMemFreeFunc = std::mem::transmute(data.free_fn);
        igSetAllocatorFunctions(alloc_fn, free_fn, data.user_data);

        let draw_ui = || -> Option<()> {
            let name = CString::new(env!("CARGO_PKG_NAME")).ok()?;
            igText(name.as_ptr());

            let size = *ImVec2_ImVec2_Float(24.0, 24.0);
            static mut IS_OPEN: bool = false;
            if igButton(c"Click Me!".as_ptr(), size) {
                IS_OPEN = !IS_OPEN;
            }
            if IS_OPEN {
                let module = sdk::get_tdb()?.get_module(0)?;
                let x = format!("{:?}, {}", module, module);
                let x = CString::new(x).ok()?;
                igText(x.as_ptr());
            }

            let player_manager = sdk::get_managed_singleton("snow.player.PlayerManager")?;
            let pm_ty = player_manager.get_type_definition()?;
            let pm = format!("{:?}, {}", pm_ty, player_manager); // it has Display and Debug!
            log::debug!("{pm}");
            let pm = CString::new(pm).ok()?;
            igText(pm.as_ptr());

            let tdb = sdk::get_tdb()?;
            let x = tdb.get_num_types()?;
            let x = CString::new(format!("There are {x} types in the TDB")).ok()?;
            igText(x.as_ptr());
            Some(())
        };

        let _ = draw_ui();
    }
}

