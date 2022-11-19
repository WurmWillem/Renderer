
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

pub fn render_object(verts: &Vec<Vec3>, tri_indices: &Vec<Indices>, frame: &mut [u8]) {
    let translation = Vec3::new(-1.5, 0., 7.);
    
    let mut projected = Vec::new();
    for vert in verts {
        projected.push(project_vertex(*vert + translation))
    }

    for tri in tri_indices {
        render_triangle(*tri, &projected, frame, GREEN);
    }
}