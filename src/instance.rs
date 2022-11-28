use crate::{clipping::BoundingSphere, consts::*, draw_line, draw_triangle};
use cgmath::*;

#[derive(Debug, Clone)]
pub struct Instance {
    model: Model,
    verts: Vertices,
    triangles: Vec<Indices>,
    pub bounding_sphere: BoundingSphere,
    pub trans: Transform,
}
impl Instance {
    pub fn new(model: Model, translation: Vec3, scale: f64) -> Self {
        let trans = Transform::new(translation, scale);
        let verts = model.get_verts();
        let bounding_sphere = BoundingSphere::new(&verts);
        Self {
            model,
            verts,
            triangles: model.get_indices(),
            bounding_sphere,
            trans,
        }
    }
    pub fn Render(&self, frame: &mut [u8], cam_trans: Transform, depth_buffer: &mut Buffer) {
        let mut projected = Vec::new();
        for vert in &self.verts {
            let mut vert = *vert;
            self.trans.apply_transform(&mut vert, cam_trans);
            projected.push(project_vertex(vert));
        }

        let colors = vec![WHITE, GREEN, DARK_GREEN, BCK, BLAK, BLCK];
        let mut i = 0;

        for tri in &self.triangles {
            //render_wireframe_triangle(*tri, &projected, frame, colors[i]);
            render_filled_triangle(
                *tri,
                &projected,
                &self.verts,
                depth_buffer,
                frame,
                colors[i],
            );
            if i >= colors.len() - 1 {
                i = 0;
            } else {
                i += 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub scale: f64,
    pub rot: f64,
}
impl Transform {
    pub fn new(translation: Vec3, scale: f64) -> Self {
        Self {
            translation,
            scale,
            rot: 0.,
        }
    }
    fn apply_transform(&self, vert: &mut Vec3, cam_trans: Transform) {
        let rot_self: Basis3<f64> = Rotation3::from_angle_y(Deg(self.rot));
        let rot_cam: Basis3<f64> = Rotation3::from_angle_y(Deg(cam_trans.rot));

        *vert = rot_self.rotate_vector(*vert) * self.scale;
        *vert += self.translation + cam_trans.translation + DEFAULT_TRANSL;
        *vert = rot_cam.rotate_vector(*vert);
    }
}

fn render_wireframe_triangle(
    tri: Indices,
    projected: &Vec<Vec2>,
    frame: &mut [u8],
    color: [u8; 3],
) {
    draw_wireframe_triangle(
        projected[tri.0],
        projected[tri.1],
        projected[tri.2],
        frame,
        color,
    );
}
fn render_filled_triangle(
    tri: Indices,
    projected: &Vec<Vec2>,
    verts: &Vertices,
    depth_buffer: &mut Buffer,
    frame: &mut [u8],
    color: [u8; 3],
) {
    let verts_z = (verts[tri.0].z, verts[tri.1].z, verts[tri.2].z);
    draw_triangle(
        projected[tri.0],
        projected[tri.1],
        projected[tri.2],
        verts_z,
        depth_buffer,
        frame,
        color,
    );
}

fn draw_wireframe_triangle(p0: Vec2, p1: Vec2, p2: Vec2, frame: &mut [u8], color: [u8; 3]) {
    draw_line(p0, p1, frame, color);
    draw_line(p1, p2, frame, color);
    draw_line(p2, p0, frame, color);
}

#[derive(Debug, Clone, Copy)]
pub enum Model {
    Cube,
}
impl Model {
    pub fn get_verts(&self) -> Vertices {
        match self {
            Model::Cube => {
                vec![
                    Vec3::new(1., 1., 1.),
                    Vec3::new(-1., 1., 1.),
                    Vec3::new(-1., -1., 1.),
                    Vec3::new(1., -1., 1.),
                    Vec3::new(1., 1., -1.),
                    Vec3::new(-1., 1., -1.),
                    Vec3::new(-1., -1., -1.),
                    Vec3::new(1., -1., -1.),
                ]
            }
        }
    }
    pub fn get_indices(&self) -> Vec<Indices> {
        match self {
            Model::Cube => {
                vec![
                    (0, 1, 2),
                    (0, 2, 3),
                    (4, 0, 3),
                    (4, 3, 7),
                    (5, 4, 7),
                    (5, 7, 6),
                    (1, 5, 6),
                    (1, 6, 2),
                    (4, 5, 1),
                    (4, 1, 0),
                    (2, 6, 7),
                    (2, 7, 3),
                ]
            }
        }
    }
}

fn project_vertex(vert: Vec3) -> Vec2 {
    viewport_to_canvas(vert.x * D / vert.z, vert.y * D / vert.z)
}

fn viewport_to_canvas(x: f64, y: f64) -> Vec2 {
    let (canvas_size, viewport_size) = (CANVAS_SIZE as f64, VIEWPORT_SIZE as f64);
    Vec2::new(
        x * canvas_size / viewport_size,
        y * canvas_size / viewport_size,
    )
}

pub fn pr<T: std::fmt::Display>(s: T) {
    println!("{}", s);
}

pub fn pre<T: std::fmt::Debug>(s: T) {
    println!("{:?}", s);
}

fn print_matrix(mat: Matrix4<f32>) {
    println!("{:?}", mat.x);
    println!("{:?}", mat.y);
    println!("{:?}\n", mat.z);
}
