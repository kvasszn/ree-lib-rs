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

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_name() {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "<unknown field>"),
        }
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field({}.{}: {} @ {:p})",
            self.get_declaring_type()
                .and_then(|t| t.get_full_name())
                .as_deref()
                .unwrap_or("?"),
            self.get_type()
                .and_then(|t| t.get_full_name())
                .as_deref()
                .unwrap_or("?"),
            self.get_name().unwrap_or("?"),
            self.0
        )
    }
}
