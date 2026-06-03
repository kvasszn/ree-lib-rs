use crate::*;

impl TypeDefinition {
    impl_handle_methods! {
        ||  &*Api::get().sdk.type_definition ;
        pub fn get_index() -> c_uint;
        pub fn get_size() -> c_uint;
        pub fn get_valuetype_size() -> c_uint;
        pub fn get_fqn() -> c_uint;
        get_name_raw = pub fn get_name() -> *const c_char;
        get_namespace_raw = pub fn get_namespace() -> *const c_char;
        pub fn has_fieldptr_offset() -> bool;
        pub fn get_fieldptr_offset() -> c_int;
        pub fn get_num_methods() -> c_uint;
        pub fn get_num_fields() -> c_uint;
        pub fn get_num_properties() -> c_uint;
        pub fn is_derived_from(other: REFrameworkTypeDefinitionHandle) -> bool;
        pub fn is_valuetype() -> bool;
        pub fn is_enum() -> bool;
        pub fn is_by_ref() -> bool;
        pub fn is_pointer() -> bool;
        pub fn is_primitive() -> bool;
        pub fn get_vm_obj_type() -> REFrameworkVMObjType;
        pub fn get_instance() -> *mut c_void;
        pub fn create_instance(flags: c_uint) -> REFrameworkManagedObjectHandle => ManagedObject;
        pub fn get_parent_type() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_declaring_type() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_underlying_type() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_type_info() -> REFrameworkTypeInfoHandle => TypeInfo;
        pub fn get_runtime_type() -> REFrameworkManagedObjectHandle => ManagedObject;
        pub fn get_methods(out: *mut REFrameworkMethodHandle, out_size: c_uint, out_count: *mut c_uint) -> REFrameworkResult;
        pub fn get_fields(out: *mut REFrameworkFieldHandle, out_size: c_uint, out_count: *mut c_uint) -> REFrameworkResult;
        is_derived_from_by_name_raw = pub fn is_derived_from_by_name(name: *const c_char) -> bool;
        find_method_raw = pub fn find_method(name: *const c_char) -> REFrameworkMethodHandle => Method;
        find_field_raw = pub fn find_field(name: *const c_char) -> REFrameworkFieldHandle => Field;
        find_property_raw = pub fn find_property(name: *const c_char) -> REFrameworkPropertyHandle => Property;
        get_full_name_raw = pub fn get_full_name(out: *mut c_char, out_size: c_uint, out_len: *mut c_uint) -> REFrameworkResult;
    }

    pub fn get_name(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_name_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }

    pub fn get_namespace(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_namespace_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }

    pub fn is_derived_from_by_name(&self, name: &str) -> Option<bool> {
        with_cstrings!(name; self.is_derived_from_by_name_raw(name))
    }

    pub fn find_method(&self, name: &str) -> Option<Method> {
        with_cstrings!(name; self.find_method_raw(name))
    }

    pub fn find_field(&self, name: &str) -> Option<Field> {
        with_cstrings!(name; self.find_field_raw(name))
    }

    pub fn find_property(&self, name: &str) -> Option<Property> {
        with_cstrings!(name; self.find_property_raw(name))
    }

    // lwk kinda scuffed but its chill
    pub fn get_full_name(&self) -> Option<String> {
        let mut buf = vec![0u8; 512];
        let mut len: c_uint = 0;
        self.get_full_name_raw(buf.as_mut_ptr() as *mut c_char, buf.len() as c_uint, &mut len)?;
        buf.truncate(len as usize);
        String::from_utf8(buf).ok()
    }
}
