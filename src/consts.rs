#![allow(dead_code)]
use cgmath::{Matrix4, Vector2, Vector3};
use pixels::wgpu::Color;

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;
pub type Mat4 = Matrix4<f64>;
pub type Vertices = Vec<Vec3>;
pub type Indices = (usize, usize, usize);
pub type Shade = (f32, f32, f32);

pub const WHITE: [u8; 3] = [255, 255, 255];
pub const GREEN: [u8; 3] = [0, 255, 0];
pub const DARK_GREEN: [u8; 3] = [53, 94, 59];
pub const BLACK: [u8; 3] = [0, 0, 0];
pub const GREY: Color = Color {
    r: 0.01,
    g: 0.01,
    b: 0.01,
    a: 1.0,
};

pub const CANVAS_SIZE: u32 = 600;
pub const VIEWPORT_SIZE: u32 = 1;
pub const D: f64 = 1.;
