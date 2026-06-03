use crate::*;
impl ReflectionProperty {
    impl_handle_methods! {
        || &*Api::get().sdk.reflection_property;
        pub fn get_getter() -> REFrameworkReflectionPropertyMethod;
        pub fn is_static() -> bool;
        pub fn get_size() -> c_uint;
    }
}
