use crate::*;
impl Method {
    impl_handle_methods! {
        ||  &*Api::get().sdk.method ;
        pub fn get_function() -> *mut c_void;
        get_name_raw = pub fn get_name() -> *const c_char;
        pub fn get_declaring_type() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_return_type() -> REFrameworkTypeDefinitionHandle => TypeDefinition;
        pub fn get_num_params() -> c_uint;
        pub fn get_index() -> c_uint;
        pub fn get_virtual_index() -> c_int;
        pub fn is_static() -> bool;
        pub fn get_flags() -> c_ushort;
        pub fn get_impl_flags() -> c_ushort;
        pub fn get_invoke_id() -> c_uint;
        pub fn invoke(thisptr: *mut c_void, in_args: *mut *mut c_void, in_args_size: c_uint, out: *mut c_void, out_size: c_uint) -> REFrameworkResult;
        pub fn get_params(out: *mut REFrameworkMethodParameter, out_size: c_uint, out_len: *mut c_uint) -> REFrameworkResult;
    }

    pub fn get_name(&self) -> Option<&str> {
        unsafe {
            let ptr = self.get_name_raw()?;
            CStr::from_ptr(ptr).to_str().ok()
        }
    }
}
