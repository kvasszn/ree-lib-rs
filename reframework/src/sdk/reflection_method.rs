use crate::*;
impl ReflectionMethod {
    impl_handle_methods! {
        || &*Api::get().sdk.reflection_method;
        pub fn get_function() -> REFrameworkInvokeMethod;
    }
}

