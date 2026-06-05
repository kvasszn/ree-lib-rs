#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ImVec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ImVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl ImVec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

impl ImVec4 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }
}

impl From<(f32, f32)> for ImVec2 {
    fn from((x, y): (f32, f32)) -> Self { Self { x, y } }
}

impl From<[f32; 2]> for ImVec2 {
    fn from([x, y]: [f32; 2]) -> Self { Self { x, y } }
}
