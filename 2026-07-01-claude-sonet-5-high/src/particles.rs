use macroquad::prelude::*;
use macroquad::rand::gen_range;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    max_life: f32,
    size: f32,
    color: Color,
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn spawn_burst(&mut self, pos: Vec2, color: Color, count: usize) {
        for _ in 0..count {
            let angle = gen_range(0.0, std::f32::consts::TAU);
            let speed = gen_range(40.0, 140.0);
            self.particles.push(Particle {
                pos,
                vel: vec2(angle.cos() * speed, angle.sin() * speed - 40.0),
                life: 0.0,
                max_life: gen_range(0.35, 0.7),
                size: gen_range(2.0, 4.0),
                color,
            });
        }
    }

    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.life += dt;
            p.vel.y += 400.0 * dt;
            p.pos += p.vel * dt;
        }
        self.particles.retain(|p| p.life < p.max_life);
    }

    pub fn draw_offset(&self, cam_x: f32, offset_y: f32) {
        for p in &self.particles {
            let t = 1.0 - (p.life / p.max_life);
            let mut c = p.color;
            c.a = t;
            draw_rectangle(p.pos.x - cam_x, p.pos.y + offset_y, p.size, p.size, c);
        }
    }
}
