use crate::{consts::*, draw_wireframe_triangle, project_vertex};
use cgmath::Matrix4;
use image::{Rgb, RgbImage};

pub struct Instance {
    model: Model,
    verts: Vertices,
    triangles: Vec<Indices>,
    pos: Vec3,
}
impl Instance {
    pub fn new(model: Model, pos: Vec3) -> Self {
        Self {
            model,
            verts: model.get_verts(),
            triangles: model.get_indices(),
            pos,
        }
    }
    pub fn Render(&self, canvas: &mut RgbImage) {
        let translation = Vec3::new(-1.5, 0., 7.);

        let value = 1.;
        let m = Matrix4::new(
            value, value, value, value, value, value, value, value, value, value, value, value, value,
            value, value, value,
        );
        //print_matrix(Matrix4::lo);

        let mut projected = Vec::new();
        for vert in &self.verts {
            let vert_pos = vert + translation + self.pos;
            projected.push(project_vertex(vert_pos));
        }

        for tri in &self.triangles {
            render_triangle(*tri, &projected, canvas, GREEN);
        }
    }
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

pub fn render_object(verts: &Vec<Vec3>, tri_indices: &Vec<Indices>, canvas: &mut RgbImage) {
    let translation = Vec3::new(-1.5, 0., 7.);
    
    let mut projected = Vec::new();
    for vert in verts {
        projected.push(project_vertex(*vert + translation))
    }

    for tri in tri_indices {
        render_triangle(*tri, &projected, canvas, GREEN);
    }
}

fn render_triangle(tri: Indices, projected: &Vec<Vec2>, canvas: &mut RgbImage, color: Rgb<u8>) {
    draw_wireframe_triangle(
        projected[tri.0],
        projected[tri.1],
        projected[tri.2],
        canvas,
        color,
    );
}

fn print_matrix(mat: Matrix4<f32>) {
    println!("{:?}", mat.x);
    println!("{:?}", mat.y);
    println!("{:?}\n", mat.z);
}
