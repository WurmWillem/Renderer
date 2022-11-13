#![allow(non_snake_case)]
#![allow(dead_code)]
use image::{ImageBuffer, Rgb, RgbImage};

mod consts;
mod instancing;
use consts::*;
use instancing::*;

fn main() {
    let mut canvas: RgbImage = ImageBuffer::new(CANVAS_SIZE, CANVAS_SIZE);
    canvas.fill(220);

    let instances = vec![
        Instance::new(Model::Cube, Vec3::new(0., 0., 0.)),
        //Instance::new(Model::Cube, Vec3::new(2.5, 0., -1.)),
    ];

    for instance in &instances {
        instance.Render(&mut canvas);
    }

    //let point_mult = 1.;
    //let p0 = Vec2::new(-50., 240.) * point_mult;
    //let p1 = Vec2::new(120., 50.) * point_mult;
    //let p2 = Vec2::new(-250., -200.) * point_mult;
    //let h = (0.3, 0., 1.);

    //draw_triangle(p0, p1, p2, &mut canvas, GREEN);
    //draw_shaded_triangle(p0, p1, p2, h, &mut canvas, GREEN);
    //draw_wireframe_triangle(p0, p1, p2, &mut canvas, WHITE);

    canvas.save("result.png").unwrap();
}

fn draw_shaded_triangle(
    mut p0: Vec2,
    mut p1: Vec2,
    mut p2: Vec2,
    h: Shade,
    canvas: &mut RgbImage,
    color: Rgb<u8>,
) {
    if p0.y > p1.y {
        swap(&mut p0, &mut p1)
    }
    if p1.y > p2.y {
        swap(&mut p1, &mut p2)
    }
    if p0.y > p1.y {
        swap(&mut p0, &mut p1)
    }
    let (x0, y0, x1, y1, x2, y2) = (p0.x, p0.y, p1.x, p1.y, p2.x, p2.y);

    let mut x01 = interpolate(y0, x0, y1, x1);
    let mut h01 = interpolate(y0, h.0, y1, h.1);
    let mut x12 = interpolate(y1, x1, y2, x2);
    let mut h12 = interpolate(y1, h.1, y2, h.2);
    let x02 = interpolate(y0, x0, y2, x2);
    let h02 = interpolate(y0, h.0, y2, h.2);

    x01.remove(x01.len() - 1);
    x01.append(&mut x12);
    let x012 = x01;

    h01.remove(h01.len() - 1);
    h01.append(&mut h12);
    let h012 = h01;

    let (mut x_left, mut x_right) = (&x012, &x02);
    let (mut h_left, mut h_right) = (&h012, &h02);

    let m = x02.len() / 2;
    if x012[m] > x02[m] {
        (x_left, x_right) = (&x02, &x012);
        (h_left, h_right) = (&h02, &h012);
    }

    for y in y0 as i32..y2 as i32 {
        let y_to_draw = (-y + CANVAS_SIZE as i32 / 2) as u32;
        let y_index = (y as f32 - y0) as usize;
        let h_segment = interpolate(
            x_left[y_index],
            h_left[y_index],
            x_right[y_index],
            h_right[y_index],
        );
        //println!("{}", h_segment.len());
        for x in x_left[y_index] as i32..x_right[y_index] as i32 {
            let x_to_draw = (x + CANVAS_SIZE as i32 / 2) as u32;

            let h = h_segment[(x as f32 - x_left[y_index]) as usize];
            let shaded_r = (color[0] as f32 * h) as u8;
            let shaded_g = (color[1] as f32 * h) as u8;
            let shaded_b = (color[2] as f32 * h) as u8;
            //println!("{}", h);

            canvas.put_pixel(x_to_draw, y_to_draw, Rgb([shaded_r, shaded_g, shaded_b]));
        }
    }
}

fn draw_triangle(mut p0: Vec2, mut p1: Vec2, mut p2: Vec2, canvas: &mut RgbImage, color: Rgb<u8>) {
    if p0.y > p1.y {
        swap(&mut p0, &mut p1)
    }
    if p1.y > p2.y {
        swap(&mut p1, &mut p2)
    }
    if p0.y > p1.y {
        swap(&mut p0, &mut p1)
    }
    let (x0, y0, x1, y1, x2, y2) = (p0.x, p0.y, p1.x, p1.y, p2.x, p2.y);

    let mut x01 = interpolate(y0, x0, y1, x1);
    let mut x12 = interpolate(y1, x1, y2, x2);
    let x02 = interpolate(y0, x0, y2, x2);

    x01.remove(x01.len() - 1);
    x01.append(&mut x12);
    let x012 = x01;

    let m = x02.len() / 2;
    let (mut x_left, mut x_right) = (&x012, &x02);
    if x012[m] > x02[m] {
        (x_left, x_right) = (&x02, &x012);
    }

    for y in y0 as i32..y2 as i32 {
        let y_to_draw = (-y + CANVAS_SIZE as i32 / 2) as u32;
        let y_index = (y as f32 - y0) as usize;

        for x in x_left[y_index] as i32..x_right[y_index] as i32 {
            let x_to_draw = (x + CANVAS_SIZE as i32 / 2) as u32;

            canvas.put_pixel(x_to_draw, y_to_draw, color);
        }
    }
}

fn draw_line(mut p0: Vec2, mut p1: Vec2, canvas: &mut RgbImage, color: Rgb<u8>) {
    if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
        if p0.x > p1.x {
            swap(&mut p0, &mut p1);
        };
        check_if_out_of_canvas(p0, p1);

        let (x0, x1, y0, y1) = (p0.x, p1.x, p0.y, p1.y);

        let ys = interpolate(x0, y0, x1, y1);

        for x in (x0 as i32)..(x1 as i32 + 1) {
            let x_to_draw = (x + CANVAS_SIZE as i32 / 2) as u32;
            let y_to_draw = (-ys[(x - x0 as i32) as usize] + CANVAS_SIZE as f32 / 2.) as u32;

            canvas.put_pixel(x_to_draw, y_to_draw, color);
        }
    } else {
        if p0.y > p1.y {
            swap(&mut p0, &mut p1);
        };
        check_if_out_of_canvas(p0, p1);

        let (x0, x1, y0, y1) = (p0.x, p1.x, p0.y, p1.y);

        let xs = interpolate(y0, x0, y1, x1);

        for y in (y0 as i32)..(y1 as i32 + 1) {
            let x_to_draw = (xs[(y - y0 as i32) as usize] + CANVAS_SIZE as f32 / 2.) as u32;
            let y_to_draw = (-y + CANVAS_SIZE as i32 / 2) as u32;

            canvas.put_pixel(x_to_draw, y_to_draw, color);
        }
    }
}

fn draw_wireframe_triangle(p0: Vec2, p1: Vec2, p2: Vec2, canvas: &mut RgbImage, color: Rgb<u8>) {
    draw_line(p0, p1, canvas, color);
    draw_line(p1, p2, canvas, color);
    draw_line(p2, p0, canvas, color);
}

fn interpolate(i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<f32> {
    if i0 as i32 == i1 as i32 {
        return vec![d0];
    }
    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;

    let mut values = Vec::new();
    for _ in i0 as i32..i1 as i32 + 1 {
        values.push(d);
        d += a;
    }
    values
}

fn swap<T: std::marker::Copy>(x0: &mut T, x1: &mut T) {
    let temp = *x0;
    *x0 = *x1;
    *x1 = temp;
}

fn check_if_out_of_canvas(p0: Vec2, p1: Vec2) {
    if p0.x < CANVAS_SIZE as f32 / -2.
        || p0.y < CANVAS_SIZE as f32 / -2.
        || p1.x > CANVAS_SIZE as f32 / 2.
        || p1.y > CANVAS_SIZE as f32 / 2.
    {
        panic!("points specified not within canvas size");
    }
}

fn project_vertex(vert: Vec3) -> Vec2 {
    viewport_to_canvas(vert.x * D / vert.z, vert.y * D / vert.z)
}

fn viewport_to_canvas(x: f32, y: f32) -> Vec2 {
    let (canvas_size, viewport_size) = (CANVAS_SIZE as f32, VIEWPORT_SIZE as f32);
    Vec2::new(
        x * canvas_size / viewport_size,
        y * canvas_size / viewport_size,
    )
}
