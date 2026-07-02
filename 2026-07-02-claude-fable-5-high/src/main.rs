use macroquad::prelude::*;

const TILE: f32 = 16.0;
const VIEW_H: f32 = 180.0;
const GRAV: f32 = 900.0;
const RUN_SPEED: f32 = 105.0;
const RUN_ACCEL: f32 = 820.0;
const JUMP_VEL: f32 = 288.0;
const DJUMP_VEL: f32 = 262.0;
const MAX_FALL: f32 = 320.0;
const DASH_SPEED: f32 = 250.0;
const DASH_TIME: f32 = 0.16;
const DASH_COOLDOWN: f32 = 0.3;
const COYOTE: f32 = 0.10;
const JUMP_BUFFER: f32 = 0.12;
const MAX_HEARTS: i32 = 3;

fn pal(c: char) -> Option<Color> {
    let rgb = match c {
        'o' => (232, 138, 60),
        'O' => (176, 84, 40),
        'w' => (245, 238, 220),
        'k' => (38, 28, 44),
        'r' => (206, 66, 58),
        'g' => (108, 172, 100),
        'G' => (62, 116, 84),
        'd' => (118, 82, 66),
        'D' => (84, 56, 52),
        'b' => (146, 100, 62),
        'B' => (100, 66, 46),
        's' => (138, 138, 160),
        'S' => (94, 92, 118),
        'p' => (182, 134, 210),
        'c' => (150, 236, 224),
        'y' => (252, 214, 130),
        'Y' => (240, 168, 76),
        'm' => (234, 148, 186),
        'e' => (128, 92, 168),
        'E' => (88, 58, 122),
        _ => return None,
    };
    Some(Color::from_rgba(rgb.0, rgb.1, rgb.2, 255))
}

fn sprite(rows: &[&str]) -> Texture2D {
    let w = rows.iter().map(|r| r.len()).max().unwrap_or(1);
    let h = rows.len();
    let mut img = Image::gen_image_color(w as u16, h as u16, Color::new(0.0, 0.0, 0.0, 0.0));
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if let Some(col) = pal(c) {
                img.set_pixel(x as u32, y as u32, col);
            }
        }
    }
    let t = Texture2D::from_image(&img);
    t.set_filter(FilterMode::Nearest);
    t
}

fn hash01(n: i32) -> f32 {
    let mut x = (n as u32) ^ 0x9e37_79b9;
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    (x & 0xffff) as f32 / 65535.0
}

struct Rng(u32);

impl Rng {
    fn f(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.0 >> 8) as f32 / 16777216.0
    }
    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.f()
    }
}

const FOX_IDLE: [&str; 12] = [
    "..........k...k.",
    ".ww.......ko.ok.",
    ".wok......oooo..",
    "..ooo....ooooo..",
    "...Oooooooooko..",
    "...oooooooooOww.",
    "..oooooooooooo..",
    "..owwwwwwoooo...",
    "..owwwwwooo.....",
    "...k...k...k....",
    "...k...k...k....",
    "................",
];

const FOX_RUN1: [&str; 12] = [
    "..........k...k.",
    ".ww.......ko.ok.",
    ".wok......oooo..",
    "..ooo....ooooo..",
    "...Oooooooooko..",
    "...oooooooooOww.",
    "..oooooooooooo..",
    "..owwwwwwoooo...",
    "..owwwwwooo.....",
    "..k.....k...k...",
    ".k.......k...k..",
    "................",
];

const FOX_RUN2: [&str; 12] = [
    "..........k...k.",
    ".ww.......ko.ok.",
    ".wok......oooo..",
    "..ooo....ooooo..",
    "...Oooooooooko..",
    "...oooooooooOww.",
    "..oooooooooooo..",
    "..owwwwwwoooo...",
    "..owwwwwooo.....",
    "....kk....kk....",
    "....kk....kk....",
    "................",
];

const FOX_JUMP: [&str; 12] = [
    "..........k...k.",
    "..........ko.ok.",
    ".ww.......oooo..",
    ".wooo....ooooo..",
    "..Oooooooooooko.",
    "...ooooooooooww.",
    "..oooooooooooo..",
    "..owwwwwwoooo...",
    "...wwwwwooo.....",
    "....kk...kk.....",
    "................",
    "................",
];

const FOX_FALL: [&str; 12] = [
    "..........k...k.",
    "..........ko.ok.",
    "..........oooo..",
    ".wwoo....ooooo..",
    "..Oooooooooooko.",
    "...ooooooooooww.",
    "..oooooooooooo..",
    "..owwwwwwoooo...",
    "..owwwwwooo.....",
    "...k....k...k...",
    "...k....k...k...",
    "................",
];

const BEETLE1: [&str; 9] = [
    "...kkkkkk...",
    "..keeeeeek..",
    ".keepppeeek.",
    ".keppppppek.",
    ".keeeeeeeek.",
    ".kkkkkkkkkk.",
    "..k..kk..k..",
    "..k..kk..k..",
    "............",
];

const BEETLE2: [&str; 9] = [
    "...kkkkkk...",
    "..keeeeeek..",
    ".keepppeeek.",
    ".keppppppek.",
    ".keeeeeeeek.",
    ".kkkkkkkkkk.",
    ".k...kk...k.",
    ".k...kk...k.",
    "............",
];

const MOTH1: [&str; 10] = [
    ".pp......pp.",
    "pppp....pppp",
    "pmppp..pppmp",
    ".ppppkkpppp.",
    "..pppkkppp..",
    "....kykk....",
    "....kkkk....",
    ".....kk.....",
    "............",
    "............",
];

const MOTH2: [&str; 10] = [
    "............",
    "............",
    "pppp....pppp",
    "pmpppkkpppmp",
    ".ppppkkpppp.",
    "....kykk....",
    "....kkkk....",
    ".....kk.....",
    "............",
    "............",
];

const WISP1: [&str; 8] = [
    "...c....",
    "..ccc...",
    ".ccwcc..",
    ".cwwwc..",
    ".cwwwc..",
    "..ccc...",
    "...c....",
    "........",
];

const WISP2: [&str; 8] = [
    "...c....",
    "...cc...",
    "..cwcc..",
    ".ccwwc..",
    ".cwwwc..",
    ".ccwcc..",
    "..ccc...",
    "...c....",
];

const HEART: [&str; 7] = [
    ".kk..kk.",
    "kwrkkrrk",
    "krrrrrrk",
    ".krrrrk.",
    "..krrk..",
    "...kk...",
    "........",
];

const GRASS_TILE: [&str; 16] = [
    "g.gg..g.gg...gg.",
    "gggggggggggggggg",
    "gGggggGggggGgggg",
    "GGdGGdGGGdGGGdGG",
    "dddddddddddddddd",
    "ddDddddDddddDddd",
    "dddddddddddddddd",
    "dDddDddddDdddDdd",
    "dddddddddddddddd",
    "ddddDdddddDddddd",
    "dDdddddDdddddDdd",
    "dddddddddddddddd",
    "ddDddddddDdddddd",
    "dddDddDddddDdddd",
    "Dddddddddddddddd",
    "DDdDDdDDDdDDdDDD",
];

const DIRT_TILE: [&str; 16] = [
    "dddddddddddddddd",
    "ddDddddDddddDddd",
    "dddddddddddddddd",
    "dDddDddddDdddDdd",
    "dddddddddddddddd",
    "ddddDdddddDddddd",
    "dDdddddDdddddDdd",
    "dddddddddddddddd",
    "ddDddddddDdddddd",
    "dddDddDddddDdddd",
    "dddddddddddddddd",
    "dDddddDdddddDddd",
    "dddddddddddddddd",
    "ddDdddddDddddddd",
    "Dddddddddddddddd",
    "DDdDDdDDDdDDdDDD",
];

const STONE_TILE: [&str; 16] = [
    "gsssssssSssssssS",
    "sssssssSsssssssS",
    "sssssssSsssssssS",
    "SSSSSSSSSSSSSSSS",
    "sssSsssssssSssss",
    "sssSsssssssSssss",
    "SSSSSSSSSSSSSSSS",
    "sssssssSsssssssS",
    "sssssssSsssssssS",
    "SSSSSSSSSSSSSSSS",
    "sssSsssssssSssss",
    "sssSsssssssSssss",
    "SSSSSSSSSSSSSSSS",
    "sssssssSsssssssS",
    "sssssssSsssssssS",
    "SSSSSSSSSSSSSSSS",
];

const PLANK_TILE: [&str; 5] = [
    "bbbbbbbbbbbbbbbb",
    "bkbbBbbbbBbbbbkb",
    "bbbbbbbbbbbbbbbb",
    "BbBBbbBBbbBBbbBB",
    "kBBBBBBBBBBBBBBk",
];

const SPIKE_TILE: [&str; 16] = [
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    ".w...w...w...w..",
    ".s...s...s...s..",
    ".s...s...s...s..",
    "sSs.sSs.sSs.sSs.",
    "sSs.sSs.sSs.sSs.",
    "sSSssSSssSSssSSs",
    "sSSssSSssSSssSSs",
    "SSSSSSSSSSSSSSSS",
];

const LANTERN_SPR: [&str; 14] = [
    "....kk....",
    "....kk....",
    "...kkkk...",
    "..kyyyyk..",
    ".kyywyyyk.",
    ".kyyyyyyk.",
    ".kyyyyyyk.",
    ".kYyyyyYk.",
    ".kYYyyYYk.",
    "..kYYYYk..",
    "...kkkk...",
    "....kk....",
    "....kk....",
    "...kkkk...",
];

const SEG_A: [&str; 16] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "..............*..*..*...",
    "...P....................",
    "....................E...",
    "########################",
    "########################",
    "########################",
];

const SEG_B: [&str; 16] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "......*........*..........",
    "...........E..............",
    "#####...######...#########",
    "#####...######...#########",
    "#####...######...#########",
];

const SEG_C: [&str; 16] = [
    "",
    "",
    "",
    "",
    "",
    "..................*...........",
    "...................M..........",
    ".........................*....",
    "...............*......########",
    "..............===.....########",
    ".......*..............########",
    "......===.............########",
    "...^^.....^^...^^.....########",
    "##############################",
    "##############################",
    "##############################",
];

const SEG_D: [&str; 16] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "..*.................",
    "....##..............",
    "....................",
    "........##..........",
    "..............L...E.",
    "####################",
    "####################",
    "####################",
];

const SEG_E: [&str; 16] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "......*...........*..........",
    "..............................",
    "..............M...............",
    ".....^^^.........^^^......E...",
    "##############################",
    "##############################",
    "##############################",
];

const SEG_F: [&str; 16] = [
    "",
    "",
    "",
    "",
    "",
    "................*.................",
    "..................................",
    "......................M...........",
    "..........M...*...................",
    ".............XXX.........*........",
    ".......*.................XXX......",
    "......XXX.........XXX.............",
    "..................................",
    "####..........................####",
    "####..........................####",
    "####..........................####",
];

const SEG_G: [&str; 16] = [
    "",
    "",
    "",
    ".............*............",
    "..........................",
    ".............L............",
    "............XXXX..........",
    ".........*................",
    "........XXX.......XXX.....",
    ".....*....................",
    "....XXX...................",
    "..........................",
    "......................E...",
    "##########################",
    "##########################",
    "##########################",
];

const SEG_H: [&str; 16] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "...............M..............",
    ".....*.........*.........*....",
    "..............................",
    "..............................",
    "...E.....^^....E....^^....E...",
    "##############################",
    "##############################",
    "##############################",
];

const SEG_I: [&str; 16] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "..................*.......",
    "..........................",
    "............*.............",
    "..................G.......",
    "..............############",
    "......*...................",
    "..........################",
    "##########################",
    "##########################",
    "##########################",
];

const SEGS: [&[&str]; 9] = [
    &SEG_A, &SEG_B, &SEG_C, &SEG_D, &SEG_E, &SEG_F, &SEG_G, &SEG_H, &SEG_I,
];

struct Assets {
    fox_idle: Texture2D,
    fox_run: [Texture2D; 2],
    fox_jump: Texture2D,
    fox_fall: Texture2D,
    beetle: [Texture2D; 2],
    moth: [Texture2D; 2],
    wisp: [Texture2D; 2],
    heart: Texture2D,
    grass: Texture2D,
    dirt: Texture2D,
    stone: Texture2D,
    plank: Texture2D,
    spike: Texture2D,
    lantern: Texture2D,
}

impl Assets {
    fn new() -> Self {
        Self {
            fox_idle: sprite(&FOX_IDLE),
            fox_run: [sprite(&FOX_RUN1), sprite(&FOX_RUN2)],
            fox_jump: sprite(&FOX_JUMP),
            fox_fall: sprite(&FOX_FALL),
            beetle: [sprite(&BEETLE1), sprite(&BEETLE2)],
            moth: [sprite(&MOTH1), sprite(&MOTH2)],
            wisp: [sprite(&WISP1), sprite(&WISP2)],
            heart: sprite(&HEART),
            grass: sprite(&GRASS_TILE),
            dirt: sprite(&DIRT_TILE),
            stone: sprite(&STONE_TILE),
            plank: sprite(&PLANK_TILE),
            spike: sprite(&SPIKE_TILE),
            lantern: sprite(&LANTERN_SPR),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Ground,
    Stone,
    Plank,
    Spike,
}

#[derive(Clone, Copy, PartialEq)]
enum EKind {
    Beetle,
    Moth,
}

struct Level {
    w: i32,
    h: i32,
    tiles: Vec<Tile>,
    player_spawn: Vec2,
    enemy_spawns: Vec<(EKind, Vec2)>,
    wisp_spawns: Vec<Vec2>,
    lantern_spawns: Vec<Vec2>,
    gate: Vec2,
    wisp_total: u32,
}

impl Level {
    fn parse() -> Self {
        let h = 16usize;
        let widths: Vec<usize> = SEGS
            .iter()
            .map(|s| s.iter().map(|r| r.len()).max().unwrap_or(0))
            .collect();
        let w: usize = widths.iter().sum();
        let mut lvl = Level {
            w: w as i32,
            h: h as i32,
            tiles: vec![Tile::Empty; w * h],
            player_spawn: vec2(48.0, 100.0),
            enemy_spawns: Vec::new(),
            wisp_spawns: Vec::new(),
            lantern_spawns: Vec::new(),
            gate: vec2(0.0, 0.0),
            wisp_total: 0,
        };
        for y in 0..h {
            let mut x0 = 0usize;
            for (si, seg) in SEGS.iter().enumerate() {
                for (i, c) in seg[y].chars().enumerate() {
                    let x = x0 + i;
                    let px = x as f32 * TILE;
                    let py = y as f32 * TILE;
                    match c {
                        '#' => lvl.tiles[y * w + x] = Tile::Ground,
                        'X' => lvl.tiles[y * w + x] = Tile::Stone,
                        '=' => lvl.tiles[y * w + x] = Tile::Plank,
                        '^' => lvl.tiles[y * w + x] = Tile::Spike,
                        'P' => lvl.player_spawn = vec2(px + 3.0, py + 3.0),
                        'E' => lvl.enemy_spawns.push((EKind::Beetle, vec2(px + 2.0, py + 7.0))),
                        'M' => lvl.enemy_spawns.push((EKind::Moth, vec2(px + 2.0, py + 3.0))),
                        '*' => lvl.wisp_spawns.push(vec2(px + 8.0, py + 8.0)),
                        'L' => lvl.lantern_spawns.push(vec2(px + 3.0, py + 2.0)),
                        'G' => lvl.gate = vec2(px + 8.0, py + TILE),
                        _ => {}
                    }
                }
                x0 += widths[si];
            }
        }
        lvl.wisp_total = lvl.wisp_spawns.len() as u32;
        lvl
    }

    fn tile(&self, tx: i32, ty: i32) -> Tile {
        if tx < 0 || tx >= self.w {
            return Tile::Ground;
        }
        if ty < 0 || ty >= self.h {
            return Tile::Empty;
        }
        self.tiles[(ty * self.w + tx) as usize]
    }

    fn solid(&self, tx: i32, ty: i32) -> bool {
        matches!(self.tile(tx, ty), Tile::Ground | Tile::Stone)
    }
}

fn tile_span(a: f32, len: f32) -> std::ops::RangeInclusive<i32> {
    let lo = (a / TILE).floor() as i32;
    let hi = ((a + len - 0.01) / TILE).floor() as i32;
    lo..=hi
}

fn move_actor(
    lvl: &Level,
    pos: &mut Vec2,
    size: Vec2,
    vel: &mut Vec2,
    dt: f32,
    drop: bool,
) -> (bool, bool) {
    let mut on_ground = false;
    let mut hit_wall = false;
    let steps = ((vel.length() * dt / 6.0).ceil() as i32).max(1);
    let sdt = dt / steps as f32;
    for _ in 0..steps {
        let dx = vel.x * sdt;
        pos.x += dx;
        if dx > 0.0 {
            let tx = ((pos.x + size.x) / TILE).floor() as i32;
            for ty in tile_span(pos.y, size.y) {
                if lvl.solid(tx, ty) {
                    pos.x = tx as f32 * TILE - size.x - 0.01;
                    vel.x = 0.0;
                    hit_wall = true;
                    break;
                }
            }
        } else if dx < 0.0 {
            let tx = (pos.x / TILE).floor() as i32;
            for ty in tile_span(pos.y, size.y) {
                if lvl.solid(tx, ty) {
                    pos.x = (tx + 1) as f32 * TILE + 0.01;
                    vel.x = 0.0;
                    hit_wall = true;
                    break;
                }
            }
        }
        let dy = vel.y * sdt;
        let prev_bottom = pos.y + size.y;
        pos.y += dy;
        if dy > 0.0 {
            let ty = ((pos.y + size.y) / TILE).floor() as i32;
            let top = ty as f32 * TILE;
            for tx in tile_span(pos.x, size.x) {
                let t = lvl.tile(tx, ty);
                let blocked = matches!(t, Tile::Ground | Tile::Stone)
                    || (t == Tile::Plank && !drop && prev_bottom <= top + 2.0);
                if blocked {
                    pos.y = top - size.y - 0.01;
                    vel.y = 0.0;
                    on_ground = true;
                    break;
                }
            }
        } else if dy < 0.0 {
            let ty = (pos.y / TILE).floor() as i32;
            for tx in tile_span(pos.x, size.x) {
                if lvl.solid(tx, ty) {
                    pos.y = (ty + 1) as f32 * TILE + 0.01;
                    vel.y = 0.0;
                    break;
                }
            }
        }
    }
    (on_ground, hit_wall)
}

fn overlap(ap: Vec2, asz: Vec2, bp: Vec2, bsz: Vec2) -> bool {
    ap.x < bp.x + bsz.x && ap.x + asz.x > bp.x && ap.y < bp.y + bsz.y && ap.y + asz.y > bp.y
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    max: f32,
    size: f32,
    col: Color,
    grav: f32,
}

struct Leaf {
    pos: Vec2,
    t: f32,
    col: Color,
}

struct Ghost {
    pos: Vec2,
    face: f32,
    life: f32,
}

struct Player {
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
    face: f32,
    on_ground: bool,
    coyote: f32,
    jbuf: f32,
    jumps: i32,
    dash_t: f32,
    dash_cd: f32,
    has_dash: bool,
    drop_t: f32,
    inv: f32,
    hearts: i32,
    anim: f32,
    land_t: f32,
    ghost_t: f32,
}

struct Enemy {
    kind: EKind,
    pos: Vec2,
    vel: Vec2,
    dir: f32,
    anchor: Vec2,
    t: f32,
    alive: bool,
    dying: f32,
}

struct Wisp {
    pos: Vec2,
    phase: f32,
    taken: bool,
}

struct Lantern {
    pos: Vec2,
    lit: bool,
}

struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    wisps: Vec<Wisp>,
    lanterns: Vec<Lantern>,
    particles: Vec<Particle>,
    leaves: Vec<Leaf>,
    ghosts: Vec<Ghost>,
    fireflies: Vec<(Vec2, f32)>,
    cam: Vec2,
    shake: f32,
    time: f32,
    leaf_t: f32,
    wisps_got: u32,
    checkpoint: Vec2,
    dead: bool,
    won: bool,
    rng: Rng,
}

fn burst(
    parts: &mut Vec<Particle>,
    rng: &mut Rng,
    pos: Vec2,
    n: usize,
    col: Color,
    speed: f32,
    grav: f32,
    life: f32,
) {
    for _ in 0..n {
        let a = rng.range(0.0, std::f32::consts::TAU);
        let sp = rng.range(speed * 0.3, speed);
        let max = rng.range(life * 0.5, life);
        parts.push(Particle {
            pos,
            vel: vec2(a.cos() * sp, a.sin() * sp),
            life: max,
            max,
            size: rng.range(1.0, 2.5),
            col,
            grav,
        });
    }
}

impl Game {
    fn new(lvl: &Level) -> Self {
        let mut fireflies = Vec::new();
        let count = (lvl.w as f32 * TILE / 48.0) as i32;
        for i in 0..count {
            let x = i as f32 * 48.0 + hash01(i * 3 + 1) * 40.0;
            let y = 26.0 + hash01(i * 5 + 2) * 150.0;
            fireflies.push((vec2(x, y), hash01(i * 7 + 3) * 10.0));
        }
        Game {
            player: Player {
                pos: lvl.player_spawn,
                vel: vec2(0.0, 0.0),
                size: vec2(10.0, 12.0),
                face: 1.0,
                on_ground: false,
                coyote: 0.0,
                jbuf: 0.0,
                jumps: 1,
                dash_t: 0.0,
                dash_cd: 0.0,
                has_dash: true,
                drop_t: 0.0,
                inv: 0.0,
                hearts: MAX_HEARTS,
                anim: 0.0,
                land_t: 0.0,
                ghost_t: 0.0,
            },
            enemies: lvl
                .enemy_spawns
                .iter()
                .map(|&(kind, pos)| Enemy {
                    kind,
                    pos,
                    vel: vec2(0.0, 0.0),
                    dir: -1.0,
                    anchor: pos,
                    t: hash01((pos.x + pos.y) as i32) * 6.0,
                    alive: true,
                    dying: 0.0,
                })
                .collect(),
            wisps: lvl
                .wisp_spawns
                .iter()
                .map(|&pos| Wisp {
                    pos,
                    phase: hash01(pos.x as i32) * 6.0,
                    taken: false,
                })
                .collect(),
            lanterns: lvl
                .lantern_spawns
                .iter()
                .map(|&pos| Lantern { pos, lit: false })
                .collect(),
            particles: Vec::new(),
            leaves: Vec::new(),
            ghosts: Vec::new(),
            fireflies,
            cam: vec2(lvl.player_spawn.x - 80.0, lvl.player_spawn.y - 100.0),
            shake: 0.0,
            time: 0.0,
            leaf_t: 0.0,
            wisps_got: 0,
            checkpoint: lvl.player_spawn,
            dead: false,
            won: false,
            rng: Rng(0xf0f0_1234),
        }
    }

    fn hurt(&mut self, dir: f32) {
        if self.player.inv > 0.0 || self.dead {
            return;
        }
        self.player.hearts -= 1;
        self.player.inv = 1.2;
        self.player.vel = vec2(dir * 150.0, -170.0);
        self.shake = 4.0;
        let c = Color::from_rgba(206, 66, 58, 255);
        let center = self.player.pos + self.player.size * 0.5;
        burst(&mut self.particles, &mut self.rng, center, 10, c, 90.0, 300.0, 0.5);
        if self.player.hearts <= 0 {
            self.dead = true;
            let c2 = Color::from_rgba(232, 138, 60, 255);
            burst(&mut self.particles, &mut self.rng, center, 26, c2, 130.0, 200.0, 0.9);
        }
    }

    fn respawn(&mut self) {
        self.player.pos = self.checkpoint;
        self.player.vel = vec2(0.0, 0.0);
        self.player.inv = 1.5;
        self.player.jumps = 1;
        self.player.has_dash = true;
        self.player.dash_t = 0.0;
    }

    fn update(&mut self, lvl: &Level, dt: f32) {
        self.time += dt;
        let p = &mut self.player;

        let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        let down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
        let jump_pressed = is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::W)
            || is_key_pressed(KeyCode::Z);
        let jump_held = is_key_down(KeyCode::Space)
            || is_key_down(KeyCode::Up)
            || is_key_down(KeyCode::W)
            || is_key_down(KeyCode::Z);
        let dash_pressed = is_key_pressed(KeyCode::LeftShift)
            || is_key_pressed(KeyCode::RightShift)
            || is_key_pressed(KeyCode::X)
            || is_key_pressed(KeyCode::K);

        p.coyote = (p.coyote - dt).max(0.0);
        p.jbuf = (p.jbuf - dt).max(0.0);
        p.inv = (p.inv - dt).max(0.0);
        p.dash_cd = (p.dash_cd - dt).max(0.0);
        p.land_t = (p.land_t - dt).max(0.0);
        p.drop_t = (p.drop_t - dt).max(0.0);
        if jump_pressed {
            p.jbuf = JUMP_BUFFER;
        }

        let dir_in = (right as i32 - left as i32) as f32;
        if dir_in != 0.0 {
            p.face = dir_in;
        }

        if dash_pressed && p.has_dash && p.dash_cd <= 0.0 && p.dash_t <= 0.0 {
            p.dash_t = DASH_TIME;
            p.dash_cd = DASH_COOLDOWN;
            p.has_dash = false;
            p.vel = vec2(p.face * DASH_SPEED, 0.0);
            self.shake = self.shake.max(1.5);
        }

        if p.dash_t > 0.0 {
            p.dash_t -= dt;
            p.vel.x = p.face * DASH_SPEED;
            p.vel.y = 0.0;
            p.ghost_t -= dt;
            if p.ghost_t <= 0.0 {
                p.ghost_t = 0.02;
                self.ghosts.push(Ghost {
                    pos: p.pos,
                    face: p.face,
                    life: 0.25,
                });
            }
        } else {
            let target = dir_in * RUN_SPEED;
            let accel = if p.on_ground { RUN_ACCEL } else { RUN_ACCEL * 0.65 };
            if p.vel.x < target {
                p.vel.x = (p.vel.x + accel * dt).min(target);
            } else if p.vel.x > target {
                p.vel.x = (p.vel.x - accel * dt).max(target);
            }
            p.vel.y = (p.vel.y + GRAV * dt).min(MAX_FALL);
            if !jump_held && p.vel.y < -120.0 {
                p.vel.y = -120.0;
            }
        }

        if p.jbuf > 0.0 {
            if p.on_ground || p.coyote > 0.0 {
                if down && p.on_ground {
                    p.drop_t = 0.18;
                } else {
                    p.vel.y = -JUMP_VEL;
                    p.jumps = 1;
                    p.coyote = 0.0;
                    p.dash_t = 0.0;
                    let feet = p.pos + vec2(p.size.x * 0.5, p.size.y);
                    burst(
                        &mut self.particles,
                        &mut self.rng,
                        feet,
                        5,
                        Color::from_rgba(150, 130, 140, 255),
                        40.0,
                        150.0,
                        0.35,
                    );
                }
                p.jbuf = 0.0;
            } else if p.jumps > 0 && p.dash_t <= 0.0 {
                p.vel.y = -DJUMP_VEL;
                p.jumps -= 1;
                p.jbuf = 0.0;
                let center = p.pos + p.size * 0.5;
                burst(
                    &mut self.particles,
                    &mut self.rng,
                    center,
                    9,
                    Color::from_rgba(150, 236, 224, 255),
                    70.0,
                    30.0,
                    0.4,
                );
            }
        }

        let was_ground = p.on_ground;
        let fall_speed = p.vel.y;
        let drop = p.drop_t > 0.0;
        let (on_ground, _) = move_actor(lvl, &mut p.pos, p.size, &mut p.vel, dt, drop);
        p.on_ground = on_ground;
        if on_ground {
            p.coyote = COYOTE;
            p.jumps = 1;
            p.has_dash = true;
            if !was_ground && fall_speed > 150.0 {
                p.land_t = 0.12;
                let feet = p.pos + vec2(p.size.x * 0.5, p.size.y);
                burst(
                    &mut self.particles,
                    &mut self.rng,
                    feet,
                    7,
                    Color::from_rgba(150, 130, 140, 255),
                    50.0,
                    150.0,
                    0.4,
                );
            }
        }

        p.anim += dt * (1.0 + p.vel.x.abs() / RUN_SPEED * 9.0);

        let ppos = p.pos;
        let psize = p.size;
        let pvel = p.vel;
        let pface = p.face;

        let foot = vec2(ppos.x + 2.0, ppos.y + 4.0);
        let foot_size = vec2(psize.x - 4.0, psize.y - 4.0);
        let mut spiked = false;
        for ty in tile_span(foot.y, foot_size.y) {
            for tx in tile_span(foot.x, foot_size.x) {
                if lvl.tile(tx, ty) == Tile::Spike {
                    let zone = vec2(tx as f32 * TILE + 1.0, ty as f32 * TILE + 9.0);
                    if overlap(foot, foot_size, zone, vec2(14.0, 7.0)) {
                        spiked = true;
                    }
                }
            }
        }
        if spiked {
            self.hurt(-pface);
        }

        let mut stomp: Option<usize> = None;
        let mut touch: Option<f32> = None;
        for i in 0..self.enemies.len() {
            let (kind, alive, dying) = {
                let e = &self.enemies[i];
                (e.kind, e.alive, e.dying)
            };
            if !alive || dying > 0.0 {
                continue;
            }
            match kind {
                EKind::Beetle => {
                    let e = &mut self.enemies[i];
                    let ahead_x = if e.dir > 0.0 { e.pos.x + 13.0 } else { e.pos.x - 1.0 };
                    let atx = (ahead_x / TILE).floor() as i32;
                    let mid_ty = ((e.pos.y + 4.0) / TILE).floor() as i32;
                    let foot_ty = ((e.pos.y + 11.0) / TILE).floor() as i32;
                    if lvl.solid(atx, mid_ty)
                        || !lvl.solid(atx, foot_ty)
                        || lvl.tile(atx, foot_ty - 1) == Tile::Spike
                    {
                        e.dir = -e.dir;
                    }
                    e.vel.x = e.dir * 26.0;
                    e.vel.y = (e.vel.y + GRAV * dt).min(MAX_FALL);
                    let mut pos = e.pos;
                    let mut vel = e.vel;
                    move_actor(lvl, &mut pos, vec2(12.0, 9.0), &mut vel, dt, false);
                    e.pos = pos;
                    e.vel = vel;
                    e.t += dt;
                }
                EKind::Moth => {
                    let e = &mut self.enemies[i];
                    e.t += dt;
                    e.pos = e.anchor + vec2((e.t * 0.8).sin() * 36.0, (e.t * 2.3).sin() * 9.0);
                    e.dir = if (e.t * 0.8).cos() >= 0.0 { 1.0 } else { -1.0 };
                }
            }
            let e = &self.enemies[i];
            let esize = match e.kind {
                EKind::Beetle => vec2(12.0, 9.0),
                EKind::Moth => vec2(12.0, 8.0),
            };
            if overlap(ppos, psize, e.pos, esize) {
                if pvel.y > 40.0 && ppos.y + psize.y - e.pos.y < 9.0 {
                    stomp = Some(i);
                } else {
                    let dir = if ppos.x + psize.x * 0.5 >= e.pos.x + esize.x * 0.5 {
                        1.0
                    } else {
                        -1.0
                    };
                    touch = Some(dir);
                }
                break;
            }
        }
        if let Some(i) = stomp {
            self.enemies[i].dying = 0.3;
            let epos = self.enemies[i].pos;
            self.player.vel.y = -235.0;
            self.player.jumps = 1;
            self.player.has_dash = true;
            self.shake = self.shake.max(2.0);
            burst(
                &mut self.particles,
                &mut self.rng,
                epos + vec2(6.0, 4.0),
                12,
                Color::from_rgba(182, 134, 210, 255),
                80.0,
                250.0,
                0.5,
            );
        } else if let Some(dir) = touch {
            self.hurt(dir);
        }
        for e in &mut self.enemies {
            if e.dying > 0.0 {
                e.dying -= dt;
                if e.dying <= 0.0 {
                    e.alive = false;
                }
            }
        }

        let pcenter = self.player.pos + self.player.size * 0.5;
        for w in &mut self.wisps {
            if w.taken {
                continue;
            }
            let d = pcenter - w.pos;
            let dist = d.length();
            if dist < 26.0 {
                w.pos += d.normalize_or_zero() * 90.0 * dt;
            }
            if dist < 10.0 {
                w.taken = true;
                self.wisps_got += 1;
                burst(
                    &mut self.particles,
                    &mut self.rng,
                    w.pos,
                    10,
                    Color::from_rgba(150, 236, 224, 255),
                    60.0,
                    -30.0,
                    0.5,
                );
            }
        }

        for l in &mut self.lanterns {
            if !l.lit && overlap(self.player.pos, self.player.size, l.pos - vec2(4.0, 2.0), vec2(18.0, 18.0)) {
                l.lit = true;
                self.checkpoint = l.pos - vec2(2.0, 14.0);
                self.player.hearts = MAX_HEARTS;
                burst(
                    &mut self.particles,
                    &mut self.rng,
                    l.pos + vec2(5.0, 6.0),
                    16,
                    Color::from_rgba(252, 214, 130, 255),
                    70.0,
                    -20.0,
                    0.7,
                );
            }
        }

        if overlap(
            self.player.pos,
            self.player.size,
            lvl.gate - vec2(12.0, 44.0),
            vec2(24.0, 44.0),
        ) {
            self.won = true;
        }

        if self.player.pos.y > lvl.h as f32 * TILE + 30.0 {
            self.player.hearts -= 1;
            self.shake = 4.0;
            if self.player.hearts <= 0 {
                self.dead = true;
            } else {
                self.respawn();
            }
        }

        self.update_ambient(lvl, dt);

        let vw = screen_width() / (screen_height() / VIEW_H).floor().max(2.0);
        let vh = VIEW_H.min(screen_height() / (screen_height() / VIEW_H).floor().max(2.0));
        let target = vec2(
            self.player.pos.x + self.player.face * 22.0 - vw * 0.5,
            self.player.pos.y - vh * 0.55,
        );
        let k = (6.0 * dt).min(1.0);
        self.cam += (target - self.cam) * k;
        self.cam.x = self.cam.x.clamp(0.0, (lvl.w as f32 * TILE - vw).max(0.0));
        self.cam.y = self.cam.y.clamp(-20.0, (lvl.h as f32 * TILE - vh).max(0.0));
    }

    fn update_ambient(&mut self, lvl: &Level, dt: f32) {
        self.shake = (self.shake - dt * 14.0).max(0.0);
        for pt in &mut self.particles {
            pt.vel.y += pt.grav * dt;
            pt.pos += pt.vel * dt;
            pt.life -= dt;
        }
        self.particles.retain(|pt| pt.life > 0.0);
        for g in &mut self.ghosts {
            g.life -= dt;
        }
        self.ghosts.retain(|g| g.life > 0.0);

        self.leaf_t -= dt;
        let vw = screen_width() / (screen_height() / VIEW_H).floor().max(2.0);
        if self.leaf_t <= 0.0 {
            self.leaf_t = 0.3;
            let cols = [
                Color::from_rgba(222, 146, 86, 255),
                Color::from_rgba(198, 104, 110, 255),
                Color::from_rgba(240, 196, 120, 255),
            ];
            let x = self.cam.x + self.rng.f() * vw;
            let ci = (self.rng.f() * 3.0) as usize % 3;
            self.leaves.push(Leaf {
                pos: vec2(x, self.cam.y - 8.0),
                t: self.rng.f() * 6.0,
                col: cols[ci],
            });
        }
        for l in &mut self.leaves {
            l.t += dt;
            l.pos.y += 22.0 * dt;
            l.pos.x += (l.t * 2.2).sin() * 16.0 * dt;
        }
        let limit = lvl.h as f32 * TILE + 40.0;
        let cy = self.cam.y;
        self.leaves.retain(|l| l.pos.y < (cy + 220.0).min(limit));
    }
}

fn lerp_col(a: (u8, u8, u8), b: (u8, u8, u8), f: f32) -> Color {
    Color::new(
        (a.0 as f32 + (b.0 as f32 - a.0 as f32) * f) / 255.0,
        (a.1 as f32 + (b.1 as f32 - a.1 as f32) * f) / 255.0,
        (a.2 as f32 + (b.2 as f32 - a.2 as f32) * f) / 255.0,
        1.0,
    )
}

fn ridge(x: f32, amp: f32) -> f32 {
    (((x * 0.013).sin() + (x * 0.031 + 1.7).sin() * 0.6 + (x * 0.007 + 4.2).cos() * 1.3) * amp
        + amp * 2.2)
        .max(2.0)
}

fn draw_bg(cam: Vec2, s: f32, vw: f32, vh: f32, t: f32) {
    let bands = 14;
    for i in 0..bands {
        let f = i as f32 / (bands - 1) as f32;
        let c = lerp_col((24, 20, 52), (188, 108, 96), f * f);
        let y = (i as f32 * vh / bands as f32).floor();
        draw_rectangle(0.0, y * s, vw * s, (vh / bands as f32 + 1.0) * s, c);
    }

    for i in 0..70 {
        let x = (hash01(i * 3 + 1) * (vw + 80.0) - cam.x * 0.08).rem_euclid(vw + 80.0) - 40.0;
        let y = hash01(i * 5 + 2) * vh * 0.5;
        let tw = ((t * 1.3 + i as f32 * 1.7).sin() * 0.5 + 0.5) * 0.6 + 0.2;
        draw_rectangle(
            (x.floor()) * s,
            (y.floor()) * s,
            s,
            s,
            Color::new(0.95, 0.93, 0.85, tw),
        );
    }

    let mx = vw * 0.76 - cam.x * 0.04;
    let my = 32.0;
    draw_circle(mx * s, my * s, 16.0 * s, Color::new(0.95, 0.9, 0.8, 0.08));
    draw_circle(mx * s, my * s, 12.0 * s, Color::new(0.95, 0.9, 0.8, 0.08));
    draw_circle(mx * s, my * s, 9.0 * s, Color::from_rgba(240, 234, 214, 255));
    draw_circle((mx - 3.0) * s, (my - 2.0) * s, 2.0 * s, Color::from_rgba(216, 206, 190, 255));
    draw_circle((mx + 2.5) * s, (my + 3.0) * s, 1.4 * s, Color::from_rgba(216, 206, 190, 255));

    let horizon1 = vh * 0.66;
    let c1 = Color::from_rgba(58, 42, 84, 255);
    let mut sx = 0.0;
    while sx < vw + 3.0 {
        let wx = sx + cam.x * 0.22;
        let h = ridge(wx, 7.0);
        draw_rectangle(sx * s, (horizon1 - h) * s, 3.0 * s, (h + vh - horizon1) * s, c1);
        sx += 3.0;
    }

    let horizon2 = vh * 0.80;
    let c2 = Color::from_rgba(40, 32, 64, 255);
    let mut sx = 0.0;
    while sx < vw + 3.0 {
        let wx = sx + cam.x * 0.45;
        let h = ridge(wx * 1.4 + 900.0, 5.0);
        draw_rectangle(sx * s, (horizon2 - h) * s, 3.0 * s, (h + vh - horizon2) * s, c2);
        sx += 3.0;
    }

    let ct = Color::from_rgba(34, 27, 58, 255);
    let base_j = ((cam.x * 0.45) / 20.0).floor() as i32 - 1;
    let nj = (vw / 20.0) as i32 + 3;
    for j in base_j..base_j + nj {
        if hash01(j * 11 + 5) > 0.72 {
            continue;
        }
        let wx = j as f32 * 20.0 + hash01(j * 7 + 1) * 12.0;
        let tx = wx - cam.x * 0.45;
        let gy = horizon2 - ridge((wx + cam.x * 0.45 * 0.0) * 1.4 + 900.0, 5.0) + 3.0;
        let th = 22.0 + hash01(j * 13 + 2) * 16.0;
        draw_triangle(
            vec2(tx * s, (gy - th) * s),
            vec2((tx - 6.0) * s, (gy - th * 0.4) * s),
            vec2((tx + 6.0) * s, (gy - th * 0.4) * s),
            ct,
        );
        draw_triangle(
            vec2(tx * s, (gy - th * 0.62) * s),
            vec2((tx - 8.0) * s, gy * s),
            vec2((tx + 8.0) * s, gy * s),
            ct,
        );
    }

    draw_rectangle(
        0.0,
        (horizon2 - 4.0) * s,
        vw * s,
        (vh - horizon2 + 4.0) * s,
        Color::new(0.55, 0.35, 0.45, 0.10),
    );
}

struct V {
    cam: Vec2,
    s: f32,
}

impl V {
    fn tex(&self, t: &Texture2D, x: f32, y: f32, flip: bool, tint: Color) {
        draw_texture_ex(
            t,
            (x - self.cam.x).floor() * self.s,
            (y - self.cam.y).floor() * self.s,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(t.width() * self.s, t.height() * self.s)),
                flip_x: flip,
                ..Default::default()
            },
        );
    }

    fn tex_sized(&self, t: &Texture2D, x: f32, y: f32, w: f32, h: f32, flip: bool, tint: Color) {
        draw_texture_ex(
            t,
            (x - self.cam.x).floor() * self.s,
            (y - self.cam.y).floor() * self.s,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(w * self.s, h * self.s)),
                flip_x: flip,
                ..Default::default()
            },
        );
    }

    fn rect(&self, x: f32, y: f32, w: f32, h: f32, c: Color) {
        draw_rectangle(
            (x - self.cam.x).floor() * self.s,
            (y - self.cam.y).floor() * self.s,
            w * self.s,
            h * self.s,
            c,
        );
    }

    fn circle(&self, x: f32, y: f32, r: f32, c: Color) {
        draw_circle(
            (x - self.cam.x) * self.s,
            (y - self.cam.y) * self.s,
            r * self.s,
            c,
        );
    }
}

fn draw_world(g: &Game, lvl: &Level, a: &Assets, s: f32, vw: f32, vh: f32, t: f32, show_player: bool) {
    let shake_off = if g.shake > 0.0 {
        vec2((t * 97.0).sin(), (t * 83.0).cos()) * g.shake
    } else {
        vec2(0.0, 0.0)
    };
    let cam = (g.cam + shake_off).floor();
    let v = V { cam, s };

    draw_bg(cam, s, vw, vh, t);

    let tx0 = ((cam.x / TILE).floor() as i32 - 1).max(0);
    let tx1 = (((cam.x + vw) / TILE).ceil() as i32 + 1).min(lvl.w);
    let ty0 = ((cam.y / TILE).floor() as i32 - 1).max(0);
    let ty1 = (((cam.y + vh) / TILE).ceil() as i32 + 1).min(lvl.h);
    for ty in ty0..ty1 {
        for tx in tx0..tx1 {
            let tile = lvl.tile(tx, ty);
            let x = tx as f32 * TILE;
            let y = ty as f32 * TILE;
            match tile {
                Tile::Ground => {
                    let tex = if lvl.solid(tx, ty - 1) { &a.dirt } else { &a.grass };
                    v.tex(tex, x, y, false, WHITE);
                    if !lvl.solid(tx, ty - 1) {
                        let r = hash01(tx * 13 + ty);
                        if r > 0.8 {
                            let fx = x + 3.0 + r * 9.0;
                            let heads = [
                                Color::from_rgba(234, 148, 186, 255),
                                Color::from_rgba(252, 214, 130, 255),
                                Color::from_rgba(182, 134, 210, 255),
                            ];
                            let hc = heads[(tx as usize * 7 + ty as usize) % 3];
                            v.rect(fx, y - 3.0, 1.0, 3.0, Color::from_rgba(62, 116, 84, 255));
                            v.rect(fx - 0.5, y - 5.0, 2.0, 2.0, hc);
                        }
                    }
                }
                Tile::Stone => v.tex(&a.stone, x, y, false, WHITE),
                Tile::Plank => v.tex(&a.plank, x, y, false, WHITE),
                Tile::Spike => v.tex(&a.spike, x, y, false, WHITE),
                Tile::Empty => {}
            }
        }
    }

    for (base, phase) in &g.fireflies {
        if base.x < cam.x - 20.0 || base.x > cam.x + vw + 20.0 {
            continue;
        }
        let fp = *base + vec2((t * 0.6 + phase).sin() * 9.0, (t * 0.9 + phase * 2.0).cos() * 6.0);
        let pulse = ((t * 2.0 + phase * 3.0).sin() * 0.5 + 0.5) * 0.7 + 0.15;
        v.circle(fp.x, fp.y, 2.6, Color::new(0.98, 0.85, 0.4, 0.10 * pulse * 3.0));
        v.rect(fp.x, fp.y, 1.0, 1.0, Color::new(0.99, 0.9, 0.55, pulse));
    }

    for l in &g.lanterns {
        if l.lit {
            let pulse = (t * 3.0).sin() * 0.02;
            v.circle(l.pos.x + 5.0, l.pos.y + 6.0, 22.0, Color::new(0.99, 0.84, 0.5, 0.08 + pulse));
            v.circle(l.pos.x + 5.0, l.pos.y + 6.0, 11.0, Color::new(0.99, 0.84, 0.5, 0.10 + pulse));
            v.tex(&a.lantern, l.pos.x, l.pos.y, false, WHITE);
        } else {
            v.tex(&a.lantern, l.pos.x, l.pos.y, false, Color::new(0.4, 0.4, 0.5, 1.0));
        }
    }

    let gx = lvl.gate.x;
    let gy = lvl.gate.y;
    let pulse = (t * 2.2).sin() * 0.03;
    v.circle(gx, gy - 22.0, 30.0, Color::new(0.95, 0.55, 0.45, 0.08 + pulse));
    let red = Color::from_rgba(206, 66, 58, 255);
    let dark = Color::from_rgba(140, 36, 44, 255);
    let ink = Color::from_rgba(38, 28, 44, 255);
    v.rect(gx - 17.0, gy - 38.0, 5.0, 38.0, red);
    v.rect(gx + 12.0, gy - 38.0, 5.0, 38.0, red);
    v.rect(gx - 13.0, gy - 38.0, 1.0, 38.0, dark);
    v.rect(gx + 16.0, gy - 38.0, 1.0, 38.0, dark);
    v.rect(gx - 20.0, gy - 31.0, 40.0, 4.0, red);
    v.rect(gx - 20.0, gy - 27.0, 40.0, 1.0, dark);
    v.rect(gx - 26.0, gy - 44.0, 52.0, 3.0, ink);
    v.rect(gx - 24.0, gy - 41.0, 48.0, 4.0, red);
    v.rect(gx - 24.0, gy - 37.0, 48.0, 1.0, dark);
    v.rect(gx - 3.0, gy - 37.0, 6.0, 7.0, ink);
    v.rect(gx - 2.0, gy - 36.0, 4.0, 5.0, Color::from_rgba(252, 214, 130, 255));

    for w in &g.wisps {
        if w.taken {
            continue;
        }
        let bob = (t * 2.4 + w.phase).sin() * 2.0;
        let frame = ((t * 6.0 + w.phase) as usize) % 2;
        v.circle(w.pos.x, w.pos.y + bob, 7.0, Color::new(0.5, 0.95, 0.9, 0.12));
        v.tex(&a.wisp[frame], w.pos.x - 4.0, w.pos.y + bob - 4.0, false, WHITE);
    }

    for e in &g.enemies {
        if !e.alive {
            continue;
        }
        match e.kind {
            EKind::Beetle => {
                let frame = ((e.t * 8.0) as usize) % 2;
                if e.dying > 0.0 {
                    v.tex_sized(&a.beetle[0], e.pos.x, e.pos.y + 5.0, 12.0, 4.0, e.dir > 0.0, WHITE);
                } else {
                    v.tex(&a.beetle[frame], e.pos.x, e.pos.y, e.dir > 0.0, WHITE);
                }
            }
            EKind::Moth => {
                let frame = ((e.t * 9.0) as usize) % 2;
                if e.dying > 0.0 {
                    v.tex_sized(&a.moth[1], e.pos.x, e.pos.y + 4.0, 12.0, 4.0, e.dir > 0.0, WHITE);
                } else {
                    v.tex(&a.moth[frame], e.pos.x, e.pos.y - 1.0, e.dir > 0.0, WHITE);
                }
            }
        }
    }

    for l in &g.leaves {
        v.rect(l.pos.x, l.pos.y, 2.0, 2.0, l.col);
    }

    for gh in &g.ghosts {
        let alpha = (gh.life * 3.0).min(0.5);
        v.tex(
            &a.fox_jump,
            gh.pos.x - 3.0,
            gh.pos.y,
            gh.face < 0.0,
            Color::new(0.5, 0.95, 0.9, alpha),
        );
    }

    if show_player && !g.dead {
        let p = &g.player;
        let blink = p.inv > 0.0 && (p.inv * 10.0).fract() < 0.4;
        if !blink {
            let tex = if p.dash_t > 0.0 {
                &a.fox_jump
            } else if !p.on_ground {
                if p.vel.y < 0.0 {
                    &a.fox_jump
                } else {
                    &a.fox_fall
                }
            } else if p.vel.x.abs() > 12.0 {
                &a.fox_run[(p.anim as usize) % 2]
            } else {
                &a.fox_idle
            };
            let (w, h, dy) = if p.land_t > 0.0 {
                (19.0, 9.5, 2.5)
            } else if !p.on_ground && p.vel.y < -60.0 {
                (14.0, 13.5, -1.5)
            } else {
                (16.0, 12.0, 0.0)
            };
            v.tex_sized(tex, p.pos.x - 3.0, p.pos.y + dy, w, h, p.face < 0.0, WHITE);
        }
    }

    for pt in &g.particles {
        let alpha = (pt.life / pt.max).clamp(0.0, 1.0);
        let mut c = pt.col;
        c.a = alpha;
        v.rect(pt.pos.x, pt.pos.y, pt.size, pt.size, c);
    }

    draw_rectangle(0.0, 0.0, vw * s, vh * 0.07 * s, Color::new(0.0, 0.0, 0.05, 0.12));
    draw_rectangle(
        0.0,
        (vh - vh * 0.07) * s,
        vw * s,
        vh * 0.07 * s,
        Color::new(0.0, 0.0, 0.05, 0.12),
    );
}

fn text_c(txt: &str, cx: f32, y: f32, size: f32, col: Color) {
    let d = measure_text(txt, None, size as u16, 1.0);
    draw_text(txt, cx - d.width / 2.0, y, size, Color::new(0.05, 0.02, 0.1, 0.7));
    draw_text(
        txt,
        cx - d.width / 2.0 - size * 0.04,
        y - size * 0.04,
        size,
        col,
    );
}

fn fmt_time(t: f32) -> String {
    format!("{}:{:04.1}", (t / 60.0) as u32, t % 60.0)
}

fn draw_hud(g: &Game, lvl: &Level, a: &Assets, s: f32) {
    let hs = s;
    for i in 0..MAX_HEARTS {
        let tint = if i < g.player.hearts {
            WHITE
        } else {
            Color::new(0.3, 0.3, 0.38, 1.0)
        };
        draw_texture_ex(
            &a.heart,
            (10.0 + i as f32 * 10.0) * hs,
            8.0 * hs,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(8.0 * hs, 7.0 * hs)),
                ..Default::default()
            },
        );
    }
    draw_texture_ex(
        &a.wisp[0],
        10.0 * hs,
        18.0 * hs,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(8.0 * hs, 8.0 * hs)),
            ..Default::default()
        },
    );
    draw_text(
        &format!("{}/{}", g.wisps_got, lvl.wisp_total),
        20.0 * hs,
        25.0 * hs,
        7.0 * hs,
        Color::from_rgba(210, 240, 235, 255),
    );
    let tt = fmt_time(g.time);
    let d = measure_text(&tt, None, (7.0 * hs) as u16, 1.0);
    draw_text(
        &tt,
        screen_width() - d.width - 10.0 * hs,
        14.0 * hs,
        7.0 * hs,
        Color::from_rgba(210, 200, 220, 255),
    );
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Menu,
    Play,
    Dead,
    Win,
}

fn conf() -> Conf {
    Conf {
        window_title: "Foxfire".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let lvl = Level::parse();
    let assets = Assets::new();
    let mut game = Game::new(&lvl);
    let mut mode = Mode::Menu;
    let mut t = 0.0f32;
    let mut paused = false;

    loop {
        let dt = get_frame_time().min(1.0 / 30.0);
        t += dt;
        let s = (screen_height() / VIEW_H).floor().max(2.0);
        let vw = screen_width() / s;
        let vh = screen_height() / s;

        match mode {
            Mode::Menu => {
                let menu_cam = vec2(
                    (t * 14.0).rem_euclid((lvl.w as f32 * TILE - vw).max(1.0)),
                    120.0,
                );
                draw_bg(menu_cam, s, vw, vh, t);
                let cx = screen_width() / 2.0;
                let fox_y = vh * 0.30 * s;
                let bob = (t * 2.0).sin() * 3.0 * s;
                draw_texture_ex(
                    &assets.fox_run[((t * 7.0) as usize) % 2],
                    cx - 8.0 * s * 3.0,
                    fox_y + bob,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(16.0 * s * 6.0, 12.0 * s * 6.0)),
                        ..Default::default()
                    },
                );
                for i in 0..3 {
                    let ang = t * 1.4 + i as f32 * 2.1;
                    let wx = cx + ang.cos() * 62.0 * s;
                    let wy = fox_y + 6.0 * s * 6.0 * 0.5 + ang.sin() * 24.0 * s + bob;
                    draw_circle(wx, wy, 3.0 * s, Color::new(0.5, 0.95, 0.9, 0.15));
                    draw_texture_ex(
                        &assets.wisp[(i + (t * 5.0) as usize) % 2],
                        wx - 4.0 * s,
                        wy - 4.0 * s,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(8.0 * s, 8.0 * s)),
                            ..Default::default()
                        },
                    );
                }
                text_c("F O X F I R E", cx, vh * 0.62 * s, 16.0 * s, Color::from_rgba(252, 214, 130, 255));
                text_c(
                    "carry the last flame through the dusk woods",
                    cx,
                    vh * 0.70 * s,
                    6.0 * s,
                    Color::from_rgba(220, 190, 210, 255),
                );
                text_c(
                    "ARROWS / WASD move    SPACE jump + double jump    SHIFT dash",
                    cx,
                    vh * 0.82 * s,
                    5.5 * s,
                    Color::from_rgba(190, 170, 200, 255),
                );
                text_c(
                    "stomp foes    light the lanterns    reach the gate",
                    cx,
                    vh * 0.88 * s,
                    5.5 * s,
                    Color::from_rgba(160, 145, 175, 255),
                );
                if (t * 1.6).sin() > -0.3 {
                    text_c("press ENTER", cx, vh * 0.96 * s, 6.5 * s, WHITE);
                }
                if is_key_pressed(KeyCode::Enter) {
                    game = Game::new(&lvl);
                    mode = Mode::Play;
                }
            }
            Mode::Play => {
                if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                    paused = !paused;
                }
                if !paused {
                    game.update(&lvl, dt);
                }
                draw_world(&game, &lvl, &assets, s, vw, vh, t, true);
                draw_hud(&game, &lvl, &assets, s);
                if paused {
                    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.05, 0.5));
                    text_c("PAUSED", screen_width() / 2.0, screen_height() * 0.5, 12.0 * s, WHITE);
                }
                if game.dead {
                    mode = Mode::Dead;
                }
                if game.won {
                    mode = Mode::Win;
                }
            }
            Mode::Dead => {
                game.update_ambient(&lvl, dt);
                draw_world(&game, &lvl, &assets, s, vw, vh, t, false);
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.1, 0.0, 0.05, 0.55));
                let cx = screen_width() / 2.0;
                text_c("the flame goes out...", cx, screen_height() * 0.42, 12.0 * s, Color::from_rgba(230, 120, 110, 255));
                text_c(
                    &format!("wisps gathered  {}/{}", game.wisps_got, lvl.wisp_total),
                    cx,
                    screen_height() * 0.54,
                    6.0 * s,
                    Color::from_rgba(200, 190, 210, 255),
                );
                text_c("ENTER to rekindle", cx, screen_height() * 0.66, 6.5 * s, WHITE);
                if is_key_pressed(KeyCode::Enter) {
                    game = Game::new(&lvl);
                    mode = Mode::Play;
                }
            }
            Mode::Win => {
                game.update_ambient(&lvl, dt);
                draw_world(&game, &lvl, &assets, s, vw, vh, t, true);
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.02, 0.08, 0.5));
                let cx = screen_width() / 2.0;
                text_c("the shrine glows again", cx, screen_height() * 0.38, 12.0 * s, Color::from_rgba(252, 214, 130, 255));
                text_c(
                    &format!("wisps  {}/{}", game.wisps_got, lvl.wisp_total),
                    cx,
                    screen_height() * 0.50,
                    7.0 * s,
                    Color::from_rgba(180, 240, 230, 255),
                );
                text_c(
                    &format!("time  {}", fmt_time(game.time)),
                    cx,
                    screen_height() * 0.58,
                    7.0 * s,
                    Color::from_rgba(210, 200, 220, 255),
                );
                if game.wisps_got == lvl.wisp_total {
                    text_c("every last flame carried home", cx, screen_height() * 0.68, 6.0 * s, Color::from_rgba(234, 148, 186, 255));
                }
                text_c("ENTER to run again", cx, screen_height() * 0.80, 6.5 * s, WHITE);
                if is_key_pressed(KeyCode::Enter) {
                    game = Game::new(&lvl);
                    mode = Mode::Play;
                }
            }
        }

        next_frame().await;
    }
}
