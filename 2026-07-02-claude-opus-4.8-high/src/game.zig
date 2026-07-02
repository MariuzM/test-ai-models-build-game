const std = @import("std");

const Draw = @import("draw.zig").Draw;
const lvl = @import("level.zig");
const T = lvl.TILE;
const sdl = @import("sdl.zig");
const Color = sdl.Color;
const spr = @import("sprites.zig");

pub const VW: f32 = 320;
pub const VH: f32 = 180;
const GRAVITY: f32 = 720;
const FALL_MULT: f32 = 1.35;
const JUMP_V: f32 = 272;
const RUN_SPEED: f32 = 96;
const ACCEL_GROUND: f32 = 900;
const ACCEL_AIR: f32 = 560;
const FRICTION: f32 = 1000;
const MAX_FALL: f32 = 380;
const COYOTE: f32 = 0.09;
const JUMP_BUFFER: f32 = 0.10;
const STOMP_BOUNCE: f32 = 215;
const INVULN_TIME: f32 = 1.2;
const BEETLE_SPEED: f32 = 26;
const MAX_HP: i32 = 3;

pub const State = enum { title, playing, win, gameover };

pub const Input = struct {
    left: bool = false,
    right: bool = false,
    jump_held: bool = false,
    jump_pressed: bool = false,
    start_pressed: bool = false,
    restart_pressed: bool = false,
};

const Player = struct {
    x: f32,
    y: f32,
    w: f32 = 9,
    h: f32 = 13,
    vx: f32 = 0,
    vy: f32 = 0,
    on_ground: bool = false,
    facing: f32 = 1,
    coyote: f32 = 0,
    buffer: f32 = 0,
    invuln: f32 = 0,
    hp: i32 = MAX_HP,
    anim: f32 = 0,
    squash: f32 = 0,
};

const Beetle = struct {
    x: f32,
    y: f32,
    w: f32 = 11,
    h: f32 = 7,
    dir: f32 = -1,
    alive: bool = true,
    anim: f32 = 0,
};

const Gem = struct {
    x: f32,
    y: f32,
    taken: bool = false,
    phase: f32,
};

const Particle = struct {
    x: f32 = 0,
    y: f32 = 0,
    vx: f32 = 0,
    vy: f32 = 0,
    life: f32 = 0,
    max_life: f32 = 1,
    size: f32 = 1,
    grav: f32 = 0,
    col: Color = Color.rgb(255, 255, 255),
};

const MAX_BEETLES = 32;
const MAX_GEMS = 64;
const MAX_PARTICLES = 256;

pub const Game = struct {
    state: State = .title,
    level: lvl.Level,
    player: Player,
    start_x: f32,
    start_y: f32,
    checkpoint_x: f32,
    checkpoint_y: f32,

    beetles: [MAX_BEETLES]Beetle = undefined,
    n_beetles: usize = 0,
    gems: [MAX_GEMS]Gem = undefined,
    n_gems: usize = 0,
    particles: [MAX_PARTICLES]Particle = undefined,

    flag_x: f32 = 0,
    flag_y: f32 = 0,

    gems_taken: u32 = 0,
    gems_total: u32 = 0,
    time: f32 = 0,
    win_time: f32 = 0,
    time_c: f32 = 0,
    cam_x: f32 = 0,
    cam_y: f32 = 0,

    prng: std.Random.DefaultPrng,

    pub fn init(seed: u64) Game {
        var g: Game = .{
            .level = lvl.build(),
            .player = undefined,
            .start_x = 0,
            .start_y = 0,
            .checkpoint_x = 0,
            .checkpoint_y = 0,
            .prng = std.Random.DefaultPrng.init(seed),
        };
        g.loadEntities();
        g.player = .{ .x = g.start_x, .y = g.start_y };
        g.checkpoint_x = g.start_x;
        g.checkpoint_y = g.start_y;
        g.cam_x = std.math.clamp(g.start_x - VW / 2, 0, @max(0, g.level.widthPx() - VW));
        g.cam_y = std.math.clamp(g.start_y - VH * 0.6, 0, @max(0, @as(f32, @floatFromInt(lvl.ROWS)) * T - VH));
        return g;
    }

    fn loadEntities(g: *Game) void {
        g.n_beetles = 0;
        g.n_gems = 0;
        var row: usize = 0;
        while (row < lvl.ROWS) : (row += 1) {
            var col: usize = 0;
            while (col < g.level.cols) : (col += 1) {
                const ch = g.level.tiles[row][col];
                const px = @as(f32, @floatFromInt(col)) * T;
                const py = @as(f32, @floatFromInt(row)) * T;
                switch (ch) {
                    'P' => {
                        g.start_x = px + 3;
                        g.start_y = py + 3;
                        g.level.tiles[row][col] = '.';
                    },
                    'o' => {
                        if (g.n_gems < MAX_GEMS) {
                            g.gems[g.n_gems] = .{ .x = px + 3.5, .y = py + 3.5, .phase = @as(f32, @floatFromInt(col)) * 0.7 };
                            g.n_gems += 1;
                        }
                        g.level.tiles[row][col] = '.';
                    },
                    'b' => {
                        if (g.n_beetles < MAX_BEETLES) {
                            g.beetles[g.n_beetles] = .{ .x = px + 2.5, .y = py + T - 7 };
                            g.n_beetles += 1;
                        }
                        g.level.tiles[row][col] = '.';
                    },
                    'F' => {
                        g.flag_x = px;
                        g.flag_y = py;
                        g.level.tiles[row][col] = '.';
                    },
                    else => {},
                }
            }
        }
        g.gems_total = @intCast(g.n_gems);
    }

    pub fn reset(g: *Game) void {
        const seed = g.prng.next();
        g.* = Game.init(seed);
        g.state = .playing;
    }

    fn solidAt(g: *Game, wx: f32, wy: f32) bool {
        const col: i32 = @intFromFloat(@floor(wx / T));
        const row: i32 = @intFromFloat(@floor(wy / T));
        return g.level.solid(col, row);
    }

    pub fn update(g: *Game, in: Input, dt: f32) void {
        g.time_c += dt;
        switch (g.state) {
            .title => {
                if (in.start_pressed) {
                    g.reset();
                }
            },
            .playing => g.updatePlaying(in, dt),
            .win, .gameover => {
                if (in.restart_pressed or in.start_pressed) g.reset();
            },
        }
        g.updateParticles(dt);
    }

    fn updatePlaying(g: *Game, in: Input, dt: f32) void {
        g.time += dt;
        const p = &g.player;
        p.anim += dt;

        if (p.invuln > 0) p.invuln -= dt;
        if (p.buffer > 0) p.buffer -= dt;
        if (p.coyote > 0) p.coyote -= dt;
        if (in.jump_pressed) p.buffer = JUMP_BUFFER;

        var target: f32 = 0;
        if (in.left) target -= RUN_SPEED;
        if (in.right) target += RUN_SPEED;
        const accel: f32 = if (p.on_ground) ACCEL_GROUND else ACCEL_AIR;
        if (target != 0) {
            p.facing = if (target > 0) 1 else -1;
            if (p.vx < target) p.vx = @min(p.vx + accel * dt, target);
            if (p.vx > target) p.vx = @max(p.vx - accel * dt, target);
            if (p.on_ground and @mod(p.anim, 0.12) < dt and @abs(p.vx) > 40) {
                g.spawnDust(p.x + p.w / 2, p.y + p.h, 1, Color.rgb(220, 180, 130));
            }
        } else {
            if (p.vx > 0) p.vx = @max(0, p.vx - FRICTION * dt);
            if (p.vx < 0) p.vx = @min(0, p.vx + FRICTION * dt);
        }

        g.moveX(p, p.vx * dt);

        if (p.buffer > 0 and (p.on_ground or p.coyote > 0)) {
            p.vy = -JUMP_V;
            p.on_ground = false;
            p.coyote = 0;
            p.buffer = 0;
            p.squash = -1;
            g.spawnDust(p.x + p.w / 2, p.y + p.h, 4, Color.rgb(230, 200, 150));
        }
        if (!in.jump_held and p.vy < -60) p.vy = -60;

        const g_mult: f32 = if (p.vy > 0) FALL_MULT else 1.0;
        p.vy += GRAVITY * g_mult * dt;
        if (p.vy > MAX_FALL) p.vy = MAX_FALL;

        const was_air = !p.on_ground;
        g.moveY(p, p.vy * dt);
        if (p.on_ground and was_air and p.vy >= 0) {
            p.squash = 1;
            if (p.vy > 120) g.spawnDust(p.x + p.w / 2, p.y + p.h, 5, Color.rgb(220, 180, 130));
        }

        if (p.squash > 0) p.squash = @max(0, p.squash - dt * 6);
        if (p.squash < 0) p.squash = @min(0, p.squash + dt * 6);

        if (p.on_ground) {
            const below_col: i32 = @intFromFloat(@floor((p.x + p.w / 2) / T));
            const surf_row: i32 = @intFromFloat(@floor((p.y + p.h) / T));
            const above = g.level.get(below_col, surf_row - 1);
            if (above != '^') {
                g.checkpoint_x = p.x;
                g.checkpoint_y = p.y;
            }
        }

        if (p.invuln <= 0 and g.touchesTile(p.x, p.y, p.w, p.h, '^')) {
            g.hurt(-1);
        }

        if (p.y > (@as(f32, @floatFromInt(lvl.ROWS)) * T) + 40) {
            g.player.hp -= 1;
            if (g.player.hp <= 0) {
                g.state = .gameover;
            } else {
                g.respawn();
            }
        }

        g.updateBeetles(dt);
        g.checkGems();
        g.checkFlag();
        g.updateCamera(dt);
    }

    fn updateCamera(g: *Game, dt: f32) void {
        const p = &g.player;
        var tx = p.x + p.w / 2 - VW / 2;
        var ty = p.y + p.h / 2 - VH * 0.60;
        const max_x = @max(0, g.level.widthPx() - VW);
        const max_y = @max(0, @as(f32, @floatFromInt(lvl.ROWS)) * T - VH);
        tx = std.math.clamp(tx, 0, max_x);
        ty = std.math.clamp(ty, 0, max_y);
        const s = @min(1.0, dt * 8.0);
        g.cam_x += (tx - g.cam_x) * s;
        g.cam_y += (ty - g.cam_y) * s;
    }

    fn respawn(g: *Game) void {
        const p = &g.player;
        p.x = g.checkpoint_x;
        p.y = g.checkpoint_y;
        p.vx = 0;
        p.vy = 0;
        p.invuln = INVULN_TIME;
    }

    fn hurt(g: *Game, knock_dir: f32) void {
        const p = &g.player;
        if (p.invuln > 0) return;
        p.hp -= 1;
        p.invuln = INVULN_TIME;
        p.vy = -170;
        p.vx = knock_dir * 120;
        for (0..10) |_| {
            g.spawnParticle(p.x + p.w / 2, p.y + p.h / 2, Color.rgb(234, 74, 74), 60);
        }
        if (p.hp <= 0) g.state = .gameover;
    }

    fn moveX(g: *Game, p: *Player, dx: f32) void {
        p.x += dx;
        if (dx == 0) return;
        const r0: i32 = @intFromFloat(@floor(p.y / T));
        const r1: i32 = @intFromFloat(@floor((p.y + p.h - 0.01) / T));
        if (dx > 0) {
            const col: i32 = @intFromFloat(@floor((p.x + p.w - 0.01) / T));
            var r = r0;
            while (r <= r1) : (r += 1) {
                if (g.level.solid(col, r)) {
                    p.x = @as(f32, @floatFromInt(col)) * T - p.w;
                    p.vx = 0;
                    break;
                }
            }
        } else {
            const col: i32 = @intFromFloat(@floor(p.x / T));
            var r = r0;
            while (r <= r1) : (r += 1) {
                if (g.level.solid(col, r)) {
                    p.x = @as(f32, @floatFromInt(col + 1)) * T;
                    p.vx = 0;
                    break;
                }
            }
        }
    }

    fn moveY(g: *Game, p: *Player, dy: f32) void {
        p.y += dy;
        p.on_ground = false;
        if (dy == 0) return;
        const c0: i32 = @intFromFloat(@floor(p.x / T));
        const c1: i32 = @intFromFloat(@floor((p.x + p.w - 0.01) / T));
        if (dy > 0) {
            const row: i32 = @intFromFloat(@floor((p.y + p.h - 0.01) / T));
            var col = c0;
            while (col <= c1) : (col += 1) {
                if (g.level.solid(col, row)) {
                    p.y = @as(f32, @floatFromInt(row)) * T - p.h;
                    p.vy = 0;
                    p.on_ground = true;
                    p.coyote = COYOTE;
                    break;
                }
            }
        } else {
            const row: i32 = @intFromFloat(@floor(p.y / T));
            var col = c0;
            while (col <= c1) : (col += 1) {
                if (g.level.solid(col, row)) {
                    p.y = @as(f32, @floatFromInt(row + 1)) * T;
                    p.vy = 0;
                    break;
                }
            }
        }
    }

    fn touchesTile(g: *Game, x: f32, y: f32, w: f32, h: f32, ch: u8) bool {
        const c0: i32 = @intFromFloat(@floor(x / T));
        const c1: i32 = @intFromFloat(@floor((x + w - 0.01) / T));
        const r0: i32 = @intFromFloat(@floor(y / T));
        const r1: i32 = @intFromFloat(@floor((y + h - 0.01) / T));
        var r = r0;
        while (r <= r1) : (r += 1) {
            var col = c0;
            while (col <= c1) : (col += 1) {
                if (g.level.get(col, r) == ch) return true;
            }
        }
        return false;
    }

    fn updateBeetles(g: *Game, dt: f32) void {
        var i: usize = 0;
        while (i < g.n_beetles) : (i += 1) {
            const b = &g.beetles[i];
            if (!b.alive) continue;
            b.anim += dt;
            b.x += b.dir * BEETLE_SPEED * dt;

            const ahead_x = if (b.dir > 0) b.x + b.w + 1 else b.x - 1;
            const foot_y = b.y + b.h + 2;
            const wall = g.solidAt(ahead_x, b.y + b.h - 2);
            const ledge = !g.solidAt(ahead_x, foot_y);
            if (wall or ledge) {
                b.dir = -b.dir;
                b.x += b.dir * 2;
            }

            const p = &g.player;
            if (overlap(p.x, p.y, p.w, p.h, b.x, b.y, b.w, b.h)) {
                const stomping = p.vy > 30 and (p.y + p.h) < (b.y + b.h * 0.6);
                if (stomping) {
                    b.alive = false;
                    p.vy = -STOMP_BOUNCE;
                    p.buffer = 0;
                    for (0..12) |_| {
                        g.spawnParticle(b.x + b.w / 2, b.y + b.h / 2, Color.rgb(150, 102, 176), 70);
                    }
                } else if (p.invuln <= 0) {
                    const dir: f32 = if (p.x < b.x) -1 else 1;
                    g.hurt(dir);
                }
            }
        }
    }

    fn checkGems(g: *Game) void {
        const p = &g.player;
        var i: usize = 0;
        while (i < g.n_gems) : (i += 1) {
            const gem = &g.gems[i];
            if (gem.taken) continue;
            if (overlap(p.x, p.y, p.w, p.h, gem.x, gem.y, 9, 9)) {
                gem.taken = true;
                g.gems_taken += 1;
                for (0..10) |_| {
                    g.spawnParticle(gem.x + 4, gem.y + 4, Color.rgb(255, 214, 92), 80);
                }
            }
        }
    }

    fn checkFlag(g: *Game) void {
        const p = &g.player;
        if (overlap(p.x, p.y, p.w, p.h, g.flag_x + 4, g.flag_y - 16, 8, 32)) {
            g.state = .win;
            g.win_time = g.time;
        }
    }

    fn spawnParticle(g: *Game, x: f32, y: f32, col: Color, speed: f32) void {
        const rnd = g.prng.random();
        const ang = rnd.float(f32) * std.math.tau;
        const spd = speed * (0.3 + rnd.float(f32) * 0.7);
        g.addParticle(.{
            .x = x,
            .y = y,
            .vx = @cos(ang) * spd,
            .vy = @sin(ang) * spd - 20,
            .life = 0.4 + rnd.float(f32) * 0.4,
            .max_life = 0.8,
            .size = 1 + rnd.float(f32) * 1.5,
            .grav = 200,
            .col = col,
        });
    }

    fn spawnDust(g: *Game, x: f32, y: f32, n: usize, col: Color) void {
        const rnd = g.prng.random();
        for (0..n) |_| {
            g.addParticle(.{
                .x = x + (rnd.float(f32) - 0.5) * 6,
                .y = y - 1,
                .vx = (rnd.float(f32) - 0.5) * 40,
                .vy = -rnd.float(f32) * 30,
                .life = 0.3 + rnd.float(f32) * 0.3,
                .max_life = 0.6,
                .size = 1 + rnd.float(f32) * 1.5,
                .grav = 60,
                .col = col,
            });
        }
    }

    fn addParticle(g: *Game, part: Particle) void {
        for (&g.particles) |*slot| {
            if (slot.life <= 0) {
                slot.* = part;
                return;
            }
        }
    }

    fn updateParticles(g: *Game, dt: f32) void {
        for (&g.particles) |*part| {
            if (part.life <= 0) continue;
            part.life -= dt;
            part.vy += part.grav * dt;
            part.x += part.vx * dt;
            part.y += part.vy * dt;
        }
    }

    pub fn render(g: *Game, d: *Draw) void {
        d.cam_x = g.cam_x;
        d.cam_y = g.cam_y;
        g.drawBackground(d);
        g.drawTiles(d);
        g.drawFlag(d);
        g.drawGems(d);
        g.drawBeetles(d);
        g.drawPlayer(d);
        g.drawParticles(d);
        g.drawHud(d);
        g.drawOverlay(d);
    }

    fn fillCircle(_: *Game, d: *Draw, cx: f32, cy: f32, r: f32, col: Color) void {
        var dy: f32 = -r;
        while (dy <= r) : (dy += 1) {
            const half = @sqrt(@max(0, r * r - dy * dy));
            d.rect(cx - half, cy + dy, half * 2, 1, col);
        }
    }

    fn drawBackground(g: *Game, d: *Draw) void {
        const top = Color.rgb(44, 32, 78);
        const mid = Color.rgb(158, 78, 112);
        const bot = Color.rgb(252, 156, 92);
        var y: f32 = 0;
        while (y < VH) : (y += 1) {
            const t = y / VH;
            const col = if (t < 0.62) top.lerp(mid, t / 0.62) else mid.lerp(bot, (t - 0.62) / 0.38);
            d.rect(0, y, VW, 1, col);
        }
        const sx = VW * 0.70 - g.cam_x * 0.03;
        g.fillCircle(d, sx, 68, 23, Color.rgb(255, 222, 150));
        g.fillCircle(d, sx, 68, 17, Color.rgb(255, 242, 206));
        g.drawClouds(d);
        g.drawDunes(d, 0.20, 120, 12, 0.016, Color.rgb(92, 62, 116), 0);
        g.drawDunes(d, 0.42, 136, 18, 0.021, Color.rgb(126, 74, 106), 90);
    }

    fn drawClouds(_: *Game, d: *Draw) void {
        const cc = Color{ .r = 255, .g = 226, .b = 210, .a = 150 };
        const pos = [_][2]f32{ .{ 40, 30 }, .{ 200, 22 }, .{ 360, 42 }, .{ 520, 26 }, .{ 660, 36 } };
        for (pos) |cpos| {
            const x = @mod(cpos[0] - d.cam_x * 0.1, 760) - 60;
            const y = cpos[1];
            d.rect(x, y, 26, 6, cc);
            d.rect(x + 6, y - 4, 16, 8, cc);
            d.rect(x + 14, y, 22, 6, cc);
        }
    }

    fn drawDunes(_: *Game, d: *Draw, factor: f32, horizon: f32, amp: f32, freq: f32, col: Color, phase: f32) void {
        var x: f32 = 0;
        while (x < VW) : (x += 2) {
            const wx = x + d.cam_x * factor + phase;
            const h = horizon - amp * (0.5 + 0.5 * @sin(wx * freq));
            d.rect(x, h, 2, VH - h, col);
        }
    }

    fn drawTiles(g: *Game, d: *Draw) void {
        const col0: i32 = @as(i32, @intFromFloat(@floor(g.cam_x / T))) - 1;
        const col1: i32 = @as(i32, @intFromFloat(@floor((g.cam_x + VW) / T))) + 1;
        const sand = Color.rgb(226, 176, 116);
        const sand_lo = Color.rgb(196, 140, 90);
        const sand_hi = Color.rgb(242, 204, 150);
        const dirt = Color.rgb(172, 120, 76);
        var col = col0;
        while (col <= col1) : (col += 1) {
            var row: i32 = 0;
            while (row < lvl.ROWS) : (row += 1) {
                const ch = g.level.get(col, row);
                const x = @as(f32, @floatFromInt(col)) * T;
                const y = @as(f32, @floatFromInt(row)) * T;
                if (ch == '#') {
                    const surface = !g.level.solid(col, row - 1);
                    d.wrect(x, y, T, T, sand);
                    d.wrect(x, y + T - 3, T, 3, sand_lo);
                    if (surface) {
                        d.wrect(x, y, T, 3, sand_hi);
                        d.wrect(x + 3, y - 2, 2, 2, Color.rgb(146, 178, 92));
                        d.wrect(x + 10, y - 1, 2, 1, Color.rgb(146, 178, 92));
                    } else {
                        d.wrect(x + 4, y + 5, 2, 2, dirt);
                        d.wrect(x + 11, y + 9, 2, 2, dirt);
                    }
                } else if (ch == '^') {
                    g.drawSpikes(d, x, y);
                }
            }
        }
    }

    fn drawSpikes(_: *Game, d: *Draw, x: f32, y: f32) void {
        const base = Color.rgb(118, 118, 138);
        const tip = Color.rgb(198, 202, 216);
        var k: f32 = 0;
        while (k < 3) : (k += 1) {
            const bx = x + k * 5 + 1;
            var i: f32 = 0;
            while (i < T) : (i += 1) {
                const frac = 1 - i / T;
                const w = 4 * frac;
                d.wrect(bx + (4 - w) / 2, y + i, w, 1, base.lerp(tip, frac));
            }
        }
    }

    fn drawFlag(g: *Game, d: *Draw) void {
        const base_y = g.flag_y + T;
        const top_y = base_y - 42;
        const px = g.flag_x + 7;
        d.wrect(px - 3, base_y - 2, 8, 2, Color.rgb(150, 110, 74));
        d.wrect(px, top_y, 2, 42, Color.rgb(122, 96, 68));
        const sway = @sin(g.time_c * 3) * 1.5;
        var i: f32 = 0;
        while (i < 12) : (i += 1) {
            const w = 15 - @abs(i - 5.5);
            const col = if (@mod(i, 2) == 0) Color.rgb(236, 88, 84) else Color.rgb(212, 60, 66);
            d.wrect(px + 2, top_y + 1 + i, w + sway, 1, col);
        }
        d.wrect(px - 1, top_y - 3, 4, 4, Color.rgb(255, 214, 92));
    }

    fn drawGems(g: *Game, d: *Draw) void {
        for (g.gems[0..g.n_gems]) |gem| {
            if (gem.taken) continue;
            const bob = @sin(g.time_c * 3 + gem.phase) * 1.5;
            d.blit(spr.gem, gem.x, gem.y + bob, false);
        }
    }

    fn drawBeetles(g: *Game, d: *Draw) void {
        for (g.beetles[0..g.n_beetles]) |b| {
            if (!b.alive) continue;
            const bob = @sin(b.anim * 12) * 0.5;
            d.blit(spr.beetle, b.x, b.y + bob, b.dir < 0);
        }
    }

    fn drawPlayer(g: *Game, d: *Draw) void {
        const p = &g.player;
        if (p.invuln > 0 and @mod(g.time_c * 20, 2) < 1) return;
        const flip = p.facing < 0;
        const sx = p.x + (p.w - 13) / 2;
        const sy = p.y + p.h - 11;
        var legs = spr.legs_idle;
        if (!p.on_ground) {
            legs = spr.legs_jump;
        } else if (@abs(p.vx) > 12) {
            legs = if (@mod(p.anim * 10, 2) < 1) spr.legs_run1 else spr.legs_run2;
        }
        d.blit(spr.fox_body, sx, sy, flip);
        d.blit(legs, sx, sy + 9, flip);
    }

    fn drawParticles(g: *Game, d: *Draw) void {
        for (g.particles) |part| {
            if (part.life <= 0) continue;
            const a = std.math.clamp(part.life / part.max_life, 0, 1);
            var col = part.col;
            col.a = @intFromFloat(a * 255);
            const s = part.size * (0.4 + a * 0.6);
            d.wrect(part.x - s / 2, part.y - s / 2, s, s, col);
        }
    }

    fn drawHud(g: *Game, d: *Draw) void {
        if (g.state == .title) return;
        var i: i32 = 0;
        while (i < MAX_HP) : (i += 1) {
            const x = 6 + @as(f32, @floatFromInt(i)) * 9;
            const filled = i < g.player.hp;
            d.blitScreen(spr.heart, x, 6, 1, if (filled) null else Color.rgb(72, 52, 62));
        }
        var buf: [16]u8 = undefined;
        const gs = std.fmt.bufPrint(&buf, "{d}/{d}", .{ g.gems_taken, g.gems_total }) catch "0";
        d.blitScreen(spr.gem, VW - 62, 4, 1, null);
        _ = d.text(gs, VW - 50, 7, 2, Color.rgb(255, 236, 180));
        var tbuf: [16]u8 = undefined;
        const whole: u32 = @intFromFloat(g.time);
        const tenth: u32 = @intFromFloat(@mod(g.time * 10, 10));
        const ts = std.fmt.bufPrint(&tbuf, "{d}.{d}", .{ whole, tenth }) catch "0";
        d.textCentered(ts, VW / 2, 6, 2, Color.rgb(240, 240, 250));
    }

    fn drawOverlay(g: *Game, d: *Draw) void {
        switch (g.state) {
            .title => {
                d.rect(0, 0, VW, VH, .{ .r = 22, .g = 14, .b = 32, .a = 140 });
                d.textCentered("FENNEC DASH", VW / 2, 48, 5, Color.rgb(255, 198, 96));
                d.textCentered("A DESERT DASH PLATFORMER", VW / 2, 86, 1, Color.rgb(244, 214, 184));
                if (@mod(g.time_c, 1.0) < 0.62)
                    d.textCentered("PRESS SPACE TO START", VW / 2, 112, 2, Color.rgb(255, 255, 255));
                d.textCentered("ARROWS OR A D  MOVE     SPACE OR W  JUMP", VW / 2, 146, 1, Color.rgb(202, 202, 214));
                d.textCentered("STOMP BEETLES   GRAB GEMS   REACH THE FLAG", VW / 2, 158, 1, Color.rgb(202, 202, 214));
            },
            .win => {
                d.rect(0, 0, VW, VH, .{ .r = 18, .g = 30, .b = 22, .a = 175 });
                d.textCentered("YOU WIN!", VW / 2, 44, 5, Color.rgb(180, 255, 140));
                var buf: [32]u8 = undefined;
                const gs = std.fmt.bufPrint(&buf, "GEMS  {d} OF {d}", .{ g.gems_taken, g.gems_total }) catch "";
                d.textCentered(gs, VW / 2, 92, 2, Color.rgb(255, 236, 180));
                var tb: [32]u8 = undefined;
                const ts = std.fmt.bufPrint(&tb, "TIME  {d}.{d} S", .{ @as(u32, @intFromFloat(g.win_time)), @as(u32, @intFromFloat(@mod(g.win_time * 10, 10))) }) catch "";
                d.textCentered(ts, VW / 2, 108, 2, Color.rgb(230, 230, 240));
                if (@mod(g.time_c, 1.0) < 0.62)
                    d.textCentered("PRESS R TO PLAY AGAIN", VW / 2, 140, 2, Color.rgb(255, 255, 255));
            },
            .gameover => {
                d.rect(0, 0, VW, VH, .{ .r = 34, .g = 12, .b = 16, .a = 180 });
                d.textCentered("GAME OVER", VW / 2, 60, 5, Color.rgb(240, 90, 90));
                if (@mod(g.time_c, 1.0) < 0.62)
                    d.textCentered("PRESS R TO RETRY", VW / 2, 112, 2, Color.rgb(255, 255, 255));
            },
            .playing => {},
        }
    }
};

fn overlap(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) bool {
    return ax < bx + bw and ax + aw > bx and ay < by + bh and ay + ah > by;
}
