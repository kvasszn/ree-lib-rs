use crate::*;
impl Resource {
    impl_handle_methods! {
        || &*Api::get().sdk.resource;
        pub fn add_ref() -> ();
        pub fn release() -> ();
        create_holder_raw = pub fn create_holder(type_name: *const c_char) -> REFrameworkManagedObjectHandle => ManagedObject;
    }

    pub fn create_holder(&self, type_name: &str) -> Option<ManagedObject> {
        with_cstrings!(type_name; self.create_holder_raw(type_name))
    }
}
