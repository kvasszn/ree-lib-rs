use crate::*;
impl TypeInfo {
    impl_handle_methods! {
        || &*Api::get().sdk.type_info;
        get_name_raw = pub fn get_name() -> *const c_char;
        pub fn get_type_definition() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn is_clr_type() -> bool;
        pub fn is_singleton() -> bool;
        pub fn get_singleton_instance() -> *mut c_void;
        pub fn create_instance() -> *mut c_void;
        pub fn get_reflection_properties() -> *mut c_void;
        pub fn get_deserializer_fn() -> *mut c_void;
        pub fn get_parent() -> REFrameworkTypeInfoHandle => TypeInfo;
        pub fn get_crc() -> c_uint;
        get_reflection_property_descriptor_raw = pub fn get_reflection_property_descriptor(name: *const c_char) -> REFrameworkReflectionPropertyHandle => ReflectionProperty;
        get_reflection_method_descriptor_raw = pub fn get_reflection_method_descriptor(name: *const c_char) -> REFrameworkReflectionMethodHandle => ReflectionMethod;
    }

    pub fn get_name(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_name_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }

    pub fn get_reflection_property_descriptor(&self, name: &str) -> Option<ReflectionProperty> {
        with_cstrings!(name; self.get_reflection_property_descriptor_raw(name))
    }

    pub fn get_reflection_method_descriptor(&self, name: &str) -> Option<ReflectionMethod> {
        with_cstrings!(name; self.get_reflection_method_descriptor_raw(name))
    }
}
