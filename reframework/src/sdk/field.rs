use crate::*;
impl Field {
    impl_handle_methods! {
        || &*Api::get().sdk.field;
        get_name_raw = pub fn get_name() -> *const c_char;
        pub fn get_declaring_type() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_type() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_offset_from_base() -> c_uint;
        pub fn get_offset_from_fieldptr() -> c_uint;
        pub fn get_flags() -> c_uint;
        pub fn is_static() -> bool;
        pub fn is_literal() -> bool;
        pub fn get_init_data() -> *mut c_void;
        pub fn get_data_raw(obj: *mut c_void, is_value_type: bool) -> *mut c_void;
        pub fn get_index() -> c_uint;
    }

    pub fn get_name(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_name_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }
}
