use crate::{
    consts::*,
    instance::{pr, pre, Instance},
};
use cgmath::*;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: Vec3,
    d: f64,
}
impl Plane {
    pub fn new(normal: Vec3, d: f64) -> Self {
        Self { normal, d }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingSphere {
    center: Vec3,
    orig_center: Vec3,
    radius: f64,
}
impl BoundingSphere {
    pub fn new(verts: &Vertices) -> Self {
        let (center, radius) = BoundingSphere::compute(verts);
        Self {
            center,
            radius,
            orig_center: center,
        }
    }

    fn compute(verts: &Vertices) -> (Vec3, f64) {
        let mut total = 0.;
        let mut total_vert_pos = Vec3::new(0., 0., 0.);
        for v in verts {
            total += 1.;
            total_vert_pos += *v;
        }
        let center = total_vert_pos / total;
        let mut furthest_away = center.distance(verts[0]);
        for v in verts {
            if center.distance(*v) > furthest_away {
                furthest_away = center.distance(*v);
            }
        }
        (center, furthest_away)
    }

    pub fn update(&mut self, trans: Vec3, cam_rot: f64) {
        let rot_cam: Basis3<f64> = Rotation3::from_angle_y(Deg(cam_rot));
        self.center = trans + self.orig_center + DEFAULT_TRANSL;
        self.center = rot_cam.rotate_vector(self.center);
        //pre(self.center);
    }
}

pub fn clip_scene(instances: &Vec<Instance>, planes: &Vec<Plane>) -> Vec<Instance> {
    let mut clipped_instances = Vec::new();
    for inst in instances {
        let clipped_instance = clip_instance(inst, planes);
        if let Some(clipped) = clipped_instance {
            clipped_instances.push(clipped);
        }
    }
    clipped_instances
}

fn clip_instance(inst: &Instance, planes: &Vec<Plane>) -> Option<Instance> {
    for p in planes {
        let instance = clip_instance_against_plane(inst, p);
        if let Some(_) = instance {
            continue;
        }
        return None;
    }
    Some(inst.clone())
}

fn clip_instance_against_plane(inst: &Instance, plane: &Plane) -> Option<Instance> {
    let d = signed_dist(plane, inst.bounding_sphere.center);
    let r = inst.bounding_sphere.radius;

    if d > r {
        return Some(inst.clone());
    } else if d < -r {
        return None;
    } else {
        return None;
        /*
        let mut clipped_inst = inst.clone();
        clipped_inst.triangles = clip_tri_against_plane(&inst.triangles, plane, &inst.verts);

        return Some(clipped_inst); */
    }
}

fn signed_dist(plane: &Plane, vert: Vec3) -> f64 {
    let normal = plane.normal;
    vert.x * normal.x + vert.y * normal.y + vert.z + normal.z + plane.d
}
