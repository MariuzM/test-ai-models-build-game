use macroquad::prelude::*;

use crate::level::{Level, TILE};
use crate::sprites::PlayerSprites;

const GRAVITY: f32 = 2200.0;
const MAX_FALL: f32 = 1000.0;
const MOVE_SPEED: f32 = 210.0;
const JUMP_VELOCITY: f32 = -650.0;
const COYOTE_TIME: f32 = 0.1;
const JUMP_BUFFER: f32 = 0.12;
const INVULN_TIME: f32 = 1.2;
pub const MAX_HEALTH: i32 = 3;

pub const PLAYER_W: f32 = 18.0;
pub const PLAYER_H: f32 = 30.0;
const SCALE: f32 = 1.6;

#[derive(PartialEq)]
enum AnimState {
    Idle,
    Run,
    Jump,
    Fall,
}

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub facing_right: bool,
    pub on_ground: bool,
    pub health: i32,
    pub gems: i32,
    pub invuln_timer: f32,
    pub dead: bool,
    pub won: bool,
    anim_state: AnimState,
    anim_timer: f32,
    anim_frame: usize,
    coyote_timer: f32,
    jump_buffer_timer: f32,
}

impl Player {
    pub fn new(start: Vec2) -> Self {
        Self {
            pos: start,
            vel: vec2(0.0, 0.0),
            facing_right: true,
            on_ground: false,
            health: MAX_HEALTH,
            gems: 0,
            invuln_timer: 0.0,
            dead: false,
            won: false,
            anim_state: AnimState::Idle,
            anim_timer: 0.0,
            anim_frame: 0,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, PLAYER_W, PLAYER_H)
    }

    pub fn update(&mut self, dt: f32, level: &Level) {
        if self.dead || self.won {
            return;
        }

        // Horizontal input
        let mut move_dir = 0.0;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            move_dir -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            move_dir += 1.0;
        }
        if move_dir != 0.0 {
            self.facing_right = move_dir > 0.0;
        }
        self.vel.x = move_dir * MOVE_SPEED;

        // Jump buffering + coyote time
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.jump_buffer_timer = JUMP_BUFFER;
        }
        if self.on_ground {
            self.coyote_timer = COYOTE_TIME;
        }
        if self.jump_buffer_timer > 0.0 && self.coyote_timer > 0.0 {
            self.vel.y = JUMP_VELOCITY;
            self.jump_buffer_timer = 0.0;
            self.coyote_timer = 0.0;
            self.on_ground = false;
        }
        self.jump_buffer_timer = (self.jump_buffer_timer - dt).max(0.0);
        self.coyote_timer = (self.coyote_timer - dt).max(0.0);

        // Variable jump height: cut upward velocity if space released early
        if !is_key_down(KeyCode::Space)
            && !is_key_down(KeyCode::Up)
            && !is_key_down(KeyCode::W)
            && self.vel.y < 0.0
        {
            self.vel.y += GRAVITY * dt * 2.0;
        }

        // Gravity
        self.vel.y += GRAVITY * dt;
        self.vel.y = self.vel.y.min(MAX_FALL);

        // Move + collide, axis separated
        self.pos.x += self.vel.x * dt;
        resolve_x(&mut self.pos, &mut self.vel, level);
        self.pos.y += self.vel.y * dt;
        self.on_ground = false;
        resolve_y(&mut self.pos, &mut self.vel, level, &mut self.on_ground);

        // Fell into a pit
        if self.pos.y > level.height_px() + 200.0 {
            self.take_damage(vec2(0.0, -1.0));
            self.respawn_after_fall(level);
        }

        self.update_animation(dt);

        if self.invuln_timer > 0.0 {
            self.invuln_timer -= dt;
        }
    }

    fn respawn_after_fall(&mut self, level: &Level) {
        self.pos = level.player_start;
        self.vel = vec2(0.0, 0.0);
    }

    fn update_animation(&mut self, dt: f32) {
        let new_state = if !self.on_ground && self.vel.y < 0.0 {
            AnimState::Jump
        } else if !self.on_ground {
            AnimState::Fall
        } else if self.vel.x.abs() > 1.0 {
            AnimState::Run
        } else {
            AnimState::Idle
        };
        if new_state != self.anim_state {
            self.anim_state = new_state;
            self.anim_frame = 0;
            self.anim_timer = 0.0;
        }
        self.anim_timer += dt;
        let frame_time = 0.11;
        if self.anim_timer >= frame_time {
            self.anim_timer -= frame_time;
            self.anim_frame = (self.anim_frame + 1) % 4;
        }
    }

    pub fn take_damage(&mut self, _from: Vec2) -> bool {
        if self.invuln_timer > 0.0 || self.dead {
            return false;
        }
        self.health -= 1;
        self.invuln_timer = INVULN_TIME;
        self.vel.y = -350.0;
        if self.health <= 0 {
            self.health = 0;
            self.dead = true;
        }
        true
    }

    /// Draws the player with its collision box's top-left placed at `screen_pos`
    /// (i.e. already translated by the camera), rather than at `self.pos`.
    pub fn draw_at(&self, sprites: &PlayerSprites, screen_pos: Vec2) {
        if self.invuln_timer > 0.0 && (self.invuln_timer * 12.0) as i32 % 2 == 0 {
            return;
        }
        let tex = match self.anim_state {
            AnimState::Idle => &sprites.idle,
            AnimState::Run => &sprites.run[self.anim_frame],
            AnimState::Jump => &sprites.jump,
            AnimState::Fall => &sprites.fall,
        };
        let draw_w = tex.width() * SCALE;
        let draw_h = tex.height() * SCALE;
        let draw_x = screen_pos.x + PLAYER_W / 2.0 - draw_w / 2.0;
        let draw_y = screen_pos.y + PLAYER_H - draw_h;
        draw_texture_ex(
            tex,
            draw_x,
            draw_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                flip_x: !self.facing_right,
                ..Default::default()
            },
        );
    }
}

fn tile_range(min: f32, max: f32) -> (i32, i32) {
    ((min / TILE).floor() as i32, (max / TILE).floor() as i32)
}

fn resolve_x(pos: &mut Vec2, vel: &mut Vec2, level: &Level) {
    let (min_row, max_row) = tile_range(pos.y, pos.y + PLAYER_H - 1.0);
    let (min_col, max_col) = tile_range(pos.x, pos.x + PLAYER_W - 1.0);
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if level.is_solid(col, row) {
                let tile_x = col as f32 * TILE;
                if vel.x > 0.0 {
                    pos.x = tile_x - PLAYER_W;
                } else if vel.x < 0.0 {
                    pos.x = tile_x + TILE;
                }
                vel.x = 0.0;
            }
        }
    }
}

fn resolve_y(pos: &mut Vec2, vel: &mut Vec2, level: &Level, on_ground: &mut bool) {
    let (min_row, max_row) = tile_range(pos.y, pos.y + PLAYER_H - 1.0);
    let (min_col, max_col) = tile_range(pos.x, pos.x + PLAYER_W - 1.0);
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if level.is_solid(col, row) {
                let tile_y = row as f32 * TILE;
                if vel.y > 0.0 {
                    pos.y = tile_y - PLAYER_H;
                    *on_ground = true;
                } else if vel.y < 0.0 {
                    pos.y = tile_y + TILE;
                }
                vel.y = 0.0;
            }
        }
    }
}
