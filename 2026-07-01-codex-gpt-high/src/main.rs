use macroquad::prelude::*;

const SCREEN_W: f32 = 320.0;
const SCREEN_H: f32 = 180.0;
const WORLD_W: f32 = 2140.0;

const fn c(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
}

const INK: Color = c(22, 19, 34, 255);
const NIGHT: Color = c(38, 36, 62, 255);
const PURPLE: Color = c(73, 57, 89, 255);
const MIST: Color = c(111, 91, 116, 255);
const CREAM: Color = c(255, 226, 164, 255);
const GOLD: Color = c(242, 170, 59, 255);
const ORANGE: Color = c(215, 92, 72, 255);
const RED: Color = c(154, 54, 68, 255);
const TEAL: Color = c(58, 127, 124, 255);
const PALE: Color = c(145, 190, 163, 255);
const GRASS: Color = c(75, 119, 83, 255);
const DIRT: Color = c(82, 57, 66, 255);
const DARK_DIRT: Color = c(50, 39, 54, 255);

#[derive(Clone, Copy)]
struct FRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Clone, Copy, Default)]
struct Platform {
    rect: FRect,
}

impl Default for FRect {
    fn default() -> Self {
        FRect { x: 0.0, y: 0.0, w: 0.0, h: 0.0 }
    }
}

#[derive(Clone, Copy, Default)]
struct Gem {
    x: f32,
    y: f32,
    taken: bool,
}

#[derive(Clone, Copy, Default)]
struct Enemy {
    x: f32,
    y: f32,
    left: f32,
    right: f32,
    vx: f32,
    alive: bool,
}

#[derive(Clone, Copy, Default)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    color: Color,
    active: bool,
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    w: f32,
    h: f32,
    grounded: bool,
    facing: i32,
    hearts: i32,
    gems: i32,
    invuln: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            x: 28.0,
            y: 130.0,
            vx: 0.0,
            vy: 0.0,
            w: 11.0,
            h: 16.0,
            grounded: false,
            facing: 1,
            hearts: 3,
            gems: 0,
            invuln: 0.0,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum GameMode {
    Title,
    Playing,
    Won,
}

struct Game {
    platforms: Vec<Platform>,
    gems: Vec<Gem>,
    enemies: Vec<Enemy>,
    particles: Vec<Particle>,
    player: Player,
    mode: GameMode,
    camera_x: f32,
    game_time: f32,
    scale: f32,
    off_x: f32,
    off_y: f32,
}

fn clampf(v: f32, lo: f32, hi: f32) -> f32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

fn overlaps(a: FRect, b: FRect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

impl Game {
    fn new() -> Self {
        let mut g = Game {
            platforms: Vec::new(),
            gems: Vec::new(),
            enemies: Vec::new(),
            particles: vec![Particle::default(); 80],
            player: Player::default(),
            mode: GameMode::Title,
            camera_x: 0.0,
            game_time: 0.0,
            scale: 4.0,
            off_x: 0.0,
            off_y: 0.0,
        };
        g.reset_level();
        g
    }

    fn box_(&self, x: f32, y: f32, w: f32, h: f32, col: Color) {
        if w <= 0.0 || h <= 0.0 {
            return;
        }
        draw_rectangle(
            self.off_x + x.trunc() * self.scale,
            self.off_y + y.trunc() * self.scale,
            w.trunc() * self.scale,
            h.trunc() * self.scale,
            col,
        );
    }

    fn world_box(&self, x: f32, y: f32, w: f32, h: f32, col: Color) {
        self.box_(x - self.camera_x, y, w, h, col);
    }

    fn reset_level(&mut self) {
        let plats: [[f32; 4]; 20] = [
            [0.0, 154.0, 250.0, 40.0],
            [282.0, 154.0, 246.0, 40.0],
            [558.0, 154.0, 252.0, 40.0],
            [842.0, 154.0, 310.0, 40.0],
            [1185.0, 154.0, 240.0, 40.0],
            [1455.0, 154.0, 285.0, 40.0],
            [1772.0, 154.0, 368.0, 40.0],
            [105.0, 121.0, 54.0, 8.0],
            [196.0, 96.0, 46.0, 8.0],
            [332.0, 118.0, 68.0, 8.0],
            [449.0, 88.0, 51.0, 8.0],
            [610.0, 112.0, 58.0, 8.0],
            [704.0, 82.0, 60.0, 8.0],
            [890.0, 112.0, 60.0, 8.0],
            [1000.0, 84.0, 66.0, 8.0],
            [1105.0, 60.0, 45.0, 8.0],
            [1230.0, 108.0, 70.0, 8.0],
            [1350.0, 78.0, 55.0, 8.0],
            [1515.0, 105.0, 72.0, 8.0],
            [1650.0, 73.0, 58.0, 8.0],
        ];
        self.platforms = plats
            .iter()
            .map(|p| Platform {
                rect: FRect { x: p[0], y: p[1], w: p[2], h: p[3] },
            })
            .collect();

        let gem_data: [[f32; 2]; 18] = [
            [126.0, 108.0],
            [217.0, 82.0],
            [300.0, 137.0],
            [360.0, 105.0],
            [470.0, 74.0],
            [585.0, 137.0],
            [638.0, 98.0],
            [732.0, 68.0],
            [870.0, 137.0],
            [920.0, 98.0],
            [1032.0, 70.0],
            [1125.0, 46.0],
            [1210.0, 137.0],
            [1265.0, 94.0],
            [1376.0, 64.0],
            [1490.0, 137.0],
            [1550.0, 91.0],
            [1680.0, 59.0],
        ];
        self.gems = gem_data
            .iter()
            .map(|d| Gem { x: d[0], y: d[1], taken: false })
            .collect();

        let enemy_data: [[f32; 5]; 6] = [
            [350.0, 142.0, 300.0, 490.0, 22.0],
            [650.0, 142.0, 580.0, 790.0, -20.0],
            [910.0, 100.0, 890.0, 940.0, 14.0],
            [1248.0, 96.0, 1232.0, 1288.0, 16.0],
            [1540.0, 93.0, 1518.0, 1574.0, -17.0],
            [1840.0, 142.0, 1790.0, 1980.0, 24.0],
        ];
        self.enemies = enemy_data
            .iter()
            .map(|e| Enemy {
                x: e[0],
                y: e[1],
                left: e[2],
                right: e[3],
                vx: e[4],
                alive: true,
            })
            .collect();

        for p in self.particles.iter_mut() {
            p.active = false;
        }
        self.player = Player::default();
        self.camera_x = 0.0;
        self.game_time = 0.0;
    }

    fn respawn(&mut self) {
        self.player.x = 28.0;
        self.player.y = 130.0;
        self.player.vx = 0.0;
        self.player.vy = 0.0;
        self.player.invuln = 1.5;
        self.camera_x = 0.0;
    }

    fn spawn_burst(&mut self, x: f32, y: f32, col: Color, count: i32) {
        let mut made = 0;
        for p in self.particles.iter_mut() {
            if !p.active && made < count {
                let seed = ((made * 37 + x as i32) % 17) as f32;
                p.active = true;
                p.x = x;
                p.y = y;
                p.vx = (seed - 8.0) * 5.0;
                p.vy = -35.0 - ((made * 11) % 28) as f32;
                p.life = 0.45 + ((made % 4) as f32) * 0.08;
                p.color = col;
                made += 1;
            }
        }
    }

    fn hurt_player(&mut self) {
        if self.player.invuln > 0.0 {
            return;
        }
        self.player.hearts -= 1;
        let (px, py) = (self.player.x + 5.0, self.player.y + 8.0);
        self.spawn_burst(px, py, ORANGE, 12);
        if self.player.hearts <= 0 {
            self.player.hearts = 3;
        }
        self.respawn();
    }

    fn update_game(&mut self, dt: f32, mv: i32, jump_pressed: bool) {
        if self.player.invuln > 0.0 {
            self.player.invuln -= dt;
        }

        let target = mv as f32 * 82.0;
        let mut accel = 620.0;
        if mv == 0 {
            accel = 800.0;
        }
        if self.player.vx < target {
            self.player.vx += accel * dt;
            if self.player.vx > target {
                self.player.vx = target;
            }
        }
        if self.player.vx > target {
            self.player.vx -= accel * dt;
            if self.player.vx < target {
                self.player.vx = target;
            }
        }
        if mv != 0 {
            self.player.facing = mv;
        }

        if jump_pressed && self.player.grounded {
            self.player.vy = -178.0;
            self.player.grounded = false;
            let (px, py) = (self.player.x + 5.0, self.player.y + 15.0);
            self.spawn_burst(px, py, MIST, 5);
        }
        self.player.vy += 475.0 * dt;
        if self.player.vy > 240.0 {
            self.player.vy = 240.0;
        }

        self.player.x += self.player.vx * dt;
        let mut pr = FRect {
            x: self.player.x,
            y: self.player.y,
            w: self.player.w,
            h: self.player.h,
        };
        for it in self.platforms.iter() {
            if overlaps(pr, it.rect) {
                if self.player.vx > 0.0 {
                    self.player.x = it.rect.x - self.player.w;
                }
                if self.player.vx < 0.0 {
                    self.player.x = it.rect.x + it.rect.w;
                }
                self.player.vx = 0.0;
                pr.x = self.player.x;
            }
        }

        let old_bottom = self.player.y + self.player.h;
        self.player.y += self.player.vy * dt;
        self.player.grounded = false;
        pr = FRect {
            x: self.player.x,
            y: self.player.y,
            w: self.player.w,
            h: self.player.h,
        };
        for it in self.platforms.iter() {
            if overlaps(pr, it.rect) {
                if self.player.vy > 0.0 && old_bottom <= it.rect.y + 3.0 {
                    self.player.y = it.rect.y - self.player.h;
                    self.player.vy = 0.0;
                    self.player.grounded = true;
                } else if self.player.vy < 0.0 {
                    self.player.y = it.rect.y + it.rect.h;
                    self.player.vy = 0.0;
                }
                pr.y = self.player.y;
            }
        }

        if self.player.y > 200.0 {
            self.hurt_player();
        }
        self.player.x = clampf(self.player.x, 0.0, WORLD_W - self.player.w);

        pr = FRect {
            x: self.player.x,
            y: self.player.y,
            w: self.player.w,
            h: self.player.h,
        };
        let mut bursts: Vec<(f32, f32, Color, i32)> = Vec::new();
        for it in self.gems.iter_mut() {
            if !it.taken
                && overlaps(
                    pr,
                    FRect { x: it.x - 3.0, y: it.y - 4.0, w: 7.0, h: 9.0 },
                )
            {
                it.taken = true;
                self.player.gems += 1;
                bursts.push((it.x, it.y, GOLD, 9));
            }
        }
        for (bx, by, col, n) in bursts {
            self.spawn_burst(bx, by, col, n);
        }

        let mut hurt = false;
        let mut stomp_bursts: Vec<(f32, f32, Color, i32)> = Vec::new();
        for it in self.enemies.iter_mut() {
            if !it.alive {
                continue;
            }
            it.x += it.vx * dt;
            if it.x < it.left {
                it.x = it.left;
                it.vx = it.vx.abs();
            }
            if it.x > it.right {
                it.x = it.right;
                it.vx = -it.vx.abs();
            }
            let er = FRect { x: it.x, y: it.y, w: 13.0, h: 12.0 };
            if overlaps(pr, er) {
                if self.player.vy > 25.0 && old_bottom <= it.y + 5.0 {
                    it.alive = false;
                    self.player.vy = -118.0;
                    stomp_bursts.push((it.x + 6.0, it.y + 6.0, TEAL, 12));
                } else {
                    hurt = true;
                }
            }
        }
        for (bx, by, col, n) in stomp_bursts {
            self.spawn_burst(bx, by, col, n);
        }
        if hurt {
            self.hurt_player();
        }

        if self.player.x > 2050.0 {
            self.mode = GameMode::Won;
        }

        for p in self.particles.iter_mut() {
            if !p.active {
                continue;
            }
            p.life -= dt;
            if p.life <= 0.0 {
                p.active = false;
                continue;
            }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += 180.0 * dt;
        }

        let desired = self.player.x - 104.0;
        self.camera_x += (desired - self.camera_x) * clampf(dt * 5.0, 0.0, 1.0);
        self.camera_x = clampf(self.camera_x, 0.0, WORLD_W - SCREEN_W);
    }

    fn draw_background(&self) {
        self.box_(0.0, 0.0, SCREEN_W, SCREEN_H, NIGHT);

        self.box_(254.0, 17.0, 18.0, 18.0, CREAM);
        self.box_(250.0, 17.0, 9.0, 9.0, NIGHT);
        for i in 0..24 {
            let x = ((i * 73 + 19) % 320) as f32;
            let y = ((i * 37 + 13) % 68) as f32;
            let mut twinkle = 1.0;
            if ((self.game_time * 4.0) as i32 + i) % 5 == 0 {
                twinkle = 2.0;
            }
            self.box_(x, y, twinkle, twinkle, c(150, 145, 174, 255));
        }

        let far_shift = ((self.camera_x * 0.10) as i32 % 96) as f32;
        for i in -1..5 {
            let bx = (i * 96) as f32 - far_shift;
            for step in 0..96 {
                let ridge = 110.0 - ((step - 48) as i32).abs() as f32 / 2.0;
                let ridge = ridge.trunc();
                self.box_(bx + step as f32, ridge, 1.0, 154.0 - ridge, PURPLE);
            }
        }
        let near_shift = ((self.camera_x * 0.23) as i32 % 74) as f32;
        for i in -1..6 {
            let bx = (i * 74) as f32 - near_shift;
            for step in 0..74 {
                let ridge = 131.0 - ((step - 37) as i32).abs() as f32 / 3.0;
                let ridge = ridge.trunc();
                self.box_(bx + step as f32, ridge, 1.0, 154.0 - ridge, c(50, 57, 75, 255));
            }
            let tx = bx + 18.0;
            self.box_(tx, 108.0, 2.0, 37.0, INK);
            self.box_(tx - 7.0, 115.0, 16.0, 3.0, INK);
            self.box_(tx - 10.0, 122.0, 22.0, 3.0, INK);
            self.box_(tx - 13.0, 130.0, 28.0, 4.0, INK);
        }
    }

    fn draw_platform(&self, p: &Platform) {
        let sx = p.rect.x - self.camera_x;
        if sx + p.rect.w < 0.0 || sx > SCREEN_W {
            return;
        }
        self.box_(sx, p.rect.y, p.rect.w, p.rect.h, DARK_DIRT);
        self.box_(sx, p.rect.y, p.rect.w, 3.0, GRASS);
        self.box_(sx, p.rect.y + 3.0, p.rect.w, 3.0, DIRT);
        let count = (p.rect.w / 18.0) as i32;
        if count > 0 {
            for i in 0..count {
                let nx = sx + 8.0 + (i * 18) as f32;
                self.box_(nx, p.rect.y + 8.0, 6.0, 2.0, c(61, 47, 60, 255));
            }
        }
    }

    fn draw_gem(&self, g: &Gem, index: i32) {
        if g.taken {
            return;
        }
        let x = g.x - self.camera_x;
        let mut bob = ((self.game_time * 6.0) as i32 + index) % 6;
        if bob > 3 {
            bob = 6 - bob;
        }
        let y = g.y - (bob / 2) as f32;
        self.box_(x - 1.0, y - 5.0, 3.0, 1.0, CREAM);
        self.box_(x - 3.0, y - 4.0, 7.0, 5.0, GOLD);
        self.box_(x - 1.0, y - 5.0, 3.0, 9.0, GOLD);
        self.box_(x - 1.0, y - 3.0, 2.0, 3.0, CREAM);
    }

    fn draw_enemy(&self, e: &Enemy) {
        if !e.alive {
            return;
        }
        let x = e.x - self.camera_x;
        let y = e.y;
        self.box_(x + 2.0, y, 9.0, 2.0, INK);
        self.box_(x, y + 3.0, 13.0, 7.0, TEAL);
        self.box_(x + 2.0, y + 1.0, 9.0, 10.0, TEAL);
        self.box_(x + 2.0, y + 4.0, 2.0, 2.0, CREAM);
        self.box_(x + 9.0, y + 4.0, 2.0, 2.0, CREAM);
        self.box_(x + 3.0, y + 5.0, 1.0, 1.0, INK);
        self.box_(x + 9.0, y + 5.0, 1.0, 1.0, INK);
        self.box_(x + 1.0, y + 11.0, 4.0, 1.0, INK);
        self.box_(x + 8.0, y + 11.0, 4.0, 1.0, INK);
    }

    fn draw_player(&self) {
        if self.player.invuln > 0.0 && ((self.game_time * 14.0) as i32 % 2) == 0 {
            return;
        }
        let x = self.player.x - self.camera_x;
        let y = self.player.y;
        let mut run_frame = (self.game_time * 10.0) as i32 % 2;
        if self.player.vx.abs() < 5.0 {
            run_frame = 0;
        }

        if self.player.grounded {
            self.box_(x - 2.0, y + 16.0, 15.0, 2.0, c(29, 27, 39, 255));
        }
        if run_frame == 0 {
            self.box_(x + 1.0, y + 14.0, 4.0, 2.0, INK);
            self.box_(x + 7.0, y + 14.0, 4.0, 2.0, INK);
        } else {
            self.box_(x, y + 14.0, 5.0, 2.0, INK);
            self.box_(x + 8.0, y + 14.0, 4.0, 2.0, INK);
        }
        self.box_(x + 1.0, y + 7.0, 10.0, 8.0, RED);
        self.box_(x + 3.0, y + 3.0, 7.0, 6.0, CREAM);
        self.box_(x + 2.0, y + 1.0, 8.0, 4.0, INK);
        self.box_(x + 1.0, y + 3.0, 2.0, 4.0, INK);
        let eye_x = if self.player.facing > 0 { x + 8.0 } else { x + 4.0 };
        self.box_(eye_x, y + 5.0, 1.0, 1.0, INK);
        self.box_(x, y + 7.0, 12.0, 2.0, GOLD);
        if self.player.facing > 0 {
            self.box_(x - 4.0, y + 8.0, 5.0, 2.0, GOLD);
            self.box_(x - 6.0, y + 9.0, 3.0, 1.0, GOLD);
        } else {
            self.box_(x + 11.0, y + 8.0, 5.0, 2.0, GOLD);
            self.box_(x + 15.0, y + 9.0, 3.0, 1.0, GOLD);
        }
        self.box_(x + 3.0, y + 10.0, 2.0, 3.0, ORANGE);
    }

    fn character(&self, ch: u8, x: f32, y: f32, scale: f32, col: Color) {
        let rows = glyph(ch);
        for yy in 0..5 {
            for gx in 0..3 {
                if rows[yy] & (1 << (2 - gx)) != 0 {
                    self.box_(
                        x + (gx as f32) * scale,
                        y + (yy as f32) * scale,
                        scale,
                        scale,
                        col,
                    );
                }
            }
        }
    }

    fn text(&self, s: &str, x: f32, y: f32, scale: f32, col: Color) {
        let mut cx = x;
        for ch in s.bytes() {
            self.character(ch, cx, y, scale, col);
            cx += 4.0 * scale;
        }
    }

    fn draw_heart(&self, x: f32, y: f32, full: bool) {
        let col = if full { ORANGE } else { c(67, 54, 72, 255) };
        self.box_(x + 1.0, y, 2.0, 1.0, col);
        self.box_(x + 5.0, y, 2.0, 1.0, col);
        self.box_(x, y + 1.0, 8.0, 3.0, col);
        self.box_(x + 1.0, y + 4.0, 6.0, 2.0, col);
        self.box_(x + 3.0, y + 6.0, 2.0, 1.0, col);
    }

    fn draw_world(&self) {
        self.draw_background();
        for p in self.platforms.iter() {
            self.draw_platform(p);
        }
        for (i, g) in self.gems.iter().enumerate() {
            self.draw_gem(g, i as i32);
        }
        for e in self.enemies.iter() {
            self.draw_enemy(e);
        }

        self.world_box(2041.0, 106.0, 3.0, 48.0, INK);
        self.world_box(2073.0, 106.0, 3.0, 48.0, INK);
        self.world_box(2038.0, 105.0, 41.0, 3.0, INK);
        self.world_box(2046.0, 99.0, 25.0, 3.0, INK);
        let glow = (self.game_time * 8.0) as i32 % 3;
        let glowf = glow as f32;
        self.world_box(
            2054.0 - glowf,
            111.0 - glowf,
            8.0 + glowf * 2.0,
            9.0 + glowf * 2.0,
            c(103, 66, 65, 255),
        );
        self.world_box(2056.0, 113.0, 6.0, 7.0, GOLD);
        self.world_box(2057.0, 114.0, 4.0, 4.0, CREAM);

        for p in self.particles.iter() {
            if p.active {
                self.world_box(p.x, p.y, 2.0, 2.0, p.color);
            }
        }
        self.draw_player();

        self.box_(7.0, 7.0, 91.0, 15.0, c(22, 19, 34, 225));
        self.box_(8.0, 8.0, 89.0, 1.0, PURPLE);
        for i in 0..3 {
            self.draw_heart(12.0 + (i * 11) as f32, 11.0, i < self.player.hearts);
        }
        self.draw_gem(&Gem { x: self.camera_x + 57.0, y: 15.0, taken: false }, 0);
        let tens = self.player.gems / 10;
        let ones = self.player.gems % 10;
        if tens > 0 {
            self.character(b'0' + tens as u8, 67.0, 11.0, 1.0, CREAM);
        }
        self.character(b'0' + ones as u8, 72.0, 11.0, 1.0, CREAM);
        self.text("-18", 78.0, 11.0, 1.0, MIST);

        self.box_(242.0, 9.0, 69.0, 4.0, INK);
        self.box_(243.0, 10.0, 67.0 * self.player.x / (WORLD_W - 90.0), 2.0, GOLD);
        self.box_(
            242.0 + 67.0 * self.player.x / (WORLD_W - 90.0),
            8.0,
            2.0,
            6.0,
            CREAM,
        );
    }

    fn draw_title(&mut self) {
        self.draw_background();
        self.box_(0.0, 145.0, SCREEN_W, 35.0, DARK_DIRT);
        self.box_(0.0, 145.0, SCREEN_W, 3.0, GRASS);
        self.camera_x = 0.0;
        self.player.x = 154.0;
        self.player.y = 128.0;
        self.draw_player();

        self.box_(57.0, 34.0, 206.0, 59.0, c(22, 19, 34, 235));
        self.box_(60.0, 37.0, 200.0, 1.0, GOLD);
        self.box_(60.0, 89.0, 200.0, 1.0, PURPLE);
        self.text("DUSK RUNNER", 73.0, 47.0, 3.0, CREAM);
        self.text("A TINY TWILIGHT ADVENTURE", 108.0, 78.0, 1.0, MIST);

        if ((self.game_time * 3.0) as i32 % 2) == 0 {
            self.text("PRESS SPACE", 116.0, 111.0, 1.0, GOLD);
        }
        self.text("ARROWS OR AD  MOVE", 96.0, 158.0, 1.0, PALE);
        self.text("SPACE  JUMP", 132.0, 168.0, 1.0, PALE);
    }

    fn draw_won(&self) {
        self.draw_world();
        self.box_(63.0, 46.0, 194.0, 78.0, c(22, 19, 34, 240));
        self.box_(67.0, 50.0, 186.0, 2.0, GOLD);
        self.box_(67.0, 118.0, 186.0, 2.0, PURPLE);
        self.text("DAWN FOUND", 79.0, 60.0, 3.0, CREAM);
        self.text("YOU REACHED THE LANTERN", 112.0, 91.0, 1.0, PALE);
        self.text("PRESS R TO RUN AGAIN", 120.0, 106.0, 1.0, GOLD);
    }

    fn update_layout(&mut self) {
        let s = (screen_width() / SCREEN_W)
            .min(screen_height() / SCREEN_H)
            .floor()
            .max(1.0);
        self.scale = s;
        self.off_x = (screen_width() - SCREEN_W * s) * 0.5;
        self.off_y = (screen_height() - SCREEN_H * s) * 0.5;
    }
}

fn glyph(ch: u8) -> [u8; 5] {
    match ch {
        b'A' => [2, 5, 7, 5, 5],
        b'B' => [6, 5, 6, 5, 6],
        b'C' => [3, 4, 4, 4, 3],
        b'D' => [6, 5, 5, 5, 6],
        b'E' => [7, 4, 6, 4, 7],
        b'F' => [7, 4, 6, 4, 4],
        b'G' => [3, 4, 5, 5, 3],
        b'H' => [5, 5, 7, 5, 5],
        b'I' => [7, 2, 2, 2, 7],
        b'J' => [1, 1, 1, 5, 2],
        b'K' => [5, 5, 6, 5, 5],
        b'L' => [4, 4, 4, 4, 7],
        b'M' => [5, 7, 7, 5, 5],
        b'N' => [5, 7, 7, 7, 5],
        b'O' => [2, 5, 5, 5, 2],
        b'P' => [6, 5, 6, 4, 4],
        b'Q' => [2, 5, 5, 3, 1],
        b'R' => [6, 5, 6, 5, 5],
        b'S' => [3, 4, 2, 1, 6],
        b'T' => [7, 2, 2, 2, 2],
        b'U' => [5, 5, 5, 5, 7],
        b'V' => [5, 5, 5, 5, 2],
        b'W' => [5, 5, 7, 7, 5],
        b'X' => [5, 5, 2, 5, 5],
        b'Y' => [5, 5, 2, 2, 2],
        b'Z' => [7, 1, 2, 4, 7],
        b'0' => [7, 5, 5, 5, 7],
        b'1' => [2, 6, 2, 2, 7],
        b'2' => [6, 1, 7, 4, 7],
        b'3' => [6, 1, 3, 1, 6],
        b'4' => [5, 5, 7, 1, 1],
        b'5' => [7, 4, 6, 1, 6],
        b'6' => [3, 4, 7, 5, 7],
        b'7' => [7, 1, 2, 2, 2],
        b'8' => [7, 5, 7, 5, 7],
        b'9' => [7, 5, 7, 1, 6],
        b':' => [0, 2, 0, 2, 0],
        b'-' => [0, 0, 7, 0, 0],
        _ => [0, 0, 0, 0, 0],
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dusk Runner - Rust".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let jump_pressed = is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::W);
        let start_pressed = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
        let restart_pressed = is_key_pressed(KeyCode::R);

        let dt = clampf(get_frame_time(), 0.0, 0.033);
        game.game_time += dt;

        let mut mv = 0;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            mv -= 1;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            mv += 1;
        }

        match game.mode {
            GameMode::Title => {
                if start_pressed {
                    game.reset_level();
                    game.mode = GameMode::Playing;
                }
            }
            GameMode::Playing => {
                game.update_game(dt, mv, jump_pressed);
            }
            GameMode::Won => {
                if restart_pressed {
                    game.reset_level();
                    game.mode = GameMode::Playing;
                }
            }
        }

        game.update_layout();
        clear_background(BLACK);
        match game.mode {
            GameMode::Title => game.draw_title(),
            GameMode::Won => game.draw_won(),
            GameMode::Playing => game.draw_world(),
        }

        next_frame().await;
    }
}
