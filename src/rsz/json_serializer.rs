use serde::{Serialize, Serializer, ser::{SerializeMap, SerializeSeq}};

use crate::rsz::{InstanceView, RszContext, Value, ValueView};

impl<'a> Serialize for RszContext<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_seq(Some(self.rsz.roots.len()))?;
        for root in &self.rsz.roots {
            if let Some(instance) = self.rsz.instances.get(*root as usize) {
                let wrapped = InstanceView {
                    instance,
                    ctx: self,
                };
                state.serialize_element(&wrapped)?;
            }
        }
        state.end()
    }
}

impl<'a> Serialize for InstanceView<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let type_map = self.ctx.type_map;
        let instance = self.instance;
        let mut state = serializer.serialize_map(Some(instance.fields.len()))?;
        let type_info = type_map.get_by_hash(instance.hash).unwrap(); // TODO should throw error if not

        for (i, value) in instance.fields.iter().enumerate() {
            if let Some(field_info) = type_info.get_by_index(i) {
                let wrapped = ValueView {value, field_info, ctx: self.ctx};
                state.serialize_entry(&field_info.name, &wrapped)?;
            } else {
                eprintln!("Skipping field {i} in {}", type_info.name);
            }
        }

        state.end()
    }
}

macro_rules! serialize_rsz_values {
    (
        $self:expr, $serializer:expr,
        enums: { $( $enum_var:ident => $wrapper:ident($cast:ty) ),* $(,)? },
        direct: [ $( $direct:ident ),* $(,)? ],
        manual: { $( $pat:pat => $expr:expr ),* $(,)? }
    ) => {
        match $self.value {
            $(
                Value::$enum_var(v) => {
                    if let Some(name) = $self.get_enum_name(crate::enums::EnumValue::$wrapper(*v as $cast)) {
                        name.serialize($serializer)
                    } else {
                        v.serialize($serializer)
                    }
                }
            )*
            $( Value::$direct(v) => v.serialize($serializer),)*
            $( $pat => $expr,)*
        }
    };
}

impl<'a> Serialize for ValueView<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_rsz_values!(
            self, serializer,
            enums: {
                S8  => Signed(i64),
                U8  => Unsigned(u64),
                S16 => Signed(i64),
                U16 => Unsigned(u64),
                S32 => Signed(i64),
                U32 => Unsigned(u64),
                S64 => Signed(i64),
                U64 => Unsigned(u64),
            },
            direct: [
                Bool, F8, F16, F32, F64, Size, UInt2, UInt3, UInt4, 
                Int2, Int3, Int4, Float2, Float3, Float4, Vec2, Vec3, Vec4, 
                Quaternion, Sphere, Position, Color, Mat4x4, Guid, OBB, 
                AABB, Data, Range, RangeI, Rect, KeyFrame, RuntimeType
            ],
            manual: {
                Value::Null => serializer.serialize_none(),
                Value::String(v) | Value::Resource(v) => v.to_string().serialize(serializer),
                Value::GameObjectRef(v) => v.0.serialize(serializer),
                Value::Object(index) | Value::UserData(index) => {
                    if let Some(instance) = self.ctx.rsz.instances.get(*index as usize) {
                        let view = InstanceView { 
                            instance, 
                            ctx: self.ctx
                        };

                        if let Some(inner_value) = view.get_serializable_enum_value() {
                            let inner_view = ValueView {
                                value: inner_value,
                                field_info: self.field_info,
                                ctx: self.ctx
                            };
                            return inner_view.serialize(serializer)
                        }
                        view.serialize(serializer)
                    } else {
                        serializer.serialize_none()
                    }
                },
                Value::Array(arr) => {
                    let mut state = serializer.serialize_seq(Some(arr.len()))?;
                    for value in arr {
                        let view = ValueView { 
                            value,
                            field_info: self.field_info,
                            ctx: self.ctx,
                        };
                        state.serialize_element(&view)?;
                    }
                    state.end()
                }
            }
        )
    }
}
