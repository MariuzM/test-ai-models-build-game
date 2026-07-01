use macroquad::prelude::*;

use crate::level::{BatSpawn, SlimeSpawn, TILE};
use crate::sprites::{BatSprites, SlimeSprites};

const SCALE: f32 = 1.6;
pub const BAT_W: f32 = 18.0 * SCALE * 0.6;
pub const BAT_H: f32 = 12.0 * SCALE * 0.6;
pub const SLIME_W: f32 = 18.0 * SCALE * 0.6;
pub const SLIME_H: f32 = 12.0 * SCALE * 0.6;

pub struct Bat {
    pub pos: Vec2,
    center_x: f32,
    center_y: f32,
    amplitude: f32,
    time: f32,
    anim_timer: f32,
    frame: usize,
    pub alive: bool,
}

impl Bat {
    pub fn from_spawn(spawn: &BatSpawn) -> Self {
        Self {
            pos: vec2(spawn.x, spawn.y),
            center_x: spawn.x,
            center_y: spawn.y,
            amplitude: (spawn.range_max - spawn.range_min).max(TILE) / 2.0,
            time: 0.0,
            anim_timer: 0.0,
            frame: 0,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.alive {
            return;
        }
        self.time += dt;
        self.pos.x = self.center_x + (self.time * 1.4).sin() * self.amplitude;
        self.pos.y = self.center_y + (self.time * 3.0).sin() * 10.0;

        self.anim_timer += dt;
        if self.anim_timer > 0.15 {
            self.anim_timer = 0.0;
            self.frame = (self.frame + 1) % 2;
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            self.pos.x - BAT_W / 2.0,
            self.pos.y - BAT_H / 2.0,
            BAT_W,
            BAT_H,
        )
    }

    pub fn draw_at(&self, sprites: &BatSprites, screen_pos: Vec2) {
        if !self.alive {
            return;
        }
        let tex = &sprites.frames[self.frame];
        let w = tex.width() * SCALE * 0.6;
        let h = tex.height() * SCALE * 0.6;
        draw_texture_ex(
            tex,
            screen_pos.x - w / 2.0,
            screen_pos.y - h / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );
    }
}

pub struct Slime {
    pub pos: Vec2,
    min_x: f32,
    max_x: f32,
    dir: f32,
    speed: f32,
    anim_timer: f32,
    frame: usize,
    pub alive: bool,
}

impl Slime {
    pub fn from_spawn(spawn: &SlimeSpawn) -> Self {
        Self {
            pos: vec2(spawn.x, spawn.y - SLIME_H),
            min_x: spawn.patrol_min,
            max_x: spawn.patrol_max,
            dir: 1.0,
            speed: 45.0,
            anim_timer: 0.0,
            frame: 0,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.alive {
            return;
        }
        self.pos.x += self.dir * self.speed * dt;
        if self.pos.x < self.min_x {
            self.pos.x = self.min_x;
            self.dir = 1.0;
        } else if self.pos.x + SLIME_W > self.max_x {
            self.pos.x = self.max_x - SLIME_W;
            self.dir = -1.0;
        }

        self.anim_timer += dt;
        if self.anim_timer > 0.35 {
            self.anim_timer = 0.0;
            self.frame = (self.frame + 1) % 2;
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, SLIME_W, SLIME_H)
    }

    pub fn draw_at(&self, sprites: &SlimeSprites, screen_pos: Vec2) {
        if !self.alive {
            return;
        }
        let tex = &sprites.frames[self.frame];
        let w = tex.width() * SCALE * 0.6;
        let h = tex.height() * SCALE * 0.6;
        draw_texture_ex(
            tex,
            screen_pos.x + SLIME_W / 2.0 - w / 2.0,
            screen_pos.y + SLIME_H - h,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                flip_x: self.dir < 0.0,
                ..Default::default()
            },
        );
    }
}
