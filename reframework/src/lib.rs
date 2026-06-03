pub mod ref_log;
pub mod sdk;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/reframework_bindings.rs"));
}
pub use sys::*;

pub use ref_log::*;

use std::sync::OnceLock;
use std::{ffi::{CString, c_int, c_ushort, c_char, c_uchar, c_void, c_uint, c_ulonglong, CStr}};

static REF_API: OnceLock<Api> = OnceLock::new();

pub struct Api {
    pub reframework_module: *mut c_void,
    //version: &'static REFrameworkPluginVersion,
    //functions: &'static REFrameworkPluginFunctions,
    //renderer_data: &'static REFrameworkRendererData,
    //sdk: &'static REFrameworkSDKData,
    version: PluginVersion,
    functions: &'static REFrameworkPluginFunctions,
    renderer_data: &'static REFrameworkRendererData,
    sdk: &'static REFrameworkSDKData,
}

unsafe impl Send for Api {}
unsafe impl Sync for Api {}

impl Api {
    pub unsafe fn initialize(param: *const REFrameworkPluginInitializeParam) -> Result<(), &'static str> {
        let raw = unsafe { param.as_ref().ok_or("REF Initialize Param is null")? };

        let api = Self {
            reframework_module: raw.reframework_module,
            version: PluginVersion::from_raw(raw.version)
                .ok_or("Version pointer was null")?,
            functions: unsafe { raw.functions.as_ref() }
                .ok_or("Functions pointer was null")?,
            renderer_data: unsafe { raw.renderer_data.as_ref() }
                .ok_or("Renderer data pointer was null")?,
            sdk: unsafe { raw.sdk.as_ref() }
                .ok_or("SDK data pointer was null")?,
        };

        REF_API.set(api).map_err(|_| "REF API is already intialized")?;
        Ok(())
    }

    #[inline(always)]
    pub fn get() -> &'static Self {
        let msg = "Attempted to access REFramework before it was initialized";
        REF_API.get()
            .or_else(|| {log_to_file!("{}", msg); None})
            .expect(msg)
    }

    #[inline(always)]
    pub fn try_get() -> Option<&'static Self> {
        REF_API.get()
    }
}

macro_rules! define_wrapper {
    (
        $(
            $wrapper_name:ident = *const $raw_ty:ident;
        )*
    ) => {
        $(
            #[derive(Debug, Copy, Clone)]
            #[repr(transparent)]
            pub struct $wrapper_name(*const $raw_ty);

            impl $wrapper_name {
                pub fn from_raw(ptr: *const $raw_ty) -> Option<Self> {
                    if ptr.is_null() { None } else { Some(Self(ptr)) }
                }
                pub fn as_ptr(self) -> *const $raw_ty { self.0 }
            }
        )*
    };
    (
        $(
            $wrapper_name:ident = *mut $raw_ty:ident;
        )*
    ) => {
        $(
            #[derive(Debug, Copy, Clone)]
            #[repr(transparent)]
            pub struct $wrapper_name($raw_ty);

            impl $wrapper_name {
                pub fn from_raw(ptr: $raw_ty) -> Option<Self> {
                    if ptr.is_null() { None } else { Some(Self(ptr)) }
                }
                pub fn as_ptr(self) -> $raw_ty { self.0 }
            }
        )*
    };
}

define_wrapper! {
    PluginInitializeParam = *const REFrameworkPluginInitializeParam;
    PluginVersion = *const REFrameworkPluginVersion;
    Sdk = *const REFrameworkSDKData;
}

define_wrapper! {
    Tdb                 = *mut REFrameworkTDBHandle;
    TypeDefinition      = *mut REFrameworkTypeDefinitionHandle;
    Method              = *mut REFrameworkMethodHandle;
    Field               = *mut REFrameworkFieldHandle;
    Property            = *mut REFrameworkPropertyHandle;
    ManagedObject       = *mut REFrameworkManagedObjectHandle;
    ResourceManager     = *mut REFrameworkResourceManagerHandle;
    Resource            = *mut REFrameworkResourceHandle;
    TypeInfo            = *mut REFrameworkTypeInfoHandle;
    VMContext           = *mut REFrameworkVMContextHandle;
    ReflectionProperty  = *mut REFrameworkReflectionPropertyHandle;
    ReflectionMethod    = *mut REFrameworkReflectionMethodHandle;
    Module              = *mut REFrameworkModuleHandle;
}

macro_rules! impl_handle_methods {
    ($get_fns:expr;) => {};

    // named + wrapped: rust_name = fn c_name(&self, ...) -> RawType => WrapperType
    (
        $get_fns:expr;
        $rust_name:ident = $pub:vis fn $c_name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty => $wrapper:ty;
        $($rest:tt)*
    ) => {
        $pub fn $rust_name(&self, $($arg: $arg_ty),*) -> Option<$wrapper> {
            unsafe {
                let func = ($get_fns)().$c_name?;
                <$wrapper>::from_raw(func(self.0, $($arg),*))
            }
        }
        impl_handle_methods!($get_fns; $($rest)*);
    };

    // shorthand wrapped: fn name(...) -> RawType => WrapperType
    (
        $get_fns:expr;
        $pub:vis fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty => $wrapper:ty;
        $($rest:tt)*
    ) => {
        impl_handle_methods! {
            $get_fns;
            $name = $pub fn $name($($arg: $arg_ty),*) -> $ret => $wrapper;
            $($rest)*
        }
    };

    // named + unwrapped: rust_name = fn c_name(...) -> RawType
    (
        $get_fns:expr;
        $rust_name:ident = $pub:vis fn $c_name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
        $($rest:tt)*
    ) => {
        $pub fn $rust_name(&self, $($arg: $arg_ty),*) -> Option<$ret> {
            unsafe {
                let func = ($get_fns)().$c_name?;
                Some(func(self.0, $($arg),*))
            }
        }
        impl_handle_methods!($get_fns; $($rest)*);
    };

    // shorthand unwrapped: fn name(...) -> RawType
    (
        $get_fns:expr;
        $pub:vis fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
        $($rest:tt)*
    ) => {
        impl_handle_methods! {
            $get_fns;
            $name = $pub fn $name($($arg: $arg_ty),*) -> $ret;
            $($rest)*
        }
    };
}

macro_rules! impl_free_fns {
    ($get_fns:expr;) => {};

    // named + wrapped: rust_name = fn c_name(...) -> RawType => WrapperType
    (
        $get_fns:expr;
        $rust_name:ident = $pub:vis fn $c_name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty => $wrapper:ty;
        $($rest:tt)*
    ) => {
        $pub fn $rust_name($($arg: $arg_ty),*) -> Option<$wrapper> {
            unsafe {
                let func = ($get_fns)().$c_name?;
                <$wrapper>::from_raw(func($($arg),*))
            }
        }
        impl_free_fns!($get_fns; $($rest)*);
    };

    // shorthand wrapped: fn name(...) -> RawType => WrapperType
    (
        $get_fns:expr;
        $pub:vis fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty => $wrapper:ty;
        $($rest:tt)*
    ) => {
        impl_free_fns! {
            $get_fns;
            $name = $pub fn $name($($arg: $arg_ty),*) -> $ret => $wrapper;
            $($rest)*
        }
    };

    // named + unwrapped: rust_name = fn c_name(...) -> RawType
    (
        $get_fns:expr;
        $rust_name:ident = $pub:vis fn $c_name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
        $($rest:tt)*
    ) => {
        $pub fn $rust_name($($arg: $arg_ty),*) -> Option<$ret> {
            unsafe {
                let func = ($get_fns)().$c_name?;
                Some(func($($arg),*))
            }
        }
        impl_free_fns!($get_fns; $($rest)*);
    };

    // shorthand unwrapped: fn name(...) -> RawType
    (
        $get_fns:expr;
        $pub:vis fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
        $($rest:tt)*
    ) => {
        impl_free_fns! {
            $get_fns;
            $name = $pub fn $name($($arg: $arg_ty),*) -> $ret;
            $($rest)*
        }
    };
}

macro_rules! with_cstrings {
    (
        $($arg:ident),*;
        $body:expr
    ) => {{
        $(
            let $arg = ::std::ffi::CString::new($arg).ok()?;
            let $arg = $arg.as_ptr();
        )*
        $body
    }};
}

pub(crate) use with_cstrings;
pub(crate) use impl_handle_methods;
pub(crate) use impl_free_fns;

impl_free_fns! {
    || &*Api::get().functions;
    pub fn on_lua_state_created(arg1: REFLuaStateCreatedCb) -> bool;
    pub fn on_lua_state_destroyed(arg1: REFLuaStateDestroyedCb) -> bool;
    on_pre_application_entry_raw = pub fn on_pre_application_entry(arg1: *const c_char, arg2: REFOnPreApplicationEntryCb) -> bool;
    on_post_application_entry_raw = pub fn on_post_application_entry(arg1: *const c_char, arg2: REFOnPostApplicationEntryCb) -> bool;
    pub fn lock_lua() -> ();
    pub fn unlock_lua() -> ();
    pub fn on_device_reset(arg1: REFOnDeviceResetCb) -> bool;
    pub fn on_message(arg1: REFOnMessageCb) -> bool;
    pub fn create_script_state() -> *mut lua_State;
    pub fn delete_script_state(arg1: &mut lua_State) -> ();
    pub fn on_present(cb: REFOnPresentCb) -> bool;
    pub fn on_imgui_frame(cb: REFOnImGuiFrameCb) -> bool;
    pub fn on_imgui_draw_ui(cb: REFOnImGuiDrawUICb) -> bool;
    pub fn on_pre_gui_draw_element(cb: REFOnPreGuiDrawElementCb) -> bool;
}

pub fn on_pre_application_entry(arg1: &str, arg2: REFOnPreApplicationEntryCb) -> Option<bool> {
    with_cstrings!(arg1; on_pre_application_entry_raw(arg1, arg2))
}

pub fn on_post_application_entry(arg1: &str, arg2: REFOnPostApplicationEntryCb) -> Option<bool> {
    with_cstrings!(arg1; on_post_application_entry_raw(arg1, arg2))
}

fn log_internal(msg: &str, func_ptr: Option<unsafe extern "C" fn(*const std::os::raw::c_char, ...)>) {
    let Some(func) = func_ptr else { return } ;
    let Ok(c_msg) = CString::new(msg) else { return };
    unsafe { func(c_msg.as_ptr(), ""); }
}

pub fn log_info(msg: &str) {
    log_internal(msg, Api::get().functions.log_info)
}

pub fn log_warn(msg: &str) {
    log_internal(msg, Api::get().functions.log_warn)
}

pub fn log_error(msg: &str) {
    log_internal(msg, Api::get().functions.log_error)
}
