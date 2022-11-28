#![allow(dead_code)]
use cgmath::{Matrix4, Vector2, Vector3};
use pixels::wgpu::Color;

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;
pub type Mat4 = Matrix4<f64>;
pub type Vertices = Vec<Vec3>;
pub type Indices = (usize, usize, usize);
pub type Shade = (f32, f32, f32);
pub type Buffer = Vec<Vec<Option<f64>>>;

pub const WHITE: [u8; 3] = [255, 255, 255];
pub const GREEN: [u8; 3] = [0, 255, 0];
pub const DARK_GREEN: [u8; 3] = [53, 94, 59];
pub const BLACK: [u8; 3] = [0, 0, 0];
pub const BCK: [u8; 3] = [123, 223, 23];
pub const BLAK: [u8; 3] = [65, 130, 231];
pub const BLCK: [u8; 3] = [231, 65, 12];
pub const GREY: Color = Color {
    r: 0.01,
    g: 0.01,
    b: 0.01,
    a: 1.0,
};
pub const TRANS_SPEED: f64 = 10.;
pub const DEFAULT_TRANSL: Vec3 = Vec3::new(-2., 0., 7.);
pub const WINDOW_SIZE: u32 = 720;
pub const CANVAS_SIZE: u32 = 900;
pub const VIEWPORT_SIZE: u32 = 1;
pub const D: f64 = 1.;
