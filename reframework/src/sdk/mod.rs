pub mod tdb;
pub mod type_definition;
pub mod method;
pub mod field;
pub mod property;
pub mod resource_manager;
pub mod resource;
pub mod type_info;
pub mod vm_context;
pub mod reflection_method;
pub mod reflection_property;
pub mod managed_object;
pub mod module;

use crate::*;

impl_free_fns! {
    || &*(*Api::get().sdk).functions;
    pub fn get_tdb() -> REFrameworkTDBHandle => Tdb;
    pub fn get_resource_manager() -> REFrameworkResourceManagerHandle => ResourceManager;
    pub fn get_vm_context() -> REFrameworkVMContextHandle => VMContext;
    get_managed_singleton_raw = pub fn get_managed_singleton(type_name: *const c_char) -> REFrameworkManagedObjectHandle => ManagedObject;
    get_native_singleton_raw = pub fn get_native_singleton(type_name: *const c_char) -> *mut c_void;
    pub fn get_managed_singletons(out: *mut REFrameworkManagedSingleton, out_size: c_uint, out_count: *mut c_uint) -> REFrameworkResult;
    pub fn get_native_singletons(out: *mut REFrameworkNativeSingleton, out_size: c_uint, out_count: *mut c_uint) -> REFrameworkResult;
    pub fn create_managed_string(str_: *const wchar_t) -> REFrameworkManagedObjectHandle => ManagedObject;
    pub fn create_managed_string_normal(str_: *const c_char) -> REFrameworkManagedObjectHandle => ManagedObject;
    pub fn add_hook(method: REFrameworkMethodHandle, pre: REFPreHookFn, post: REFPostHookFn, ignore_jmp: bool) -> c_uint;
    pub fn remove_hook(method: REFrameworkMethodHandle, id: c_uint) -> ();
    pub fn allocate(size: c_ulonglong) -> *mut c_void;
    pub fn deallocate(ptr: *mut c_void) -> ();
    pub fn create_managed_array(type_def: REFrameworkTypeDefinitionHandle, size: c_uint) -> REFrameworkManagedObjectHandle => ManagedObject;
    typeof_raw = pub fn typeof_(type_name: *const c_char) -> REFrameworkManagedObjectHandle => ManagedObject;
}

pub fn type_of(type_name: &str) -> Option<ManagedObject> {
    with_cstrings!(type_name; typeof_raw(type_name))
}

pub fn get_managed_singleton(type_name: &str) -> Option<ManagedObject> {
    with_cstrings!(type_name; get_managed_singleton_raw(type_name))
}

pub fn get_native_singleton(type_name: &str) -> Option<*mut c_void> {
    with_cstrings!(type_name; get_native_singleton_raw(type_name))
}
