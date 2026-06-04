use crate::*;
impl ManagedObject {
    impl_handle_methods! {
        || &*Api::get().sdk.managed_object;
        pub fn add_ref() -> ();
        pub fn release() -> ();
        pub fn get_type_definition() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_ref_count() -> c_uint;
        pub fn get_size() -> c_uint;
        pub fn get_vm_obj_type() -> c_uint;
        pub fn get_type_info() -> REFrameworkTypeInfoHandle => TypeInfo;
        pub fn get_reflection_properties() -> *mut c_void;
        get_reflection_property_descriptor_raw = pub fn get_reflection_property_descriptor(name: *const c_char) -> REFrameworkReflectionPropertyHandle => ReflectionProperty;
        get_reflection_method_descriptor_raw = pub fn get_reflection_method_descriptor(name: *const c_char) -> REFrameworkReflectionMethodHandle => ReflectionMethod;
    }

    pub fn get_reflection_property_descriptor(&self, name: &str) -> Option<ReflectionProperty> {
        with_cstrings!(name; self.get_reflection_property_descriptor_raw(name))
    }

    pub fn get_reflection_method_descriptor(&self, name: &str) -> Option<ReflectionMethod> {
        with_cstrings!(name; self.get_reflection_method_descriptor_raw(name))
    }
}

impl std::fmt::Display for ManagedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_type_definition().and_then(|t| t.get_full_name()) {
            Some(name) => write!(f, "{}@{:p}", name, self.0),
            None => write!(f, "<unknown>@{:p}", self.0),
        }
    }
}

impl std::fmt::Debug for ManagedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ManagedObject({} refs={:?} @ {:p})",
            self.get_type_definition()
                .and_then(|t| t.get_full_name())
                .as_deref()
                .unwrap_or("?"),
            self.get_ref_count(),
            self.0
        )
    }
}
