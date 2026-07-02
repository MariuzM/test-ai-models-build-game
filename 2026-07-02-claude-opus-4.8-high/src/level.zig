pub const ROWS: usize = 12;
pub const MAX_COLS: usize = 256;
pub const TILE: f32 = 16;

pub const Level = struct {
    tiles: [ROWS][MAX_COLS]u8,
    cols: usize,

    pub fn get(self: *const Level, col: i32, row: i32) u8 {
        if (col < 0 or row < 0 or col >= @as(i32, @intCast(self.cols)) or row >= @as(i32, @intCast(ROWS)))
            return '.';
        return self.tiles[@intCast(row)][@intCast(col)];
    }

    pub fn solid(self: *const Level, col: i32, row: i32) bool {
        return self.get(col, row) == '#';
    }

    pub fn widthPx(self: *const Level) f32 {
        return @as(f32, @floatFromInt(self.cols)) * TILE;
    }
};

fn isPit(col: usize) bool {
    return (col >= 30 and col <= 32) or
        (col >= 62 and col <= 64) or
        (col >= 121 and col <= 123) or
        (col >= 168 and col <= 170);
}

fn groundTop(col: usize) usize {
    if (col >= 84 and col <= 85) return 8;
    if (col >= 86 and col <= 87) return 7;
    if (col >= 88 and col <= 103) return 6;
    if (col >= 104 and col <= 105) return 7;
    if (col >= 106 and col <= 107) return 8;
    return 9;
}

fn platform(lvl: *Level, x0: usize, x1: usize, row: usize) void {
    var x = x0;
    while (x <= x1) : (x += 1) lvl.tiles[row][x] = '#';
}

pub fn build() Level {
    var lvl: Level = .{ .tiles = undefined, .cols = 212 };

    for (0..ROWS) |r| {
        for (0..MAX_COLS) |cc| lvl.tiles[r][cc] = '.';
    }

    for (0..lvl.cols) |col| {
        if (isPit(col)) continue;
        var r = groundTop(col);
        while (r < ROWS) : (r += 1) lvl.tiles[r][col] = '#';
    }

    platform(&lvl, 46, 51, 6);
    platform(&lvl, 55, 59, 4);
    platform(&lvl, 110, 115, 7);
    platform(&lvl, 118, 122, 5);
    platform(&lvl, 130, 135, 6);
    platform(&lvl, 140, 143, 4);
    platform(&lvl, 178, 183, 6);
    platform(&lvl, 186, 190, 4);

    lvl.tiles[7][3] = 'P';

    const gems = [_][2]usize{
        .{ 8, 8 },   .{ 12, 8 },  .{ 16, 8 },
        .{ 31, 5 },  .{ 48, 5 },  .{ 50, 5 },
        .{ 57, 3 },  .{ 63, 6 },  .{ 76, 8 },
        .{ 80, 7 },  .{ 90, 5 },  .{ 95, 5 },
        .{ 100, 5 }, .{ 112, 6 }, .{ 120, 4 },
        .{ 122, 8 }, .{ 132, 5 }, .{ 141, 3 },
        .{ 150, 8 }, .{ 156, 8 }, .{ 169, 6 },
        .{ 180, 5 }, .{ 188, 3 }, .{ 198, 8 },
        .{ 203, 8 },
    };
    for (gems) |g| lvl.tiles[g[1]][g[0]] = 'o';

    const spikes = [_]usize{ 40, 41, 42, 148, 149, 160, 161 };
    for (spikes) |sx| lvl.tiles[groundTop(sx) - 1][sx] = '^';

    const beetles = [_]usize{ 22, 58, 78, 96, 133, 155, 182, 200 };
    for (beetles) |bx| lvl.tiles[groundTop(bx) - 1][bx] = 'b';

    lvl.tiles[groundTop(206) - 1][206] = 'F';

    return lvl;
}
