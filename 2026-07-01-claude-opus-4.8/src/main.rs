use macroquad::prelude::*;

const VW: f32 = 320.0;
const VH: f32 = 180.0;

const HORIZON: f32 = 150.0;
const LEVEL_W: f32 = 3200.0;

const GRAVITY: f32 = 720.0;
const MOVE_SPEED: f32 = 96.0;
const JUMP_VEL: f32 = -248.0;
const MAX_FALL: f32 = 360.0;
const ACCEL: f32 = 900.0;
const FRICTION: f32 = 1100.0;

const PLAYER_W: f32 = 12.0;
const PLAYER_H: f32 = 14.0;

fn col4(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::new(r, g, b, a)
}

const SKY_TOP: Color = Color::new(0.27, 0.55, 0.86, 1.0);
const SKY_BOTTOM: Color = Color::new(0.65, 0.82, 0.92, 1.0);
const HILL_FAR: Color = Color::new(0.42, 0.62, 0.55, 1.0);
const HILL_MID: Color = Color::new(0.30, 0.56, 0.40, 1.0);
const HILL_NEAR: Color = Color::new(0.20, 0.46, 0.30, 1.0);

const GRASS: Color = Color::new(0.34, 0.74, 0.34, 1.0);
const GRASS_HI: Color = Color::new(0.50, 0.88, 0.46, 1.0);
const DIRT: Color = Color::new(0.50, 0.34, 0.22, 1.0);
const DIRT_DK: Color = Color::new(0.36, 0.24, 0.16, 1.0);

#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Clone, Copy)]
struct Coin {
    x: f32,
    y: f32,
    taken: bool,
}

#[derive(Clone, Copy)]
struct Player {
    pos: Vec2,
    vel: Vec2,
    on_ground: bool,
    facing: f32,
    anim_t: f32,
    coyote: f32,
    jump_buffer: f32,
    squash: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            pos: vec2(0.0, 0.0),
            vel: vec2(0.0, 0.0),
            on_ground: false,
            facing: 1.0,
            anim_t: 0.0,
            coyote: 0.0,
            jump_buffer: 0.0,
            squash: 0.0,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum GameState {
    Start,
    Play,
    Win,
}

const SLIME: [&str; 11] = [
    "....KKKKKK....",
    "..KKllllllKK..",
    ".KllllllllllK.",
    ".KllllllllllK.",
    "KgggwwggwwgggK",
    "KgggweggwegggK",
    "KggggggggggggK",
    "KgggggmmgggggK",
    "KggggggggggggK",
    ".KggggggggggK.",
    "..KKKKKKKKKK..",
];

const COIN: [&str; 8] = [
    "..KKKK..", ".KywyyK.", "KyywyyyK", "KyywyyyK", "KyyyyyyK", "KYyyyyYK", ".KYyyYK.", "..KKKK..",
];

const SPIKE: [&str; 8] = [
    "...KK...", "..KssK..", "..KssK..", ".KssssK.", ".KssssK.", "KsSSSSsK", "KSSSSSSK", "KKKKKKKK",
];

fn sprite_color(ch: u8) -> (Color, bool) {
    match ch {
        b'.' => (col4(0.0, 0.0, 0.0, 0.0), false),
        b'K' => (col4(0.10, 0.12, 0.14, 1.0), true),
        b'g' => (col4(0.30, 0.78, 0.38, 1.0), true),
        b'l' => (col4(0.55, 0.93, 0.55, 1.0), true),
        b'w' => (col4(0.97, 0.98, 0.95, 1.0), true),
        b'e' => (col4(0.08, 0.10, 0.12, 1.0), true),
        b'm' => (col4(0.15, 0.30, 0.18, 1.0), true),
        b'y' => (col4(1.00, 0.85, 0.25, 1.0), true),
        b'Y' => (col4(0.80, 0.58, 0.10, 1.0), true),
        b's' => (col4(0.74, 0.76, 0.82, 1.0), true),
        b'S' => (col4(0.45, 0.48, 0.55, 1.0), true),
        _ => (col4(1.0, 0.0, 1.0, 1.0), true),
    }
}

fn get_glyph(ch: u8) -> [&'static str; 5] {
    let mut c = ch;
    if c >= b'a' && c <= b'z' {
        c -= 32;
    }
    match c {
        b'A' => ["###", "# #", "###", "# #", "# #"],
        b'B' => ["## ", "# #", "## ", "# #", "## "],
        b'C' => ["###", "#  ", "#  ", "#  ", "###"],
        b'D' => ["## ", "# #", "# #", "# #", "## "],
        b'E' => ["###", "#  ", "###", "#  ", "###"],
        b'F' => ["###", "#  ", "###", "#  ", "#  "],
        b'G' => ["###", "#  ", "# #", "# #", "###"],
        b'H' => ["# #", "# #", "###", "# #", "# #"],
        b'I' => ["###", " # ", " # ", " # ", "###"],
        b'J' => ["  #", "  #", "  #", "# #", "###"],
        b'K' => ["# #", "# #", "## ", "# #", "# #"],
        b'L' => ["#  ", "#  ", "#  ", "#  ", "###"],
        b'M' => ["# #", "###", "###", "# #", "# #"],
        b'N' => ["# #", "###", "###", "###", "# #"],
        b'O' => ["###", "# #", "# #", "# #", "###"],
        b'P' => ["###", "# #", "###", "#  ", "#  "],
        b'Q' => ["###", "# #", "# #", "###", "  #"],
        b'R' => ["###", "# #", "## ", "# #", "# #"],
        b'S' => ["###", "#  ", "###", "  #", "###"],
        b'T' => ["###", " # ", " # ", " # ", " # "],
        b'U' => ["# #", "# #", "# #", "# #", "###"],
        b'V' => ["# #", "# #", "# #", "# #", " # "],
        b'W' => ["# #", "# #", "###", "###", "# #"],
        b'X' => ["# #", "# #", " # ", "# #", "# #"],
        b'Y' => ["# #", "# #", "###", " # ", " # "],
        b'Z' => ["###", "  #", " # ", "#  ", "###"],
        b'0' => ["###", "# #", "# #", "# #", "###"],
        b'1' => ["  #", "  #", "  #", "  #", "  #"],
        b'2' => ["###", "  #", "###", "#  ", "###"],
        b'3' => ["###", "  #", "###", "  #", "###"],
        b'4' => ["# #", "# #", "###", "  #", "  #"],
        b'5' => ["###", "#  ", "###", "  #", "###"],
        b'6' => ["###", "#  ", "###", "# #", "###"],
        b'7' => ["###", "  #", "  #", "  #", "  #"],
        b'8' => ["###", "# #", "###", "# #", "###"],
        b'9' => ["###", "# #", "###", "  #", "###"],
        b'/' => ["  #", "  #", " # ", "#  ", "#  "],
        b'!' => [" # ", " # ", " # ", "   ", " # "],
        b'.' => ["   ", "   ", "   ", "   ", " # "],
        b'-' => ["   ", "   ", "###", "   ", "   "],
        _ => ["   ", "   ", "   ", "   ", "   "],
    }
}

fn aabb(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

fn move_toward(v: f32, target: f32, max_delta: f32) -> f32 {
    if v < target {
        (v + max_delta).min(target)
    } else if v > target {
        (v - max_delta).max(target)
    } else {
        v
    }
}

fn wrapf(x: f32, m: f32) -> f32 {
    let mut r = x - ((x / m) as i32 as f32) * m;
    if r < 0.0 {
        r += m;
    }
    r
}

struct Game {
    state: GameState,
    player: Player,
    platforms: Vec<Rect>,
    spikes: Vec<Rect>,
    coins: Vec<Coin>,
    flag_x: f32,
    spawn: Vec2,
    coins_total: i32,
    deaths: i32,
    g_scale: f32,
    g_off_x: f32,
    g_off_y: f32,
    g_cam: f32,
    g_flash: f32,
    g_time: f32,
}

impl Game {
    fn new() -> Self {
        let mut g = Game {
            state: GameState::Start,
            player: Player::default(),
            platforms: Vec::new(),
            spikes: Vec::new(),
            coins: Vec::new(),
            flag_x: 0.0,
            spawn: vec2(32.0, HORIZON - PLAYER_H),
            coins_total: 0,
            deaths: 0,
            g_scale: 4.0,
            g_off_x: 0.0,
            g_off_y: 0.0,
            g_cam: 0.0,
            g_flash: 0.0,
            g_time: 0.0,
        };
        g.build_level();
        g.reset_game();
        g
    }

    fn build_level(&mut self) {
        let ground_h = VH - HORIZON + 40.0;
        let add_plat = |g: &mut Game, x: f32, y: f32, w: f32, h: f32| {
            g.platforms.push(Rect { x, y, w, h });
        };
        add_plat(self, 0.0, HORIZON, 360.0, ground_h);
        add_plat(self, 430.0, HORIZON, 300.0, ground_h);
        add_plat(self, 770.0, HORIZON, 520.0, ground_h);
        add_plat(self, 1340.0, HORIZON, 260.0, ground_h);
        add_plat(self, 1660.0, HORIZON, 700.0, ground_h);
        add_plat(self, 2420.0, HORIZON, 780.0, ground_h);

        add_plat(self, 250.0, 118.0, 60.0, 10.0);
        add_plat(self, 380.0, 92.0, 48.0, 10.0);
        add_plat(self, 560.0, 104.0, 70.0, 10.0);
        add_plat(self, 640.0, 72.0, 56.0, 10.0);
        add_plat(self, 980.0, 100.0, 64.0, 10.0);
        add_plat(self, 1100.0, 74.0, 64.0, 10.0);
        add_plat(self, 1480.0, 96.0, 70.0, 10.0);
        add_plat(self, 1880.0, 110.0, 60.0, 10.0);
        add_plat(self, 1990.0, 84.0, 60.0, 10.0);
        add_plat(self, 2100.0, 60.0, 60.0, 10.0);
        add_plat(self, 2540.0, 104.0, 80.0, 10.0);
        add_plat(self, 2720.0, 80.0, 80.0, 10.0);

        let add_spike = |g: &mut Game, x: f32, count: i32| {
            for i in 0..count {
                g.spikes.push(Rect {
                    x: x + i as f32 * 8.0,
                    y: HORIZON - 8.0,
                    w: 8.0,
                    h: 8.0,
                });
            }
        };
        add_spike(self, 820.0, 3);
        add_spike(self, 1180.0, 2);
        add_spike(self, 1720.0, 4);
        add_spike(self, 2520.0, 2);

        let coin_arc = |g: &mut Game, x: f32, y: f32, n: i32| {
            for i in 0..n {
                g.coins.push(Coin {
                    x: x + i as f32 * 14.0,
                    y,
                    taken: false,
                });
                g.coins_total += 1;
            }
        };
        coin_arc(self, 255.0, 100.0, 3);
        coin_arc(self, 384.0, 74.0, 3);
        coin_arc(self, 470.0, 120.0, 3);
        coin_arc(self, 645.0, 54.0, 3);
        coin_arc(self, 820.0, 120.0, 3);
        coin_arc(self, 985.0, 82.0, 4);
        coin_arc(self, 1105.0, 56.0, 3);
        coin_arc(self, 1360.0, 120.0, 3);
        coin_arc(self, 1485.0, 78.0, 4);
        coin_arc(self, 1995.0, 66.0, 3);
        coin_arc(self, 2105.0, 42.0, 4);
        coin_arc(self, 2545.0, 86.0, 4);
        coin_arc(self, 2725.0, 62.0, 4);

        self.flag_x = 3120.0;
    }

    fn reset_game(&mut self) {
        for coin in self.coins.iter_mut() {
            coin.taken = false;
        }
        self.player = Player::default();
        self.player.pos = self.spawn;
        self.player.facing = 1.0;
        self.deaths = 0;
        self.g_cam = 0.0;
        self.g_flash = 0.0;
        self.state = GameState::Start;
    }

    fn player_box(&self) -> Rect {
        Rect {
            x: self.player.pos.x,
            y: self.player.pos.y,
            w: PLAYER_W,
            h: PLAYER_H,
        }
    }

    fn die(&mut self) {
        self.deaths += 1;
        self.g_flash = 1.0;
        self.player.pos = self.spawn;
        self.player.vel = vec2(0.0, 0.0);
        self.player.on_ground = false;
    }

    fn simulate(&mut self, dt: f32, left: bool, right: bool, want_jump: bool) {
        let mut target = 0.0;
        if left {
            target -= MOVE_SPEED;
        }
        if right {
            target += MOVE_SPEED;
        }

        if target != 0.0 {
            let rate = ACCEL * dt;
            self.player.vel.x = move_toward(self.player.vel.x, target, rate);
            self.player.facing = if target > 0.0 { 1.0 } else { -1.0 };
        } else {
            self.player.vel.x = move_toward(self.player.vel.x, 0.0, FRICTION * dt);
        }

        self.player.vel.y += GRAVITY * dt;
        if self.player.vel.y > MAX_FALL {
            self.player.vel.y = MAX_FALL;
        }

        if want_jump {
            self.player.jump_buffer = 0.12;
        }
        self.player.jump_buffer -= dt;
        if self.player.on_ground {
            self.player.coyote = 0.10;
        } else {
            self.player.coyote -= dt;
        }

        if self.player.jump_buffer > 0.0 && self.player.coyote > 0.0 {
            self.player.vel.y = JUMP_VEL;
            self.player.on_ground = false;
            self.player.coyote = 0.0;
            self.player.jump_buffer = 0.0;
            self.player.squash = 1.0;
        }

        self.player.pos.x += self.player.vel.x * dt;
        {
            let mut b = self.player_box();
            for it in self.platforms.iter() {
                if aabb(b, *it) {
                    if self.player.vel.x > 0.0 {
                        self.player.pos.x = it.x - PLAYER_W;
                    } else if self.player.vel.x < 0.0 {
                        self.player.pos.x = it.x + it.w;
                    }
                    self.player.vel.x = 0.0;
                    b = self.player_box();
                }
            }
        }

        let was_air = !self.player.on_ground;
        self.player.on_ground = false;
        self.player.pos.y += self.player.vel.y * dt;
        {
            let mut b = self.player_box();
            for it in self.platforms.iter() {
                if aabb(b, *it) {
                    if self.player.vel.y > 0.0 {
                        self.player.pos.y = it.y - PLAYER_H;
                        self.player.on_ground = true;
                        if was_air {
                            self.player.squash = -1.0;
                        }
                    } else if self.player.vel.y < 0.0 {
                        self.player.pos.y = it.y + it.h;
                    }
                    self.player.vel.y = 0.0;
                    b = self.player_box();
                }
            }
        }

        if self.player.pos.x < 0.0 {
            self.player.pos.x = 0.0;
        }
        if self.player.pos.x > LEVEL_W - PLAYER_W {
            self.player.pos.x = LEVEL_W - PLAYER_W;
        }

        self.player.anim_t += dt * (self.player.vel.x.abs() * 0.06 + 1.0);
        self.player.squash = move_toward(self.player.squash, 0.0, dt * 6.0);

        let target_cam = self.player.pos.x + PLAYER_W * 0.5 - VW * 0.40;
        self.g_cam = move_toward(self.g_cam, target_cam, (target_cam - self.g_cam).abs() * dt * 8.0 + 1.0);
        if self.g_cam < 0.0 {
            self.g_cam = 0.0;
        }
        if self.g_cam > LEVEL_W - VW {
            self.g_cam = LEVEL_W - VW;
        }

        let b = self.player_box();
        for coin in self.coins.iter_mut() {
            if coin.taken {
                continue;
            }
            let cb = Rect {
                x: coin.x,
                y: coin.y,
                w: 8.0,
                h: 8.0,
            };
            if aabb(b, cb) {
                coin.taken = true;
            }
        }

        let mut died = false;
        for it in self.spikes.iter() {
            let hb = Rect {
                x: it.x + 1.0,
                y: it.y + 3.0,
                w: it.w - 2.0,
                h: it.h - 3.0,
            };
            if aabb(b, hb) {
                died = true;
            }
        }
        if self.player.pos.y > VH + 40.0 {
            died = true;
        }
        if died {
            self.die();
        }

        if self.player.pos.x + PLAYER_W > self.flag_x {
            self.state = GameState::Win;
        }

        if self.g_flash > 0.0 {
            self.g_flash -= dt * 2.0;
        }
    }

    fn rect_v(&self, px: f32, py: f32, w: f32, h: f32, color: Color) {
        let x0 = px * self.g_scale + self.g_off_x;
        let y0 = py * self.g_scale + self.g_off_y;
        draw_rectangle(x0, y0, w * self.g_scale, h * self.g_scale, color);
    }

    fn rect_w(&self, px: f32, py: f32, w: f32, h: f32, color: Color) {
        self.rect_v(px - self.g_cam, py, w, h, color);
    }

    fn draw_sprite_xy(&self, rows: &[&str], at_x: f32, at_y: f32, cw: f32, ch: f32, flip: bool) {
        let h = rows.len();
        if h == 0 {
            return;
        }
        let w = rows[0].len();
        for ry in 0..h {
            let row = rows[ry].as_bytes();
            for cx in 0..row.len() {
                let (color, vis) = sprite_color(row[cx]);
                if !vis {
                    continue;
                }
                let col = if flip { w - 1 - cx } else { cx };
                self.rect_v(at_x + col as f32 * cw, at_y + ry as f32 * ch, cw + 0.05, ch + 0.05, color);
            }
        }
    }

    fn draw_sprite_world(&self, rows: &[&str], wx: f32, wy: f32, cw: f32, ch: f32, flip: bool) {
        self.draw_sprite_xy(rows, wx - self.g_cam, wy, cw, ch, flip);
    }

    fn draw_text(&self, text: &str, x: f32, y: f32, cell: f32, color: Color) -> f32 {
        let mut cx = x;
        for ch in text.bytes() {
            let glyph = get_glyph(ch);
            for gy in 0..5 {
                let row = glyph[gy].as_bytes();
                for gx in 0..3 {
                    if row[gx] == b'#' {
                        self.rect_v(cx + gx as f32 * cell, y + gy as f32 * cell, cell + 0.05, cell + 0.05, color);
                    }
                }
            }
            cx += 4.0 * cell;
        }
        cx - x
    }

    fn center_text(&self, text: &str, y: f32, cell: f32, color: Color) {
        let width = (text.len() as f32 * 4.0 - 1.0) * cell;
        self.draw_text(text, (VW - width) * 0.5, y, cell, color);
    }

    fn dim(&self) {
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), col4(0.0, 0.0, 0.0, 0.55));
    }

    fn render(&mut self) {
        let rw = screen_width();
        let rh = screen_height();
        self.g_scale = (rw / VW).min(rh / VH);
        self.g_off_x = (rw - VW * self.g_scale) * 0.5;
        self.g_off_y = (rh - VH * self.g_scale) * 0.5;

        clear_background(SKY_TOP);

        self.draw_sky();
        self.draw_sun();
        self.draw_clouds();

        self.draw_hills(0.25, 30.0, HILL_FAR, 0.012, 0.0);
        self.draw_hills(0.45, 22.0, HILL_MID, 0.020, 130.0);
        self.draw_hills(0.70, 16.0, HILL_NEAR, 0.034, 60.0);

        self.draw_level();
        self.draw_coins();
        self.draw_flag();
        self.draw_player();

        if self.g_flash > 0.0 {
            draw_rectangle(0.0, 0.0, rw, rh, col4(1.0, 0.25, 0.25, self.g_flash * 0.5));
        }

        self.draw_hud();

        if self.state == GameState::Start {
            self.draw_start_overlay();
        }
        if self.state == GameState::Win {
            self.draw_win_overlay();
        }
    }

    fn draw_sky(&self) {
        let bands = 24;
        for i in 0..bands {
            let t = i as f32 / (bands - 1) as f32;
            let c = lerp_color(SKY_TOP, SKY_BOTTOM, t);
            let y0 = t * HORIZON;
            let y1 = ((i + 1) as f32 / (bands - 1) as f32) * HORIZON;
            self.rect_v(0.0, y0, VW, y1 - y0 + 1.0, c);
        }
    }

    fn draw_sun(&self) {
        let sx = VW - 56.0 - self.g_cam * 0.05;
        let sy = 26.0;
        self.rect_v(sx - 2.0, sy + 2.0, 20.0, 16.0, col4(1.0, 0.92, 0.55, 0.4));
        self.rect_v(sx, sy, 16.0, 16.0, col4(1.0, 0.95, 0.70, 1.0));
        self.rect_v(sx + 3.0, sy + 3.0, 8.0, 8.0, col4(1.0, 1.0, 0.88, 1.0));
    }

    fn draw_clouds(&self) {
        let cloud = |g: &Game, base_x: f32, y: f32, scale: f32| {
            let x = wrapf(base_x - g.g_cam * 0.4, VW + 80.0) - 40.0;
            let w = col4(1.0, 1.0, 1.0, 0.9);
            g.rect_v(x, y, 22.0 * scale, 8.0 * scale, w);
            g.rect_v(x + 8.0 * scale, y - 5.0 * scale, 16.0 * scale, 9.0 * scale, w);
            g.rect_v(x + 18.0 * scale, y, 16.0 * scale, 7.0 * scale, w);
        };
        cloud(self, 40.0, 28.0, 1.0);
        cloud(self, 180.0, 44.0, 0.8);
        cloud(self, 300.0, 22.0, 1.2);
        cloud(self, 120.0, 60.0, 0.7);
    }

    fn draw_hills(&self, parallax: f32, amp: f32, color: Color, freq: f32, phase: f32) {
        let step = 3.0;
        let mut x = 0.0;
        while x < VW + step {
            let wx = self.g_cam * parallax + x + phase;
            let mut v = 0.5 + 0.32 * (wx * freq).sin() + 0.18 * (wx * freq * 2.3 + 1.7).sin();
            if v < 0.0 {
                v = 0.0;
            }
            if v > 1.0 {
                v = 1.0;
            }
            let surf = HORIZON - amp * v;
            self.rect_v(x, surf, step + 0.5, VH - surf, color);
            x += step;
        }
    }

    fn draw_level(&self) {
        for it in self.platforms.iter() {
            if it.x + it.w < self.g_cam || it.x > self.g_cam + VW {
                continue;
            }
            self.rect_w(it.x, it.y + 5.0, it.w, it.h - 5.0, DIRT);
            self.rect_w(it.x, it.y + it.h - 4.0, it.w, 4.0, DIRT_DK);
            self.rect_w(it.x, it.y, it.w, 5.0, GRASS);
            self.rect_w(it.x, it.y, it.w, 1.0, GRASS_HI);

            let mut bx = it.x + 2.0;
            while bx < it.x + it.w - 2.0 {
                self.rect_w(bx, it.y - 1.0, 1.0, 1.0, GRASS_HI);
                bx += 7.0;
            }
        }
        for it in self.spikes.iter() {
            self.draw_sprite_world(&SPIKE, it.x, it.y, 1.0, 1.0, false);
        }
    }

    fn draw_coins(&self) {
        for coin in self.coins.iter() {
            if coin.taken {
                continue;
            }
            let bob = (self.g_time * 4.0 + coin.x * 0.2).sin() * 1.5;
            self.draw_sprite_world(&COIN, coin.x, coin.y + bob, 1.0, 1.0, false);
        }
    }

    fn draw_flag(&self) {
        let fx = self.flag_x;
        self.rect_w(fx, HORIZON - 48.0, 3.0, 48.0, col4(0.85, 0.85, 0.9, 1.0));
        self.rect_w(fx, HORIZON - 48.0, 3.0, 2.0, col4(0.55, 0.55, 0.6, 1.0));

        for i in 0..8 {
            let w = 18.0 - i as f32 * 2.0;
            self.rect_w(fx + 3.0, HORIZON - 47.0 + i as f32 * 2.0, w, 2.0, col4(0.92, 0.26, 0.30, 1.0));
        }
        self.rect_w(fx - 2.0, HORIZON - 2.0, 7.0, 2.0, col4(0.3, 0.3, 0.34, 1.0));
    }

    fn draw_player(&self) {
        let sw = 14.0;
        let sh = 11.0;

        let mut sx = 1.0 - self.player.squash * 0.18;
        let mut sy = 1.0 + self.player.squash * 0.22;

        if self.player.on_ground && self.player.vel.x.abs() > 6.0 {
            let b = (self.player.anim_t * 8.0).sin();
            sy += 0.06 * b;
            sx -= 0.06 * b;
        } else if self.player.on_ground {
            sy += 0.03 * (self.g_time * 3.0).sin();
        }

        let cw = sx;
        let ch = sy;
        let draw_w = sw * cw;
        let draw_h = sh * ch;

        let cx = self.player.pos.x + PLAYER_W * 0.5;
        let base_x = cx - draw_w * 0.5 - self.g_cam;
        let base_y = self.player.pos.y + PLAYER_H - draw_h;

        let flip = self.player.facing < 0.0;
        self.draw_sprite_xy(&SLIME, base_x, base_y, cw, ch, flip);

        self.rect_w(self.player.pos.x + 1.0, HORIZON - 2.0, PLAYER_W - 2.0, 2.0, col4(0.0, 0.0, 0.0, 0.15));
    }

    fn draw_hud(&self) {
        let mut got = 0;
        for coin in self.coins.iter() {
            if coin.taken {
                got += 1;
            }
        }

        self.draw_sprite_xy(&COIN, 6.0, 6.0, 1.0, 1.0, false);
        self.draw_text(&format!("{} / {}", got, self.coins_total), 18.0, 7.0, 2.0, col4(1.0, 1.0, 1.0, 1.0));
        self.draw_text(&format!("DEATHS {}", self.deaths), 18.0, 20.0, 1.0, col4(1.0, 0.85, 0.5, 1.0));
    }

    fn draw_start_overlay(&self) {
        self.dim();
        self.center_text("PIXEL SLIME", 56.0, 5.0, col4(1.0, 0.95, 0.4, 1.0));
        self.center_text("ARROWS OR A D TO MOVE", 92.0, 1.0, col4(1.0, 1.0, 1.0, 1.0));
        self.center_text("SPACE OR UP TO JUMP", 104.0, 1.0, col4(1.0, 1.0, 1.0, 1.0));
        self.center_text("COLLECT COINS REACH THE FLAG", 116.0, 1.0, col4(0.8, 0.95, 0.8, 1.0));
        let blink = (self.g_time * 4.0).sin() > 0.0;
        if blink {
            self.center_text("PRESS SPACE TO START", 138.0, 2.0, col4(1.0, 0.8, 0.3, 1.0));
        }
    }

    fn draw_win_overlay(&self) {
        self.dim();
        let mut got = 0;
        for coin in self.coins.iter() {
            if coin.taken {
                got += 1;
            }
        }
        self.center_text("YOU WIN", 54.0, 6.0, col4(0.5, 1.0, 0.5, 1.0));
        self.center_text(&format!("COINS {} OF {}", got, self.coins_total), 92.0, 2.0, col4(1.0, 0.95, 0.4, 1.0));
        self.center_text(&format!("DEATHS {}", self.deaths), 108.0, 2.0, col4(1.0, 1.0, 1.0, 1.0));
        self.center_text("PRESS R TO PLAY AGAIN", 134.0, 2.0, col4(1.0, 0.8, 0.3, 1.0));
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(a.r + (b.r - a.r) * t, a.g + (b.g - a.g) * t, a.b + (b.b - a.b) * t, a.a + (b.a - a.a) * t)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pixel Slime".to_owned(),
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

        let dt = get_frame_time().min(0.033);
        game.g_time += dt;

        let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);

        let mut want_jump = false;
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Space) {
            if game.state == GameState::Start {
                game.state = GameState::Play;
            }
            want_jump = true;
        }
        if is_key_pressed(KeyCode::W) {
            want_jump = true;
        }
        if is_key_pressed(KeyCode::R) {
            game.reset_game();
        }

        if game.state == GameState::Play {
            game.simulate(dt, left, right, want_jump);
        }

        game.render();

        next_frame().await;
    }
}
