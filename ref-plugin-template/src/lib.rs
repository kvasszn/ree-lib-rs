use reframework::*;

#[unsafe(no_mangle)]
pub extern "C" fn reframework_plugin_required() -> bool {
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn reframework_plugin_initialize(
    param: *const REFrameworkPluginInitializeParam,
) -> bool {
    ref_log::initialize_logging();
    unsafe {
        if let Err(e) = Api::initialize(param) {
            log_to_file!("Initializatino error: {e}");
            return false
        }
    }
    entry().is_some()
}

pub fn entry() -> Option<()> {
    log::debug!("entry");
    log::info!("Entry from Rust! 🚀");
    let _ = foo();
    log::debug!("exit");
    None
}

pub fn foo() -> Option<()>{
    log::debug!("call foo");
    let player_manager = sdk::get_managed_singleton("snow.player.PlayerManager")?;
    let pm_type_info = player_manager.get_type_definition()?;
    let name = pm_type_info.get_name()?;
    let namespace = pm_type_info.get_namespace()?;
    log::info!("{namespace}::{name}");
    log::debug!("exit foo");
    None
}

