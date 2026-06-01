pub mod guid;
pub mod strings;

pub use guid::*;
pub use strings::*;

use std::{fmt::{Debug}};

use bytemuck::{Pod, Zeroable};
use half::f16;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Object {
    pub hash: u32,
    pub index: u32,
}

pub type UserData = Object;

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable, PartialEq)]
pub struct Range {
    pub start: f32,
    pub end: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable, PartialEq)]
pub struct RangeI {
    pub start: i32,
    pub end: i32,
}

pub type UInt2 = [u32; 2];
pub type UInt3 = [u32; 3];
pub type UInt4 = [u32; 4];
pub type Int2 = [i32; 2];
pub type Int3 = [i32; 3];
pub type Int4 = [i32; 4];
pub type Float2 = [f32; 2];
pub type Float3 = [f32; 3];
pub type Float4 = [f32; 4];

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable, PartialEq)]
pub struct Vec2(pub f32, pub f32, pub f32, pub f32);

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32, pub f32);

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable, PartialEq)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct Quaternion(f32, f32, f32, f32);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct Sphere(f32, f32, f32, f32);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct Position(f64, f64, f64);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct Mat4x4(pub [f32; 16]);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RuntimeType(pub String);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct GameObjectRef(pub Guid);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct OBB {
    center: Vec3,
    half_extents: Vec3,
    orientation: [Vec3; 3], // local axes (right, up, forward)
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct AABB(pub Vec4, pub Vec4);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct Rect {
    start: UInt2,
    end: UInt2,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Data(pub Vec<u8>);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct KeyFrame{
    time: f32,
    val: [f32; 3],
}

/*
 * Native Structs/Custom impls
 */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationCurve3D {
    pub xkeys: Vec<AnimationCurveKey>,
    pub ykeys: Vec<AnimationCurveKey>,
    pub zkeys: Vec<AnimationCurveKey>,
    pub min_value: f32,
    pub max_value: f32,
    pub min_time: f32,
    pub max_time: f32,
    pub loop_count: u32,
    pub loop_wrap_no: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AnimationCurveKey {
    pub value: f32,
    pub curve_type: u16,
    pub time: f16,
    pub in_normal_x: f16,
    pub in_normal_y: f16,
    pub out_normal_x: f16,
    pub out_normal_y: f16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimationCurve {
    pub keys: Vec<AnimationCurveKey>,
    pub min_value: f32,
    pub max_value: f32,
    pub min_time: f32,
    pub max_time: f32,
    pub loop_count: u32,
    pub loop_wrap_no: u32,
}


#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Pod, Zeroable)]
pub struct Mandrake {
    pub v: i64,
    pub m: i64, // maybe change to NonZeroU64
}

impl Mandrake {
    pub fn to_buf(self) -> [u8; size_of::<Self>()] {
        let mut buf = [0u8; size_of::<Self>()]; 
        buf[0..8].copy_from_slice(&self.v.to_le_bytes());
        buf[8..16].copy_from_slice(&self.m.to_le_bytes());
        buf
    }

    pub fn set(&mut self, n: i64) {
        self.v = n * self.m 
    }

    pub fn get(&self) -> Option<i64> {
        if self.m == 0 {
            None
        } else {
            Some(self.v / self.m)
        }
    }
}
