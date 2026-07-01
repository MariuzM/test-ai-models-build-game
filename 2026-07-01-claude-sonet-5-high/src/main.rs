mod background;
mod enemy;
mod hud;
mod level;
mod particles;
mod player;
mod sprites;

use macroquad::prelude::*;

use enemy::{Bat, Slime};
use hud::{draw_centered_text, Hud};
use level::{build_level, Level, TileKind, COLS, ROWS, TILE};
use particles::ParticleSystem;
use player::Player;
use sprites::{BatSprites, PlayerSprites, SlimeSprites, TorchSprites};

const OFFSET_Y: f32 = 60.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ember Knight".to_owned(),
        window_width: 1024,
        window_height: 576,
        ..Default::default()
    }
}

#[derive(PartialEq)]
enum GameState {
    Title,
    Playing,
    Dead,
    Won,
}

struct Assets {
    player: PlayerSprites,
    bat: BatSprites,
    slime: SlimeSprites,
    ground_tiles: [Texture2D; 2],
    spike: Texture2D,
    torch: TorchSprites,
    gem_frames: [Texture2D; 2],
    hud: Hud,
}

impl Assets {
    fn load() -> Self {
        Self {
            player: PlayerSprites::generate(),
            bat: BatSprites::generate(),
            slime: SlimeSprites::generate(),
            ground_tiles: [sprites::generate_ground_tile(0), sprites::generate_ground_tile(1)],
            spike: sprites::generate_spike(),
            torch: TorchSprites::generate(),
            gem_frames: [sprites::generate_gem(0), sprites::generate_gem(1)],
            hud: Hud::new(),
        }
    }
}

struct World {
    level: Level,
    player: Player,
    bats: Vec<Bat>,
    slimes: Vec<Slime>,
    gems_collected: Vec<bool>,
    total_gems: i32,
    particles: ParticleSystem,
    elapsed: f32,
    time: f32,
}

impl World {
    fn new() -> Self {
        let level = build_level();
        let player = Player::new(level.player_start);
        let bats = level.bats.iter().map(Bat::from_spawn).collect();
        let slimes = level.slimes.iter().map(Slime::from_spawn).collect();
        let total_gems = level.gems.len() as i32;
        let gems_collected = vec![false; level.gems.len()];
        Self {
            level,
            player,
            bats,
            slimes,
            gems_collected,
            total_gems,
            particles: ParticleSystem::new(),
            elapsed: 0.0,
            time: 0.0,
        }
    }

    fn update(&mut self, dt: f32) -> GameState {
        self.time += dt;
        self.elapsed += dt;
        self.player.update(dt, &self.level);

        for bat in &mut self.bats {
            bat.update(dt);
        }
        for slime in &mut self.slimes {
            slime.update(dt);
        }

        let gem_color = Color::from_rgba(90, 220, 230, 255);
        let player_rect = self.player.rect();
        for (i, gem_pos) in self.level.gems.iter().enumerate() {
            if self.gems_collected[i] {
                continue;
            }
            let gem_rect = Rect::new(gem_pos.x - 8.0, gem_pos.y - 8.0, 16.0, 16.0);
            if player_rect.overlaps(&gem_rect) {
                self.gems_collected[i] = true;
                self.player.gems += 1;
                self.particles.spawn_burst(*gem_pos, gem_color, 12);
            }
        }

        let hazard_color = Color::from_rgba(220, 60, 60, 255);
        if !self.player.dead {
            for (col, row) in &self.level.spikes {
                let hazard_rect = Rect::new(
                    *col as f32 * TILE + 4.0,
                    *row as f32 * TILE + 10.0,
                    TILE - 8.0,
                    TILE - 10.0,
                );
                if self.player.rect().overlaps(&hazard_rect) {
                    let hurt_pos = vec2(*col as f32 * TILE + TILE / 2.0, *row as f32 * TILE);
                    if self.player.take_damage(hurt_pos) {
                        self.particles.spawn_burst(hurt_pos, hazard_color, 10);
                    }
                }
            }
        }

        if !self.player.dead {
            for bat in &mut self.bats {
                if !bat.alive {
                    continue;
                }
                let player_rect = self.player.rect();
                if player_rect.overlaps(&bat.rect()) {
                    let stomp = self.player.vel.y > 0.0
                        && player_rect.y + player_rect.h - bat.rect().y < 16.0;
                    if stomp {
                        bat.alive = false;
                        self.player.vel.y = -420.0;
                        self.particles.spawn_burst(bat.pos, Color::from_rgba(120, 80, 160, 255), 10);
                    } else if self.player.take_damage(bat.pos) {
                        self.particles.spawn_burst(bat.pos, hazard_color, 10);
                    }
                }
            }
            for slime in &mut self.slimes {
                if !slime.alive {
                    continue;
                }
                let player_rect = self.player.rect();
                if player_rect.overlaps(&slime.rect()) {
                    let stomp = self.player.vel.y > 0.0
                        && player_rect.y + player_rect.h - slime.rect().y < 16.0;
                    if stomp {
                        slime.alive = false;
                        self.player.vel.y = -420.0;
                        self.particles.spawn_burst(slime.pos, Color::from_rgba(90, 200, 110, 255), 10);
                    } else if self.player.take_damage(slime.pos) {
                        self.particles.spawn_burst(slime.pos, hazard_color, 10);
                    }
                }
            }
        }

        if !self.player.dead && !self.player.won {
            let portal_rect = Rect::new(self.level.portal.x - 20.0, self.level.portal.y - 48.0, 40.0, 56.0);
            if self.player.rect().overlaps(&portal_rect) {
                self.player.won = true;
            }
        }

        self.particles.update(dt);

        if self.player.dead {
            GameState::Dead
        } else if self.player.won {
            GameState::Won
        } else {
            GameState::Playing
        }
    }

    fn cam_x(&self, screen_w: f32) -> f32 {
        let target = self.player.pos.x + player::PLAYER_W / 2.0 - screen_w / 2.0;
        target.clamp(0.0, (self.level.width_px() - screen_w).max(0.0))
    }

    fn draw(&self, assets: &Assets) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let cam_x = self.cam_x(screen_w);

        background::draw_background(cam_x, screen_w, screen_h);

        let min_col = ((cam_x) / TILE).floor().max(0.0) as i32;
        let max_col = ((cam_x + screen_w) / TILE).ceil().min(COLS as f32) as i32;

        for row in 0..ROWS as i32 {
            for col in min_col..max_col {
                if self.level.tile_at(col as usize, row as usize) == TileKind::Ground {
                    let variant = ((col + row) % 2) as usize;
                    let sx = col as f32 * TILE - cam_x;
                    let sy = row as f32 * TILE + OFFSET_Y;
                    draw_texture(&assets.ground_tiles[variant], sx, sy, WHITE);
                }
            }
        }

        for (col, row) in &self.level.spikes {
            let sx = *col as f32 * TILE - cam_x;
            let sy = *row as f32 * TILE + OFFSET_Y;
            draw_texture(&assets.spike, sx, sy, WHITE);
        }

        for (x, y) in &self.level.torches {
            let sx = x - cam_x;
            let sy = y + OFFSET_Y;
            draw_texture(&assets.torch.base, sx, sy, WHITE);
            let frame = ((self.time * 8.0) as usize + (*x as usize)) % 3;
            let flame = &assets.torch.flame[frame];
            draw_texture(flame, sx, sy - 12.0, WHITE);
        }

        for (i, gem_pos) in self.level.gems.iter().enumerate() {
            if self.gems_collected[i] {
                continue;
            }
            let bob = (self.time * 3.0 + i as f32).sin() * 3.0;
            let frame = if ((self.time * 4.0) as i32 + i as i32) % 2 == 0 { 0 } else { 1 };
            let tex = &assets.gem_frames[frame];
            let sx = gem_pos.x - cam_x - tex.width() / 2.0;
            let sy = gem_pos.y + OFFSET_Y - tex.height() / 2.0 + bob;
            draw_texture(tex, sx, sy, WHITE);
        }

        draw_portal(self.level.portal, self.time, cam_x, OFFSET_Y);

        for slime in &self.slimes {
            slime.draw_at(&assets.slime, vec2(slime.pos.x - cam_x, slime.pos.y + OFFSET_Y));
        }
        for bat in &self.bats {
            bat.draw_at(&assets.bat, vec2(bat.pos.x - cam_x, bat.pos.y + OFFSET_Y));
        }

        self.player.draw_at(
            &assets.player,
            vec2(self.player.pos.x - cam_x, self.player.pos.y + OFFSET_Y),
        );

        self.particles.draw_offset(cam_x, OFFSET_Y);

        assets.hud.draw(self.player.health, player::MAX_HEALTH, self.player.gems, self.total_gems, self.elapsed);
    }
}

fn draw_portal(pos: Vec2, time: f32, cam_x: f32, offset_y: f32) {
    let sx = pos.x - cam_x;
    let sy = pos.y + offset_y - 24.0;
    for i in 0..4 {
        let t = time * 2.0 + i as f32 * 0.6;
        let r = 8.0 + (t.sin() * 0.5 + 0.5) * 10.0 + i as f32 * 3.0;
        let alpha = (0.55 - i as f32 * 0.12).max(0.0);
        draw_circle_lines(sx, sy, r, 2.0, Color::new(0.55, 0.85, 1.0, alpha));
    }
    draw_circle(sx, sy, 6.0, Color::new(0.9, 0.95, 1.0, 0.9));
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = Assets::load();
    let mut world = World::new();
    let mut state = GameState::Title;

    loop {
        let dt = get_frame_time().min(1.0 / 30.0);

        match state {
            GameState::Title => {
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                state = world.update(dt);
            }
            GameState::Dead | GameState::Won => {
                world.particles.update(dt);
                if is_key_pressed(KeyCode::Enter) {
                    world = World::new();
                    state = GameState::Title;
                }
            }
        }

        clear_background(Color::from_rgba(8, 6, 16, 255));

        if state == GameState::Title {
            background::draw_background(0.0, screen_width(), screen_height());
            draw_centered_text("EMBER KNIGHT", screen_height() * 0.32, 64, WHITE);
            draw_centered_text(
                "A pixel torch-lit descent through the deep caves",
                screen_height() * 0.32 + 46.0,
                22,
                Color::from_rgba(200, 190, 220, 255),
            );
            draw_centered_text(
                "Arrows / WASD to move    Space to jump    Avoid spikes & foes, collect gems",
                screen_height() * 0.6,
                22,
                LIGHTGRAY,
            );
            draw_centered_text("Press ENTER to begin", screen_height() * 0.72, 28, YELLOW);
        } else {
            world.draw(&assets);

            if state == GameState::Dead {
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.55));
                draw_centered_text("YOU DIED", screen_height() * 0.4, 56, Color::from_rgba(220, 70, 70, 255));
                draw_centered_text("Press ENTER to retry", screen_height() * 0.4 + 50.0, 26, WHITE);
            } else if state == GameState::Won {
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.55));
                draw_centered_text("LEVEL COMPLETE!", screen_height() * 0.36, 56, Color::from_rgba(250, 210, 90, 255));
                draw_centered_text(
                    &format!("Gems collected: {}/{}", world.player.gems, world.total_gems),
                    screen_height() * 0.36 + 50.0,
                    26,
                    WHITE,
                );
                draw_centered_text(
                    &format!("Time: {:.1}s", world.elapsed),
                    screen_height() * 0.36 + 84.0,
                    26,
                    WHITE,
                );
                draw_centered_text("Press ENTER to play again", screen_height() * 0.36 + 128.0, 24, YELLOW);
            }
        }

        next_frame().await;
    }
}
