use macroquad::prelude::*;

use crate::sprites::{generate_gem_icon, generate_heart};

pub struct Hud {
    heart_full: Texture2D,
    heart_empty: Texture2D,
    gem_icon: Texture2D,
}

impl Hud {
    pub fn new() -> Self {
        Self {
            heart_full: generate_heart(true),
            heart_empty: generate_heart(false),
            gem_icon: generate_gem_icon(),
        }
    }

    pub fn draw(&self, health: i32, max_health: i32, gems: i32, total_gems: i32, elapsed: f32) {
        let scale = 3.0;
        for i in 0..max_health {
            let tex = if i < health {
                &self.heart_full
            } else {
                &self.heart_empty
            };
            draw_texture_ex(
                tex,
                16.0 + i as f32 * (tex.width() * scale + 6.0),
                16.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tex.width() * scale, tex.height() * scale)),
                    ..Default::default()
                },
            );
        }

        let gx = 16.0;
        let gy = 16.0 + self.heart_full.height() * scale + 10.0;
        draw_texture_ex(
            &self.gem_icon,
            gx,
            gy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.gem_icon.width() * scale, self.gem_icon.height() * scale)),
                ..Default::default()
            },
        );
        draw_text(
            &format!("x {gems}/{total_gems}"),
            gx + self.gem_icon.width() * scale + 8.0,
            gy + self.gem_icon.height() * scale * 0.8,
            26.0,
            WHITE,
        );

        let mins = (elapsed / 60.0) as i32;
        let secs = elapsed as i32 % 60;
        let time_text = format!("{mins:01}:{secs:02}");
        let dim = measure_text(&time_text, None, 26, 1.0);
        draw_text(&time_text, screen_width() - dim.width - 20.0, 34.0, 26.0, WHITE);
    }
}

pub fn draw_centered_text(text: &str, y: f32, size: u16, color: Color) {
    let dim = measure_text(text, None, size, 1.0);
    draw_text(text, screen_width() / 2.0 - dim.width / 2.0, y, size as f32, color);
}
