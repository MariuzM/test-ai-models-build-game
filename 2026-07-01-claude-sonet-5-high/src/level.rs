use macroquad::prelude::*;

pub const TILE: f32 = 32.0;
pub const COLS: usize = 100;
pub const ROWS: usize = 14;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Empty,
    Ground,
}

pub struct SlimeSpawn {
    pub x: f32,
    pub y: f32,
    pub patrol_min: f32,
    pub patrol_max: f32,
}

pub struct BatSpawn {
    pub x: f32,
    pub y: f32,
    pub range_min: f32,
    pub range_max: f32,
}

pub struct Level {
    tiles: Vec<TileKind>,
    pub spikes: Vec<(usize, usize)>,
    pub gems: Vec<Vec2>,
    pub bats: Vec<BatSpawn>,
    pub slimes: Vec<SlimeSpawn>,
    pub torches: Vec<(f32, f32)>,
    pub player_start: Vec2,
    pub portal: Vec2,
}

impl Level {
    fn new() -> Self {
        Self {
            tiles: vec![TileKind::Empty; COLS * ROWS],
            spikes: Vec::new(),
            gems: Vec::new(),
            bats: Vec::new(),
            slimes: Vec::new(),
            torches: Vec::new(),
            player_start: vec2(0.0, 0.0),
            portal: vec2(0.0, 0.0),
        }
    }

    fn idx(col: i32, row: i32) -> Option<usize> {
        if col < 0 || row < 0 || col as usize >= COLS || row as usize >= ROWS {
            None
        } else {
            Some(row as usize * COLS + col as usize)
        }
    }

    pub fn is_solid(&self, col: i32, row: i32) -> bool {
        match Self::idx(col, row) {
            Some(i) => self.tiles[i] == TileKind::Ground,
            None => false,
        }
    }

    pub fn tile_at(&self, col: usize, row: usize) -> TileKind {
        self.tiles[row * COLS + col]
    }

    pub fn width_px(&self) -> f32 {
        COLS as f32 * TILE
    }

    pub fn height_px(&self) -> f32 {
        ROWS as f32 * TILE
    }

    fn ground(&mut self, x0: usize, x1: usize, row: usize) {
        for x in x0..=x1 {
            self.tiles[row * COLS + x] = TileKind::Ground;
        }
    }

    fn spike(&mut self, x: usize, row: usize) {
        self.spikes.push((x, row));
    }

    fn gem(&mut self, x: usize, row: usize) {
        self.gems
            .push(vec2(x as f32 * TILE + TILE / 2.0, row as f32 * TILE + TILE / 2.0));
    }

    fn bat(&mut self, x0: usize, x1: usize, row: usize) {
        self.bats.push(BatSpawn {
            x: (x0 as f32 + x1 as f32) / 2.0 * TILE + TILE / 2.0,
            y: row as f32 * TILE + TILE / 2.0,
            range_min: x0 as f32 * TILE,
            range_max: x1 as f32 * TILE,
        });
    }

    fn slime(&mut self, x0: usize, x1: usize, row: usize) {
        self.slimes.push(SlimeSpawn {
            x: x0 as f32 * TILE + TILE / 2.0,
            y: row as f32 * TILE,
            patrol_min: x0 as f32 * TILE,
            patrol_max: x1 as f32 * TILE,
        });
    }

    fn torch(&mut self, x: usize, row: usize) {
        self.torches.push((x as f32 * TILE, row as f32 * TILE));
    }
}

pub fn build_level() -> Level {
    let mut lvl = Level::new();
    let floor = ROWS - 1; // row 13

    // Opening stretch
    lvl.ground(0, 9, floor);
    lvl.player_start = vec2(2.0 * TILE, (floor as f32 - 2.0) * TILE);
    lvl.torch(4, floor - 4);
    lvl.gem(6, floor - 1);
    lvl.gem(7, floor - 1);

    // Gap 1: x10-11 is a pit
    lvl.ground(12, 24, floor);
    lvl.slime(16, 20, floor);
    lvl.spike(22, floor - 1);
    lvl.spike(23, floor - 1);
    lvl.gem(13, floor - 2);
    lvl.gem(14, floor - 3);
    lvl.gem(15, floor - 2);

    // Gap 2: x25-26 pit
    lvl.ground(27, 39, floor);
    lvl.ground(30, 34, floor - 4); // elevated detour platform with gems
    lvl.gem(31, floor - 5);
    lvl.gem(32, floor - 5);
    lvl.gem(33, floor - 5);
    lvl.bat(30, 38, floor - 7);
    lvl.spike(36, floor - 1);
    lvl.spike(37, floor - 1);
    lvl.torch(28, floor - 4);

    // Big chasm x40-55 crossed via staircase platforms
    lvl.ground(41, 43, floor - 3);
    lvl.ground(45, 47, floor - 5);
    lvl.ground(49, 51, floor - 7);
    lvl.gem(50, floor - 8);
    lvl.bat(47, 53, floor - 9);
    lvl.ground(53, 55, floor - 5);
    lvl.ground(57, 59, floor - 3);

    // Main stretch
    lvl.ground(60, 74, floor);
    lvl.slime(63, 70, floor);
    lvl.spike(72, floor - 1);
    lvl.spike(73, floor - 1);
    lvl.gem(61, floor - 2);
    lvl.gem(65, floor - 2);
    lvl.gem(68, floor - 2);
    lvl.torch(62, floor - 4);

    // Gap 3: x75-77 pit
    lvl.ground(78, 99, floor);
    lvl.bat(80, 86, floor - 6);
    lvl.bat(87, 92, floor - 6);
    lvl.spike(92, floor - 1);
    lvl.spike(93, floor - 1);
    lvl.spike(94, floor - 1);
    lvl.gem(80, floor - 4);
    lvl.gem(84, floor - 4);
    lvl.gem(88, floor - 4);

    lvl.torch(95, floor - 4);
    lvl.torch(99, floor - 4);
    lvl.portal = vec2(97.0 * TILE + TILE / 2.0, (floor as f32 - 1.0) * TILE);

    lvl
}
