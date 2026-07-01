use macroquad::prelude::*;

fn hash(n: f32) -> f32 {
    let x = (n * 12.9898).sin() * 43758.5453;
    x.fract().abs()
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        1.0,
    )
}

fn draw_floor_layer(cam_x: f32, screen_w: f32, screen_h: f32, parallax: f32, seg_w: f32, max_frac: f32, min_frac: f32, color: Color) {
    let world_offset = cam_x * parallax;
    let start_col = (world_offset / seg_w).floor() as i32 - 1;
    let end_col = ((world_offset + screen_w) / seg_w).ceil() as i32 + 1;
    for col in start_col..=end_col {
        let h = (min_frac + hash(col as f32) * (max_frac - min_frac)) * screen_h;
        let x = col as f32 * seg_w - world_offset;
        draw_rectangle(x, screen_h - h, seg_w + 1.0, h, color);
    }
}

fn draw_ceiling_layer(cam_x: f32, screen_w: f32, screen_h: f32, parallax: f32, seg_w: f32, max_frac: f32, color: Color) {
    let world_offset = cam_x * parallax;
    let start_col = (world_offset / seg_w).floor() as i32 - 1;
    let end_col = ((world_offset + screen_w) / seg_w).ceil() as i32 + 1;
    for col in start_col..=end_col {
        let h = hash(col as f32 + 500.0) * max_frac * screen_h;
        let x = col as f32 * seg_w - world_offset;
        draw_rectangle(x, 0.0, seg_w + 1.0, h, color);
    }
}

pub fn draw_background(cam_x: f32, screen_w: f32, screen_h: f32) {
    let bands = 14;
    let top = Color::from_rgba(28, 16, 42, 255);
    let bottom = Color::from_rgba(8, 6, 16, 255);
    for i in 0..bands {
        let t0 = i as f32 / bands as f32;
        let t1 = (i + 1) as f32 / bands as f32;
        let c = lerp_color(top, bottom, t0);
        draw_rectangle(0.0, t0 * screen_h, screen_w, (t1 - t0) * screen_h + 1.0, c);
    }

    draw_ceiling_layer(cam_x, screen_w, screen_h, 0.25, 56.0, 0.14, Color::from_rgba(36, 22, 52, 255));
    draw_floor_layer(cam_x, screen_w, screen_h, 0.2, 48.0, 0.45, 0.15, Color::from_rgba(42, 26, 60, 255));
    draw_ceiling_layer(cam_x, screen_w, screen_h, 0.5, 44.0, 0.10, Color::from_rgba(50, 32, 68, 255));
    draw_floor_layer(cam_x, screen_w, screen_h, 0.45, 38.0, 0.32, 0.1, Color::from_rgba(58, 36, 78, 255));
    draw_floor_layer(cam_x, screen_w, screen_h, 0.7, 30.0, 0.22, 0.08, Color::from_rgba(72, 46, 92, 255));
}
