#![allow(non_snake_case)]
#![allow(dead_code)]

mod consts;
mod instance;
use cgmath::{Rad, Matrix};
use consts::*;
use instance::*;

use log::error;
use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(CANVAS_SIZE as f64, CANVAS_SIZE as f64);
        WindowBuilder::new()
            .with_title("Renderer")
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
    pixels.set_clear_color(Color::BLACK);

    let mut instances = vec![
        Instance::new(Model::Cube, Vec3::new(0., 0., 0.), 1.),
        //Instance::new(Model::Cube, Vec3::new(2.5, 0., 0.), 1.),
    ];

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            //show_individual_pixels(pixels.get_frame_mut());
            pixels.get_frame_mut().fill(50);
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

            if input.key_held(VirtualKeyCode::W) {
                instances[0].trans.translation += Vec3::new(0.,0.,0.1);
            }
            if input.key_held(VirtualKeyCode::A) {
                instances[0].trans.translation += Vec3::new(-0.1,0.,0.);
            }
            if input.key_held(VirtualKeyCode::S) {
                instances[0].trans.translation += Vec3::new(0.,0.,-0.1);
            }
            if input.key_held(VirtualKeyCode::D) {
                instances[0].trans.translation += Vec3::new(0.1,0.,0.);
            }

            if input.key_held(VirtualKeyCode::Q) {
                instances[0].trans.scale += 0.1;
            }
            if input.key_held(VirtualKeyCode::R) {
                instances[0].trans.rot += 5.;
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

fn draw_line(mut p0: Vec2, mut p1: Vec2, frame: &mut [u8], color: [u8; 3]) {
    if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
        if p0.x > p1.x {
            swap(&mut p0, &mut p1);
        };
        //check_if_out_of_canvas(p0, p1);

        let (x0, x1, y0, y1) = (p0.x, p1.x, p0.y, p1.y);

        let ys = interpolate(x0, y0, x1, y1);

        for x in (x0 as i32)..(x1 as i32 + 1) {
            let x_to_draw = x + CANVAS_SIZE as i32 / 2;
            let y_to_draw = (-ys[(x - x0 as i32) as usize] + CANVAS_SIZE as f64 / 2.) as i32;
            if check_if_out_of_canvas(x_to_draw, y_to_draw) {
                continue;
            }

            let x_y_to_i = x_y_to_i(x_to_draw as u32, y_to_draw as u32) * 4;
            frame[x_y_to_i] = color[0];
            frame[x_y_to_i + 1] = color[1];
            frame[x_y_to_i + 2] = color[2];
            frame[x_y_to_i + 3] = 0xff;
        }
    } else {
        if p0.y > p1.y {
            swap(&mut p0, &mut p1);
        };
        //check_if_out_of_canvas(p0, p1);

        let (x0, x1, y0, y1) = (p0.x, p1.x, p0.y, p1.y);

        let xs = interpolate(y0, x0, y1, x1);

        for y in (y0 as i32)..(y1 as i32 + 1) {
            let x_to_draw = (xs[(y - y0 as i32) as usize] + CANVAS_SIZE as f64 / 2.) as i32;
            let y_to_draw = -y + CANVAS_SIZE as i32 / 2;
            if check_if_out_of_canvas(x_to_draw, y_to_draw) {
                continue;
            }

            let x_y_to_i = x_y_to_i(x_to_draw as u32, y_to_draw as u32) * 4;
            frame[x_y_to_i] = color[0];
            frame[x_y_to_i + 1] = color[1];
            frame[x_y_to_i + 2] = color[2];
            frame[x_y_to_i + 3] = 0xff;
        }
    }
}

fn x_y_to_i(x: u32, y: u32) -> usize {
    (y * CANVAS_SIZE + x) as usize
}

fn draw_wireframe_triangle(p0: Vec2, p1: Vec2, p2: Vec2, frame: &mut [u8], color: [u8; 3]) {
    draw_line(p0, p1, frame, color);
    draw_line(p1, p2, frame, color);
    draw_line(p2, p0, frame, color);
}

fn interpolate(i0: f64, d0: f64, i1: f64, d1: f64) -> Vec<f64> {
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

fn check_if_out_of_canvas(x: i32, y: i32) -> bool {
    x < 0
        || y < 0
        || x >= CANVAS_SIZE as i32
        || y >= CANVAS_SIZE as i32
    
}
