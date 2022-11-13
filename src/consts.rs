#![allow(dead_code)]
use cgmath::{Vector2, Vector3};
use image::Rgb;

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vertices = Vec<Vec3>;
pub type Indices = (usize, usize, usize);
pub type Shade = (f32, f32, f32);

pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
pub const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
pub const DARK_GREEN: Rgb<u8> = Rgb([53, 94, 59]);
pub const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

pub const CANVAS_SIZE: u32 = 600;
pub const VIEWPORT_SIZE: u32 = 1;
pub const D: f32 = 1.;
