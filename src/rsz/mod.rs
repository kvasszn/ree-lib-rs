pub mod map;
pub mod error;
pub mod rsz_type;
pub mod deserializer;
pub mod json_serializer;

pub use map::*;

use std::{collections::HashMap, io::{self, Read, Seek}};

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use half::f16;

use crate::{enums::{EnumMap, EnumValue}, rsz::{error::{Result, RszError}, rsz_type::RszType}, types::*, util::{read_pod, read_pod_vec}};

#[derive(Debug, Clone)]
pub struct Rsz {
    pub roots: Vec<u32>,
    pub instances: Vec<Instance>,
    pub externs: HashMap<u32, Extern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub hash: u32,
    pub fields: Vec<Value>,
}

pub struct RszContext<'a> {
    pub rsz: &'a Rsz,
    pub type_map: &'a RszMap,
    pub enum_map: &'a EnumMap,
}

pub struct InstanceView<'a> {
    pub instance: &'a Instance,
    pub ctx: &'a RszContext<'a>
}

impl<'a> InstanceView<'a> {

    pub fn get_type_info(&self) -> Option<&'a TypeInfo> {
        self.ctx.type_map.get_by_hash(self.instance.hash)
    }

    pub fn get(&self, field_name: &str) -> Option<&'a Value> {
        let field_idx = self.get_type_info()?.get_field_idx(field_name)?;
        self.instance.fields.get(field_idx)
    }

    /*pub fn get_object(&self, field_name: &str) -> Option<InstanceView<'a>> {
        let value = self.get(field_name)?;
        let object_id = value.as_object()?;
        
        let child_instance = self.rsz.instances.get(object_id as usize)?;

        let child_type_info = self.map.types.get(&child_instance.hash)?;

        Some(InstanceView {
            instance: child_instance,
            type_info: child_type_info,
            rsz: self.rsz,
            map: self.map,
        })
    }*/
}

pub struct ValueView<'a> {
    pub value: &'a Value,
    pub field_info: &'a FieldInfo,
    pub ctx: &'a RszContext<'a>,
}

impl<'a> ValueView<'a> {
    fn get_enum_name(&self, value: EnumValue) -> Option<&String> {
        let original_type = self.field_info.original_type.as_str();

        let base_name = match original_type.find('[') {
            Some(idx) => &original_type[..idx],
            None => original_type,
        };

        let enum_def = self.ctx.enum_map.get(base_name)?;
        enum_def.get_name(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extern {
    pub index: u32,
    pub r#type: String,
    pub path: String,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Pod, Zeroable)]
pub struct TypeDescriptor {
    pub hash: u32,
    pub crc: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Object(u32),
    Array(Vec<Value>),
    UserData(u32),
    Null,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    S8(i8),
    S16(i16),
    S32(i32),
    S64(i64),
    F8(u8),
    F16(f16),
    F32(f32),
    F64(f64),
    Size(u64),
    RuntimeType(RuntimeType),
    String(StringU16),
    Resource(StringU16),
    UInt2(UInt2),
    UInt3(UInt3),
    UInt4(UInt4),
    Int2(Int2),
    Int3(Int3),
    Int4(Int4),
    Float2(Float2),
    Float3(Float3),
    Float4(Float4),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Quaternion(Quaternion),
    Sphere(Sphere),
    Position(Position),
    Color(Color),
    Mat4x4(Box<Mat4x4>),
    Guid(Guid),
    OBB(Box<OBB>),
    AABB(Box<AABB>),
    Data(Data),
    Range(Range),
    RangeI(RangeI),
    Rect(Rect),
    GameObjectRef(GameObjectRef),
    KeyFrame(KeyFrame),
}

impl Value {
    pub fn as_i128(&self) -> Option<i128> {
        match self {
            Value::U8(v)  => Some(*v as i128),
            Value::U16(v) => Some(*v as i128),
            Value::U32(v) => Some(*v as i128),
            Value::U64(v) => Some(*v as i128),
            Value::S8(v)  => Some(*v as i128),
            Value::S16(v) => Some(*v as i128),
            Value::S32(v) => Some(*v as i128),
            Value::S64(v) => Some(*v as i128),
            Value::Size(v) => Some(*v as i128),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F8(v)  => Some(*v as f64),
            Value::F16(v) => Some(v.to_f64()),
            Value::F32(v) => Some(*v as f64),
            Value::F64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_string(&self) -> Option<&StringU16> {
        match self {
            Value::String(s) | Value::Resource(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_object_id(&self) -> Option<u32> {
        match self {
            Value::Object(id) | Value::UserData(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        if let Value::Array(arr) = self { Some(arr) } else { None }
    }

    pub fn as_bool(&self) -> Option<bool> { if let Value::Bool(v) = self { Some(*v) } else { None } }

    pub fn as_u8(&self)  -> Option<u8>  { if let Value::U8(v) = self { Some(*v) } else { None } }
    pub fn as_u16(&self) -> Option<u16> { if let Value::U16(v) = self { Some(*v) } else { None } }
    pub fn as_u32(&self) -> Option<u32> { if let Value::U32(v) = self { Some(*v) } else { None } }
    pub fn as_u64(&self) -> Option<u64> { if let Value::U64(v) = self { Some(*v) } else { None } }

    pub fn as_s8(&self)  -> Option<i8>  { if let Value::S8(v) = self { Some(*v) } else { None } }
    pub fn as_s16(&self) -> Option<i16> { if let Value::S16(v) = self { Some(*v) } else { None } }
    pub fn as_s32(&self) -> Option<i32> { if let Value::S32(v) = self { Some(*v) } else { None } }
    pub fn as_s64(&self) -> Option<i64> { if let Value::S64(v) = self { Some(*v) } else { None } }

    pub fn as_f32(&self) -> Option<f32> { if let Value::F32(v) = self { Some(*v) } else { None } }

    pub fn as_vec2(&self) -> Option<&Vec2> { if let Value::Vec2(v) = self { Some(v) } else { None } }
    pub fn as_vec3(&self) -> Option<&Vec3> { if let Value::Vec3(v) = self { Some(v) } else { None } }
    pub fn as_vec4(&self) -> Option<&Vec4> { if let Value::Vec4(v) = self { Some(v) } else { None } }

    pub fn as_int2(&self) -> Option<&Int2> { if let Value::Int2(v) = self { Some(v) } else { None } }
    pub fn as_int3(&self) -> Option<&Int3> { if let Value::Int3(v) = self { Some(v) } else { None } }
    pub fn as_int4(&self) -> Option<&Int4> { if let Value::Int4(v) = self { Some(v) } else { None } }

    pub fn as_float2(&self) -> Option<&Float2> { if let Value::Float2(v) = self { Some(v) } else { None } }
    pub fn as_float3(&self) -> Option<&Float3> { if let Value::Float3(v) = self { Some(v) } else { None } }
    pub fn as_float4(&self) -> Option<&Float4> { if let Value::Float4(v) = self { Some(v) } else { None } }

    pub fn as_quaternion(&self) -> Option<&Quaternion> { if let Value::Quaternion(v) = self { Some(v) } else { None } }
    pub fn as_color(&self)      -> Option<&Color>      { if let Value::Color(v) = self { Some(v) } else { None } }
    pub fn as_guid(&self)       -> Option<&Guid>       { if let Value::Guid(v) = self { Some(v) } else { None } }

    pub fn as_mat4x4(&self) -> Option<&Mat4x4> { if let Value::Mat4x4(v) = self { Some(v.as_ref()) } else { None } }
    pub fn as_obb(&self)    -> Option<&OBB>    { if let Value::OBB(v) = self { Some(v.as_ref()) } else { None } }
    pub fn as_aabb(&self)   -> Option<&AABB>   { if let Value::AABB(v) = self { Some(v.as_ref()) } else { None } }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct RszHeader {
    magic: [u8; 4],
    version: u32,
    root_count: u32,
    type_descriptor_count: u32,
    extern_count: u32,
    _padding: u32,
    type_descriptor_offset: u64,
    data_offset: u64,
    extern_offset: u64,
}

pub struct RszInfo {
    header: RszHeader,
    roots: Vec<u32>,
    extern_slots: HashMap<u32, (u32, String)>,
    type_descriptors: Vec<TypeDescriptor>,
}

impl RszInfo {
    // the reader should have its start be at teh RSZ magic
    // once this is done, data can be deserialized
    pub fn read<R: Read + Seek>(r: &mut R) -> Result<Self> {
        let h = read_pod::<RszHeader, R>(r)?;
        if &h.magic != b"RSZ\0" {
            return Err(RszError::InvalidMagic(*b"RSZ\0", h.magic))
        }

        if h.version != 0x10 {
            return Err(RszError::InvalidVersion(h.version, 0x10))
        }

        let roots = read_pod_vec::<u32, R>(r, h.root_count as usize)?;
        let type_descriptors = read_pod_vec::<TypeDescriptor, R>(r, h.type_descriptor_count as usize)?;

        if type_descriptors.first() != Some(&TypeDescriptor { hash: 0, crc: 0 }) {
            return Err(format!("The first type descriptor should be 0").into())
        }

        #[repr(C)]
        #[derive(Debug, Clone, Copy, Pod, Zeroable, PartialEq)]
        pub struct SlotInfo(pub u32, pub u32, pub u64);
        let extern_slot_info = read_pod_vec::<SlotInfo, R>(r, h.extern_count as usize)?;
        let extern_slots = extern_slot_info.into_iter()
            .map(|SlotInfo(slot, hash, _offset)| {
                let path = StringU16C::read_rsz(r)?;
                Ok((slot, (hash, path.as_string())))
            }).collect::<Result<HashMap<u32, (u32, String)>>>()?;

        r.seek(io::SeekFrom::Start(h.data_offset))?;
        Ok(Self {
            header: h,
            roots,
            type_descriptors,
            extern_slots
        })
    }
}

