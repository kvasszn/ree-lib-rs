use crate::*;
impl ResourceManager {
    impl_handle_methods! {
        || &*Api::get().sdk.resource_manager;
        create_resource_raw = pub fn create_resource(type_name: *const c_char, name: *const c_char) -> REFrameworkResourceHandle => Resource;
        create_userdata_raw = pub fn create_userdata(type_name: *const c_char, name: *const c_char) -> REFrameworkManagedObjectHandle => ManagedObject;
    }

    pub fn create_resource(&self, type_name: &str, name: &str) -> Option<Resource> {
        with_cstrings!(type_name, name; self.create_resource_raw(type_name, name))
    }

    pub fn create_userdata(&self, type_name: &str, name: &str) -> Option<ManagedObject> {
        with_cstrings!(type_name, name; self.create_userdata_raw(type_name, name))
    }
}
