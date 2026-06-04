use reframework::{prelude::*};

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
    //let tdb = sdk::get_tdb()?;
    //let x = tdb.get_num_types()?;
    //log::info!("There are {x} types in the TDB");

    /*let player_manager = sdk::get_managed_singleton("snow.player.PlayerManager")?;
    let pm_type_info = player_manager.get_type_definition()?;
    let name = pm_type_info.get_name()?;
    let namespace = pm_type_info.get_namespace()?;
    log::debug!("{namespace}::{name}");*/

    let res = reframework::on_imgui_draw_ui(Some(reframework::rust_on_imgui_draw_ui));
    //let res = reframework::on_imgui_frame(Some(reframework::rust_on_imgui_frame));
    //log::debug!("imgui res={res:?}");
    log::debug!("exit foo");
    Some(())
}

