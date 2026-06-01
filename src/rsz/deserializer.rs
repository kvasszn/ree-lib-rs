use std::{collections::HashMap, error::Error, io::{Read, Seek}};

use byteorder::{LE, ReadBytesExt};
use half::f16;

use crate::{rsz::{Extern, FieldInfo, Instance, Rsz, RszInfo, RszMap, TypeDescriptor, TypeInfo, Value, rsz_type::RszType}, util::{ReadStringExt, seek_align_up}};
use crate::types::*;


macro_rules! match_rsz_types {
    (
        $type_str:expr, 
        $data:expr,
        direct: [ $( $direct:ident ),* $(,)? ],
        boxed:  [ $( $boxed:ident ),* $(,)? ],
        manual: { $( $pat:pat => $expr:expr ),* $(,)? }
    ) => {
        match $type_str {
            $(
                stringify!($direct) => Value::$direct(<$direct>::read_rsz($data)?),
            )*
            $(
                stringify!($boxed) => Value::$boxed(Box::new(<$boxed>::read_rsz($data)?)),
            )*
            $(
                $pat => $expr,
            )*
            _ => return Err(format!("Unknown or unsupported type: {}", $type_str).into()),
        }
    };
}

pub struct RszDeserializer<'a, R: Read + Seek> {
    data: R,
    rsz_map: &'a RszMap,
    info: &'a RszInfo,
}

impl<'a, R: Read + Seek> RszDeserializer<'a, R> {
    pub fn from_rsz_info(data: R, info: &'a RszInfo, rsz_map: &'a RszMap) -> Self {
        Self {
            data,
            rsz_map,
            info
        }
    }

    pub fn deserialize(&mut self) -> Result<Rsz, Box<dyn Error>> {
        let mut instances: Vec<Instance> = Vec::new();
        let mut externs = HashMap::new();
        for (i, TypeDescriptor {hash, ..}) in self.info.type_descriptors.iter().enumerate() {
            let type_info = self.rsz_map.get_by_hash(*hash).ok_or("hash not found in type map")?;
            log::debug!("class: {}", type_info.name);
            
            if let Some(extern_slot) = self.info.extern_slots.get(&(i as u32)) {
                externs.insert(i as u32, Extern {
                    index: i as u32,
                    path: extern_slot.1.clone(),
                    r#type: type_info.name.clone()
                });
            }

            let mut fields = Vec::new();
            for (_hash, field) in &type_info.fields {
                log::debug!("field: {}", field.name);
                let value = self.deserialize_field(field, type_info)?;
                log::debug!("value: {:?}", value);
                fields.push(value);
            }
            instances.push(Instance {hash: *hash, fields});
        }
        Ok(Rsz {
            roots: self.info.roots.clone(),
            instances,
            externs,
        })
    }


    fn deserialize_field(&mut self, field: &FieldInfo, parent: &TypeInfo) -> Result<Value, Box<dyn Error>> {
        let value = if field.array {
            seek_align_up(&mut self.data, 4)?;
            let len = self.data.read_u32::<LE>()?;
            let mut arr_vals = Vec::new();
            for _ in 0..len {
                seek_align_up(&mut self.data, field.align as u64)?;
                let value = self.deserialize_field_single(field, parent)?;
                arr_vals.push(value);
            }
            Value::Array(arr_vals)
        } else {
            seek_align_up(&mut self.data, field.align as u64)?;
            let value = self.deserialize_field_single(field, parent)?;
            value
        };
        Ok(value)
    }


    fn deserialize_field_single(&mut self, field: &FieldInfo, _parent: &TypeInfo) -> Result<Value, Box<dyn Error>> {
        let value = match_rsz_types!(
            field.r#type.as_str(),
            &mut self.data,

            direct: [
                UInt2, UInt3, UInt4, Int2, Int3, Int4, Float2, Float3, Float4,
                Vec2, Vec3, Quaternion, Sphere, Position, Color, Guid, Data, 
                Range, RangeI, Rect, KeyFrame, GameObjectRef
            ],

            boxed: [
                Mat4x4, OBB, AABB
            ],

            manual: {
                "Bool" => Value::Bool(self.data.read_u8()? != 0),
                "U8"   => Value::U8(self.data.read_u8()?),
                "U16"  => Value::U16(self.data.read_u16::<LE>()?),
                "U32"  => Value::U32(self.data.read_u32::<LE>()?),
                "U64"  => Value::U64(self.data.read_u64::<LE>()?),
                "S8"   => Value::S8(self.data.read_i8()?),
                "S16"  => Value::S16(self.data.read_i16::<LE>()?),
                "S32"  => Value::S32(self.data.read_i32::<LE>()?),
                "S64"  => Value::S64(self.data.read_i64::<LE>()?),
                "F8"   => Value::F8(self.data.read_u8()?),
                "F16"  => Value::F16(f16::from_bits(self.data.read_u16::<LE>()?)),
                "F32"  => Value::F32(self.data.read_f32::<LE>()?),
                "F64"  => Value::F64(self.data.read_f64::<LE>()?),
                "Size" => Value::Size(self.data.read_u64::<LE>()?),
                "Object" | "UserData" => Value::Object(self.data.read_u32::<LE>()?),
                "String" | "Resource" => Value::String(StringU16::read_rsz(&mut self.data)?),
                "RuntimeType"   => Value::RuntimeType(crate::types::RuntimeType(self.data.read_u8str()?)),
            }
        );

        Ok(value)
    }
}
