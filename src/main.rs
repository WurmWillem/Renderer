#![allow(non_snake_case)]
#![allow(dead_code)]
use image::{Rgb, RgbImage};

mod consts;
mod instancing;
use consts::*;
use instancing::*;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

//const WIDTH: u32 = 640;
//const HEIGHT: u32 = 640;
const BOX_SIZE: i16 = 64;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(CANVAS_SIZE as f64, CANVAS_SIZE as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(CANVAS_SIZE, CANVAS_SIZE, surface_texture)?
    };

    let instances = vec![
        Instance::new(Model::Cube, Vec3::new(0., 0., 0.)),
        Instance::new(Model::Cube, Vec3::new(2.5, 0., -1.)),
    ];

    event_loop.run(move |event, _, control_flow| {
        
        if let Event::RedrawRequested(_) = event {
            //show_individual_pixels(pixels.get_frame_mut());
            for instance in &instances {
                instance.Render(pixels.get_frame_mut());
            }

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

fn show_individual_pixels(frame: &mut [u8]) {
    let mut pog = true;
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        if i % 64 == 0 {
            pog = !pog;
        }
        let rgba = if pog {
            [0x48, 0xb2, 0xe8, 0xff]
        } else {
            [0x5e, 0x48, 0xe8, 0xff]
        };
        pog = !pog;

        pixel.copy_from_slice(&rgba);
    }
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

fn draw_line(mut p0: Vec2, mut p1: Vec2, frame: &mut [u8], color: Rgb<u8>) {
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

            let x_y_to_i = x_y_to_i(x_to_draw, y_to_draw) * 4;
            frame[x_y_to_i] = color.0[0];
            frame[x_y_to_i + 1] = color.0[1];
            frame[x_y_to_i + 2] = color.0[2];
            frame[x_y_to_i + 3] = 0xff;
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

            let x_y_to_i = x_y_to_i(x_to_draw, y_to_draw) * 4;
            frame[x_y_to_i] = color.0[0];
            frame[x_y_to_i + 1] = color.0[1];
            frame[x_y_to_i + 2] = color.0[2];
            frame[x_y_to_i + 3] = 0xff;
        }
    }
}

fn x_y_to_i(x: u32, y: u32) -> usize {
    (y * CANVAS_SIZE + x) as usize
}

fn draw_wireframe_triangle(p0: Vec2, p1: Vec2, p2: Vec2, frame: &mut [u8], color: Rgb<u8>) {
    draw_line(p0, p1, frame, color);
    draw_line(p1, p2, frame, color);
    draw_line(p2, p0, frame, color);
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
