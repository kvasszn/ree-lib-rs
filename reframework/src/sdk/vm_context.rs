use crate::*;
impl VMContext {
    impl_handle_methods! {
        || &*Api::get().sdk.vm_context;
        pub fn has_exception() -> bool;
        pub fn unhandled_exception() -> ();
        pub fn local_frame_gc() -> ();
        pub fn cleanup_after_exception(old_reference_count: c_int) -> ();
    }
}
