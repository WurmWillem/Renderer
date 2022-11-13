use crate::{consts::*, draw_wireframe_triangle};
use cgmath::*;

pub struct Instance {
    model: Model,
    verts: Vertices,
    triangles: Vec<Indices>,
    pub trans: Transform,
}
impl Instance {
    pub fn new(model: Model, translation: Vec3, scale: f64) -> Self {
        let trans = Transform::new(translation, scale);
        Self {
            model,
            verts: model.get_verts(),
            triangles: model.get_indices(),
            trans,
        }
    }
    pub fn Render(&self, frame: &mut [u8]) {
        let mut projected = Vec::new();
        for vert in &self.verts {
            let mut vert = *vert;
            self.trans.apply_transform(&mut vert);
            projected.push(project_vertex(vert));
        }

        for tri in &self.triangles {
            render_triangle(*tri, &projected, frame, GREEN);
        }
    }
}

pub struct Transform {
    pub translation: Vec3,
    pub scale: f64,
    pub rot: f64,
}
impl Transform {
    fn new(translation: Vec3, scale: f64) -> Self {
        Self {
            translation,
            scale,
            rot: 0.
        }
    }
    fn apply_transform(&self, vert: &mut Vec3) {
        let default_transl = Vec3::new(-2., 0., 7.);
        let rot: Basis3<f64> = Rotation3::from_angle_y(Deg(self.rot));

        *vert = rot.rotate_vector(*vert) * self.scale;
        *vert += self.translation + default_transl;
    }
}

fn render_triangle(tri: Indices, projected: &Vec<Vec2>, frame: &mut [u8], color: [u8; 3]) {
    draw_wireframe_triangle(
        projected[tri.0],
        projected[tri.1],
        projected[tri.2],
        frame,
        color,
    );
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

fn pr<T: std::fmt::Display>(s: T) {
    println!("{}", s);
}

fn print_matrix(mat: Matrix4<f32>) {
    println!("{:?}", mat.x);
    println!("{:?}", mat.y);
    println!("{:?}\n", mat.z);
}
