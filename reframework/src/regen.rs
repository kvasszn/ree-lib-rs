#[repr(C)]
pub struct ManagedObject<T> {
    vftable: *const std::ffi::c_void,
    ref_count: u32,
    _unk: u32,
    obj: T,
}

#[repr(transparent)]
pub struct ManagedPtr<T> {
    pub ptr: *mut T,
}

impl<T> Clone for ManagedPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ManagedPtr<T> {}

#[repr(C)]
pub struct Array<T> {
    vftable: *const std::ffi::c_void,
    ref_count: u32,
    _unk: u32,
    contained_type: *const std::ffi::c_void,
    count: u32,
    n: u32,
    obj: [T; 0]
}
