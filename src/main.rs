#![allow(non_snake_case)]
#![allow(dead_code)]

mod consts;
mod instance;
use consts::*;
use instance::{Transform, *};

use log::error;
use pixels::{Error, PixelsBuilder, SurfaceTexture};
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
        let size = LogicalSize::new(WINDOW_SIZE as f64, WINDOW_SIZE as f64);
        WindowBuilder::new()
            .with_title("Renderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let surface_texture = SurfaceTexture::new(WINDOW_SIZE, WINDOW_SIZE, &window);
    let mut pixels = PixelsBuilder::new(CANVAS_SIZE, CANVAS_SIZE, surface_texture)
        .enable_vsync(true)
        .build()?;

    //pixels.set_clear_color(Color::BLACK);

    let mut instances = vec![
        Instance::new(Model::Cube, Vec3::new(0., 0., 0.), 1.),
        Instance::new(Model::Cube, Vec3::new(2.5, 0., 0.), 1.),
    ];

    let mut cam_trans = Transform::new(Vec3::new(0., 0., 0.), 1.);
    let mut cam_is_current_trans = true;
    
    let mut last_frame = std::time::Instant::now();
    let mut frames_passed = 0;
    let mut total_frame_time = 0.;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            total_frame_time += last_frame.elapsed().as_secs_f64();
            last_frame = std::time::Instant::now();
            frames_passed += 1;
            //show_individual_pixels(pixels.get_frame_mut());
            let frame = pixels.get_frame_mut();
            clear_screen(frame);

            for instance in &instances {
                instance.Render(frame, cam_trans);
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

            if input.mouse_pressed(0) {
                pr(frames_passed as f64 / total_frame_time);
                cam_is_current_trans = !cam_is_current_trans;
            }

            let trans_speed = TRANS_SPEED * last_frame.elapsed().as_secs_f64();

            if input.key_held(VirtualKeyCode::W) {
                let translation = Vec3::new(0., 0., trans_speed);
                if cam_is_current_trans {
                    cam_trans.translation += translation;
                } else {
                    instances[0].trans.translation += translation;
                }
            }
            if input.key_held(VirtualKeyCode::A) {
                let translation = Vec3::new(-trans_speed, 0., 0.);
                if cam_is_current_trans {
                    cam_trans.translation += translation;
                } else {
                    instances[0].trans.translation += translation;
                }
            }
            if input.key_held(VirtualKeyCode::S) {
                let translation = Vec3::new(0., 0., -trans_speed);
                if cam_is_current_trans {
                    cam_trans.translation += translation;
                } else {
                    instances[0].trans.translation += translation;
                }
            }
            if input.key_held(VirtualKeyCode::D) {
                let translation = Vec3::new(trans_speed, 0., 0.);
                if cam_is_current_trans {
                    cam_trans.translation += translation;
                } else {
                    instances[0].trans.translation += translation;
                }
            }

            if input.key_held(VirtualKeyCode::Q) {
                if cam_is_current_trans {
                    cam_trans.scale += trans_speed;
                } else {
                    instances[0].trans.scale += trans_speed;
                }
            }
            if input.key_held(VirtualKeyCode::R) {
                if cam_is_current_trans {
                    cam_trans.rot += trans_speed * 30.;
                } else {
                    instances[0].trans.rot += trans_speed * 30.;
                }
            }
            window.request_redraw();
        }
    });
}

fn draw_triangle(mut p0: Vec2, mut p1: Vec2, mut p2: Vec2, frame: &mut [u8], color: [u8; 3]) {
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
        let y_to_draw = -y + CANVAS_SIZE as i32 / 2;
        let y_index = (y as f64 - y0) as usize;

        for x in x_left[y_index] as i32..x_right[y_index] as i32 {
            let x_to_draw = x + CANVAS_SIZE as i32 / 2;

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
    x < 0 || y < 0 || x >= CANVAS_SIZE as i32 || y >= CANVAS_SIZE as i32
}

fn clear_screen(frame: &mut [u8]) {
    for pixel in frame.chunks_exact_mut(64) {
        pixel[0] = 0x00; // R
        pixel[1] = 0x00; // G
        pixel[2] = 0x00; // B

        pixel[4] = 0x00; // R
        pixel[5] = 0x00; // G
        pixel[6] = 0x00; // B

        pixel[8] = 0x00; // R
        pixel[9] = 0x00; // G
        pixel[10] = 0x00; // B

        pixel[12] = 0x00; // R
        pixel[13] = 0x00; // G
        pixel[14] = 0x00; // B

        pixel[16] = 0x00; // R
        pixel[17] = 0x00; // G
        pixel[18] = 0x00; // B

        pixel[20] = 0x00; // R
        pixel[21] = 0x00; // G
        pixel[22] = 0x00; // B

        pixel[24] = 0x00; // R
        pixel[25] = 0x00; // G
        pixel[26] = 0x00; // B

        pixel[28] = 0x00; // R
        pixel[29] = 0x00; // G
        pixel[30] = 0x00; // B

        pixel[32] = 0x00; // R
        pixel[33] = 0x00; // G
        pixel[34] = 0x00; // B

        pixel[36] = 0x00; // R
        pixel[37] = 0x00; // G
        pixel[38] = 0x00; // B

        pixel[39] = 0x00; // R
        pixel[40] = 0x00; // G
        pixel[41] = 0x00; // B

        pixel[43] = 0x00; // R
        pixel[44] = 0x00; // G
        pixel[45] = 0x00; // B

        pixel[47] = 0x00; // R
        pixel[48] = 0x00; // G
        pixel[49] = 0x00; // B

        pixel[51] = 0x00; // R
        pixel[52] = 0x00; // G
        pixel[53] = 0x00; // B

        pixel[55] = 0x00; // R
        pixel[56] = 0x00; // G
        pixel[57] = 0x00; // B

        pixel[58] = 0x00; // R
        pixel[59] = 0x00; // G
        pixel[60] = 0x00; // B

        pixel[61] = 0x00; // G
        pixel[62] = 0x00; // B
        pixel[63] = 0x00; // B
    }
}
