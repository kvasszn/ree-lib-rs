use crate::*;
impl Module {
    impl_handle_methods! {
        || &*Api::get().sdk.module;
        pub fn get_major() -> c_ushort;
        pub fn get_minor() -> c_ushort;
        pub fn get_build() -> c_ushort;
        pub fn get_revision() -> c_ushort;
        get_assembly_name_raw = pub fn get_assembly_name() -> *const c_char;
        get_location_raw = pub fn get_location() -> *const c_char;
        get_module_name_raw = pub fn get_module_name() -> *const c_char;
        pub fn get_num_types() -> c_uint;
        pub fn get_types() -> *mut c_uint;
        pub fn get_num_methods() -> c_uint;
        pub fn get_methods() -> *mut c_uint;
        pub fn get_num_instantiated_methods() -> c_uint;
        pub fn get_instantiated_methods() -> *mut c_uint;
        pub fn get_num_member_references() -> c_uint;
        pub fn get_member_references() -> *mut c_uint;
    }

    pub fn get_assembly_name(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_assembly_name_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }

    pub fn get_location(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_location_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }

    pub fn get_module_name(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_module_name_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let assembly = self.get_assembly_name().unwrap_or("<unknown>");
        let version = match (self.get_major(), self.get_minor(), self.get_build(), self.get_revision()) {
            (Some(maj), Some(min), Some(build), Some(rev)) => 
                format!("{}.{}.{}.{}", maj, min, build, rev),
            _ => "?.?.?.?".to_string(),
        };
        write!(f, "{}({})", assembly, version)
    }
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Module({} @ {:p})", 
            self.get_assembly_name().unwrap_or("?"),
            self.0
        )
    }
}
