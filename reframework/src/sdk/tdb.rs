use crate::*;

impl Tdb {
    impl_handle_methods! {
        || &*Api::get().sdk.tdb;
        pub fn get_num_types() -> c_uint;
        pub fn get_num_methods() -> c_uint;
        pub fn get_num_fields() -> c_uint;
        pub fn get_num_properties() -> c_uint;
        pub fn get_strings_size() -> c_uint;
        pub fn get_raw_data_size() -> c_uint;
        get_string_database_raw = pub fn get_string_database() -> *const c_char;
        pub fn get_raw_database() -> *mut c_uchar;
        pub fn get_type(index: c_uint) -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        find_type_raw = pub fn find_type(name: *const c_char) -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn find_type_by_fqn(fqn: c_uint) -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_method(index: c_uint) -> REFrameworkMethodHandle => Method;
        find_method_raw = pub fn find_method(type_name: *const c_char, name: *const c_char) -> REFrameworkMethodHandle => Method;
        pub fn get_field(index: c_uint) -> REFrameworkFieldHandle => Field;
        find_field_raw = pub fn find_field(type_name: *const c_char, name: *const c_char) -> REFrameworkFieldHandle => Field;
        pub fn get_property(index: c_uint) -> REFrameworkPropertyHandle => Property;
        pub fn get_module(index: c_uint) -> REFrameworkModuleHandle => Module;
        pub fn get_num_modules() -> c_uint;
    }

    pub fn get_string_database(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_string_database_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }

    pub fn find_type(&self, name: &str) -> Option<TypeDefinition> {
        with_cstrings!(name; self.find_type_raw(name))
    }

    pub fn find_method(&self, type_name: &str, name: &str) -> Option<Method> {
        with_cstrings!(type_name, name; self.find_method_raw(type_name, name))
    }

    pub fn find_field(&self, type_name: &str, name: &str) -> Option<Field> {
        with_cstrings!(type_name, name; self.find_field_raw(type_name, name))
    }
}
